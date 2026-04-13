use std::path::Path;

use chronos::texture_registry::{TextureData, TextureDataError};

use crate::workspace;

pub fn load_texture_from_assets(filename: &str) -> Result<TextureData, TextureDataError> {
    let path = Path::new(workspace::TEXTURES_DIR).join(filename);
    load_texture_rgba(&path)
}

pub fn load_texture_rgba(path: &Path) -> Result<TextureData, TextureDataError> {
    let img = image::open(path)
        .unwrap_or_else(|e| panic!("Failed to open texture '{}': {e}", path.display()))
        .to_rgba8();
    let (width, height) = img.dimensions();
    TextureData::from_rgba(width, height, img.into_raw())
}
