// src/services/video_compression.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Uint8Array;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = compressVideo, catch)]
    fn compress_video_js(bytes: Uint8Array, on_progress: &Closure<dyn FnMut(f64)>) -> Result<js_sys::Promise, JsValue>;
}

pub async fn compress_video(
    bytes: Vec<u8>,
    mut on_progress: impl FnMut(f64) + 'static,
) -> Result<Vec<u8>, String> {
    let array = Uint8Array::from(bytes.as_slice());
    let closure = Closure::wrap(Box::new(move |pct: f64| on_progress(pct)) as Box<dyn FnMut(f64)>);

    let promise = compress_video_js(array, &closure).map_err(|e| format!("{:?}", e))?;
    let result = JsFuture::from(promise).await.map_err(|e| format!("{:?}", e))?;
    closure.forget();

    let output_array = Uint8Array::new(&result);
    Ok(output_array.to_vec())
}