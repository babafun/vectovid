use wasm_bindgen::prelude::*;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use std::io::Write;
use zip::write::FileOptions;
use std::io::Cursor;

// Public version string for all consumers
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[wasm_bindgen]
pub fn hello() -> String {
    "vectovid core (wasm) initialized".into()
}

// Minimal exported flags for web shims to detect wasm capability.
#[wasm_bindgen]
pub fn has_packer() -> bool { true }

#[wasm_bindgen]
pub fn has_player() -> bool { true }

#[derive(Serialize)]
struct Meta {
    version: &'static str,
    fps: u32,
    frameCount: u32,
    width: u32,
    height: u32,
    hasAudio: bool,
    audioFile: Option<String>,
}

/// Create a meta object for the VVF archive. Returns a JS object.
#[wasm_bindgen(js_name = create_meta)]
pub fn create_meta(fps: u32, frame_count: u32, width: u32, height: u32, has_audio: bool, audio_file: Option<String>) -> JsValue {
    let meta = Meta {
        version: "1.0",
        fps,
        frameCount: frame_count,
        width,
        height,
        hasAudio: has_audio,
        audioFile: audio_file,
    };

    to_value(&meta).unwrap()
}

/// Return frame interval in milliseconds for a given fps.
#[wasm_bindgen(js_name = frame_interval_ms)]
pub fn frame_interval_ms(fps: u32) -> f64 {
    if fps == 0 { return 0.0; }
    1000.0 / (fps as f64)
}

/// Pack VVF archive from frames (array of SVG strings) and optional audio bytes.
/// Returns a byte vector containing the ZIP archive (.vvf).
#[wasm_bindgen(js_name = pack_vvf)]
pub fn pack_vvf(fps: u32, frames: JsValue, audio: Option<Vec<u8>>) -> Result<Vec<u8>, JsValue> {
    // Deserialize frames from JsValue
    let frames_vec: Vec<String> = from_value(frames).map_err(|e| JsValue::from_str(&format!("frames deserialize error: {}", e)))?;

    let mut cursor = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(&mut cursor);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // write frames folder
    for (i, svg) in frames_vec.iter().enumerate() {
        let name = format!("frames/{:03}.svg", i);
        zip.start_file(name, options).map_err(|e| JsValue::from_str(&format!("zip error: {}", e)))?;
        zip.write_all(svg.as_bytes()).map_err(|e| JsValue::from_str(&format!("zip write error: {}", e)))?;
    }

    // audio if provided
    if let Some(ref audio_bytes) = audio {
        zip.start_file("audio.mp3", options).map_err(|e| JsValue::from_str(&format!("zip error: {}", e)))?;
        zip.write_all(audio_bytes.as_slice()).map_err(|e| JsValue::from_str(&format!("zip write error: {}", e)))?;
    }

    // meta
    let meta = Meta {
        version: "1.0",
        fps,
        frameCount: frames_vec.len() as u32,
        width: 400,
        height: 300,
        hasAudio: audio.is_some(),
        audioFile: if audio.is_some() { Some("audio.mp3".to_string()) } else { None },
    };
    let meta_json = serde_json::to_vec_pretty(&meta).map_err(|e| JsValue::from_str(&format!("meta serialize error: {}", e)))?;
    zip.start_file("meta.json", options).map_err(|e| JsValue::from_str(&format!("zip error: {}", e)))?;
    zip.write_all(&meta_json).map_err(|e| JsValue::from_str(&format!("zip write error: {}", e)))?;

    zip.finish().map_err(|e| JsValue::from_str(&format!("zip finish error: {}", e)))?;
    drop(zip);

    Ok(cursor.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_interval_ms() {
        assert_eq!(frame_interval_ms(0), 0.0);
        assert_eq!(frame_interval_ms(12), 1000.0 / 12.0);
        assert_eq!(frame_interval_ms(60), 1000.0 / 60.0);
    }

    #[test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty());
    }
}
