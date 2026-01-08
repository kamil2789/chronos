use crate::game_engine::RendererType;

pub struct EngineConfig {
    pub window: WindowConfig,
    pub renderer_type: RendererType,
}

pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

pub struct WindowConfig {
    pub resolution: Resolution,
    pub title: String,
    pub resizable: bool,
}

impl WindowConfig {
    pub fn new(resolution: Resolution, title: &str, resizable: bool) -> Self {
        Self {
            resolution,
            title: title.to_string(),
            resizable,
        }
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            renderer_type: RendererType::Wgpu,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            resolution: Resolution {
                width: 1280,
                height: 720,
            },
            title: "Chronos Engine".to_string(),
            resizable: true,
        }
    }
}
