use wasm_bindgen::prelude::*;
use serde::Serialize;
use serde_wasm_bindgen::to_value;

#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;

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
    #[serde(rename = "frameCount")]
    frame_count: u32,
    width: u32,
    height: u32,
    #[serde(rename = "hasAudio")]
    has_audio: bool,
    #[serde(rename = "audioFile")]
    audio_file: Option<String>,
}

/// Create a meta object for the VVF archive. Returns a JS object.
#[wasm_bindgen(js_name = create_meta)]
pub fn create_meta(fps: u32, frame_count: u32, width: u32, height: u32, has_audio: bool, audio_file: Option<String>) -> JsValue {
    let meta = Meta {
        version: "1.0",
        fps,
        frame_count,
        width,
        height,
        has_audio,
        audio_file,
    };

    to_value(&meta).unwrap()
}

/// Return frame interval in milliseconds for a given fps.
#[wasm_bindgen(js_name = frame_interval_ms)]
pub fn frame_interval_ms(fps: u32) -> f64 {
    if fps == 0 { return 0.0; }
    1000.0 / (fps as f64)
}

// Desktop/native packing (requires zip crate, not available in WASM)
#[cfg(not(target_arch = "wasm32"))]
pub fn pack_vvf_native(fps: u32, frames: Vec<String>, audio: Option<Vec<u8>>) -> Result<Vec<u8>, String> {
    use zip::write::FileOptions;
    use std::io::Cursor;

    let mut cursor = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(&mut cursor);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // write frames folder
    for (i, svg) in frames.iter().enumerate() {
        let name = format!("frames/{:03}.svg", i);
        zip.start_file(name, options).map_err(|e| format!("zip error: {}", e))?;
        zip.write_all(svg.as_bytes()).map_err(|e| format!("zip write error: {}", e))?;
    }

    // audio if provided
    if let Some(ref audio_bytes) = audio {
        zip.start_file("audio.mp3", options).map_err(|e| format!("zip error: {}", e))?;
        zip.write_all(audio_bytes).map_err(|e| format!("zip write error: {}", e))?;
    }

    // meta
    let meta = Meta {
        version: "1.0",
        fps,
        frame_count: frames.len() as u32,
        width: 400,
        height: 300,
        has_audio: audio.is_some(),
        audio_file: if audio.is_some() { Some("audio.mp3".to_string()) } else { None },
    };
    let meta_json = serde_json::to_vec_pretty(&meta).map_err(|e| format!("meta serialize error: {}", e))?;
    zip.start_file("meta.json", options).map_err(|e| format!("zip error: {}", e))?;
    zip.write_all(&meta_json).map_err(|e| format!("zip write error: {}", e))?;

    zip.finish().map_err(|e| format!("zip finish error: {}", e))?;
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

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_pack_vvf_native_simple() {
        let frames = vec![
            "<svg><text>Frame 0</text></svg>".to_string(),
            "<svg><text>Frame 1</text></svg>".to_string(),
        ];
        let result = pack_vvf_native(12, frames, None);
        assert!(result.is_ok());
        
        let bytes = result.unwrap();
        assert!(bytes.len() > 0);
        // Check ZIP signature (PK\x03\x04)
        assert_eq!(&bytes[0..2], b"PK");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_pack_vvf_native_with_audio() {
        let frames = vec!["<svg></svg>".to_string()];
        let audio = vec![0xFF, 0xFB, 0x00, 0x00]; // Fake MP3 header
        let result = pack_vvf_native(24, frames, Some(audio));
        assert!(result.is_ok());
        
        let bytes = result.unwrap();
        assert!(bytes.len() > 0);
        assert_eq!(&bytes[0..2], b"PK");
    }
}
