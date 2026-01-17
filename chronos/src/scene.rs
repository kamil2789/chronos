use crate::{components::color::RGBA, entity::EntityManager};

pub struct Scene {
    pub entity_manager: EntityManager,
    pub background_color: RGBA,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            entity_manager: EntityManager::new(100),
            background_color: RGBA::from_hex(0x00_00_00_FF),
        }
    }
}
