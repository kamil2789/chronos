use std::collections::BTreeMap;

use tracing::warn;

use crate::graphic_engine::Result;
use crate::{components::color::RGBA, entity::EntityManager, graphic_engine::EngineError};

pub struct Scene {
    pub name: String,
    pub entity_manager: EntityManager,
    pub background_color: RGBA,
}

#[derive(Default)]
pub(crate) struct SceneManager {
    pub(crate) scenes: BTreeMap<String, Scene>,
    active_scene: Option<String>,
}

impl Scene {
    pub fn set_background_color(&mut self, color: &RGBA) {
        self.background_color = color.clone();
    }
}

impl SceneManager {
    #[must_use]
    pub fn get_active_scene(&self) -> Option<&Scene> {
        if let Some(active_scene_name) = &self.active_scene {
            self.scenes.get(active_scene_name)
        } else {
            None
        }
    }

    pub fn register_scene(&mut self, scene: Scene) {
        let scene_name = scene.name.clone();
        if self.scenes.is_empty() {
            self.active_scene = Some(scene_name.clone());
        }
        self.scenes.insert(scene_name, scene);
    }

    pub fn unregister_scene(&mut self, name: &str) {
        self.scenes.remove(name);
        if self.active_scene.as_deref() == Some(name) {
            self.active_scene = None;
            warn!("Current scene was unregistered, no active scene now");
        }
    }

    pub fn set_current_scene(&mut self, name: &str) -> Result<()> {
        if self.scenes.contains_key(name) {
            self.active_scene = Some(name.to_string());
            Ok(())
        } else {
            Err(EngineError::SceneNotFound(name.to_string()))
        }
    }

    pub fn get_scenes(&self) -> impl Iterator<Item = &String> {
        self.scenes.keys()
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
