use crate::{components::color::RGBA, entity::EntityManager};

pub struct Scene {
    pub name: String,
    pub entity_manager: EntityManager,
    pub background_color: RGBA,
}

impl Scene {
    pub fn set_background_color(&mut self, color: &RGBA) {
        self.background_color = color.clone();
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            name: String::from("Default Scene"),
            entity_manager: EntityManager::new(100),
            background_color: RGBA::from_hex(0x00_00_00_FF),
        }
    }
}
