use chronos::graphic_engine::ChronosEngine;
use chronos::texture_registry::TextureDataError;

use crate::texture_loader;

pub mod texture_ids {
    pub const WOOD: &str = "wood";
}

pub fn register_assets(engine: &mut ChronosEngine) -> Result<(), TextureDataError> {
    let texture_data = texture_loader::load_texture_from_assets("wodden_container.png")?;
    engine.register_texture(texture_ids::WOOD, texture_data);
    Ok(())
}
