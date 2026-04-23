use std::collections::HashMap;
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
#[error("RGBA buffer size mismatch: expected {expected} bytes ({width}x{height}x4), got {actual}")]
pub struct TextureDataError {
    expected: usize,
    actual: usize,
    width: u32,
    height: u32,
}

pub struct TextureData {
    width: u32,
    height: u32,
    bytes: Vec<u8>,
}

#[derive(Default)]
pub(crate) struct TextureRegistry {
    textures: HashMap<String, Arc<TextureData>>,
}

impl TextureData {
    /// Creates a new `TextureData` from raw RGBA bytes.
    ///
    /// # Errors
    ///
    /// Returns [`TextureDataError`] if `bytes.len() != width * height * 4`.
    pub fn from_rgba(width: u32, height: u32, bytes: Vec<u8>) -> Result<Self, TextureDataError> {
        let expected = (width * height * 4) as usize;
        if bytes.len() != expected {
            return Err(TextureDataError {
                expected,
                actual: bytes.len(),
                width,
                height,
            });
        }
        Ok(Self {
            width,
            height,
            bytes,
        })
    }

    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl TextureRegistry {
    pub fn register(&mut self, id: &str, data: TextureData) {
        self.textures.insert(id.to_string(), Arc::new(data));
    }

    #[must_use]
    pub fn get(&self, id: &str) -> Option<Arc<TextureData>> {
        self.textures.get(id).cloned()
    }
}
