#[derive(Debug, Clone, PartialEq)]
pub enum UploadStatus {
    Idle,
    Ready,
    Uploading,
    Completed,
    Error,
}


#[derive(Clone, PartialEq)]
pub struct PhotoFile {
    pub name: String,
    pub size: usize,
    pub status: UploadStatus,
    pub bytes: Option<Vec<u8>>, 
}

impl PhotoFile {
    pub fn new(name: String, size: usize) -> Self {
        Self {
            name,
            size,
            status: UploadStatus::Idle, // Statut de départ : en attente
            bytes: None,
        }
    }

    pub fn is_video(filename: &str) -> bool {
        let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
        matches!(ext.as_str(), "mp4" | "mov" | "webm" | "avi" | "mkv" | "m4v")
    }
}