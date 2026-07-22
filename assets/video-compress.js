window.isWebCodecsSupported = function() {
    return typeof VideoEncoder !== 'undefined' && typeof VideoDecoder !== 'undefined';
};
window.compressVideo = async function(inputBytes, onProgress) {
    return new Promise((resolve, reject) => {
        const mp4boxFile = MP4Box.createFile();
        let videoTrack = null;
        let muxer = null;
        let videoEncoder = null;
        let videoDecoder = null;
        let frameCount = 0;
        let totalSamples = 0;
        let processedSamples = 0;

        mp4boxFile.onReady = (info) => {
            videoTrack = info.videoTracks[0];
            if (!videoTrack) {
                reject("Aucune piste vidéo trouvée");
                return;
            }

            const targetWidth = Math.min(1280, videoTrack.video.width);
            const scale = targetWidth / videoTrack.video.width;
            const targetHeight = Math.round(videoTrack.video.height * scale / 2) * 2;

            muxer = new Mp4Muxer.Muxer({
                target: new Mp4Muxer.ArrayBufferTarget(),
                video: {
                    codec: 'avc',
                    width: targetWidth,
                    height: targetHeight,
                },
                fastStart: 'in-memory',
            });

            videoEncoder = new VideoEncoder({
                output: (chunk, meta) => muxer.addVideoChunk(chunk, meta),
                error: (e) => reject("Erreur encodeur : " + e.message),
            });
            videoEncoder.configure({
                codec: 'avc1.42001f',
                width: targetWidth,
                height: targetHeight,
                bitrate: 2_500_000, // 2.5 Mbps, ajustable
                framerate: videoTrack.video.timescale ? undefined : 30,
            });

            videoDecoder = new VideoDecoder({
                output: (frame) => {
                    const resized = new VideoFrame(frame, {
                        displayWidth: targetWidth,
                        displayHeight: targetHeight,
                    });
                    videoEncoder.encode(resized);
                    resized.close();
                    frame.close();
                    processedSamples++;
                    if (onProgress && totalSamples > 0) {
                        onProgress((processedSamples / totalSamples) * 100);
                    }
                },
                error: (e) => reject("Erreur décodeur : " + e.message),
            });

            const desc = videoTrack.codec.startsWith('avc1') 
                ? mp4boxFile.getTrackById(videoTrack.id).mdia.minf.stbl.stsd.entries[0].avcC 
                : null;

            videoDecoder.configure({
                codec: videoTrack.codec,
                codedWidth: videoTrack.video.width,
                codedHeight: videoTrack.video.height,
                description: desc ? getAvcCDescription(mp4boxFile, videoTrack.id) : undefined,
            });

            mp4boxFile.setExtractionOptions(videoTrack.id, null, { nbSamples: 1000 });
            mp4boxFile.start();
        };

        mp4boxFile.onSamples = (id, user, samples) => {
            totalSamples += samples.length;
            for (const sample of samples) {
                const chunk = new EncodedVideoChunk({
                    type: sample.is_sync ? 'key' : 'delta',
                    timestamp: (sample.cts * 1_000_000) / sample.timescale,
                    duration: (sample.duration * 1_000_000) / sample.timescale,
                    data: sample.data,
                });
                videoDecoder.decode(chunk);
            }
        };

        mp4boxFile.flush = async () => {
            await videoDecoder.flush();
            await videoEncoder.flush();
            muxer.finalize();
            const { buffer } = muxer.target;
            resolve(new Uint8Array(buffer));
        };

        function getAvcCDescription(file, trackId) {
            const track = file.getTrackById(trackId);
            const avcC = track.mdia.minf.stbl.stsd.entries[0].avcC;
            const stream = new MP4Box.DataStream(undefined, 0, MP4Box.DataStream.BIG_ENDIAN);
            avcC.write(stream);
            return new Uint8Array(stream.buffer, 8);
        }

        inputBytes.buffer.fileStart = 0;
        mp4boxFile.appendBuffer(inputBytes.buffer);
        mp4boxFile.flush();
    });
};
console.log("✅ video-compress.js chargé");
console.log("📦 MP4Box disponible :", typeof MP4Box !== 'undefined');
console.log("📦 Mp4Muxer disponible :", typeof Mp4Muxer !== 'undefined');
console.log("🎬 WebCodecs supporté :", window.isWebCodecsSupported());