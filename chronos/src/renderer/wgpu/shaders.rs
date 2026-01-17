use std::collections::HashMap;

use wgpu::{ShaderModule, ShaderSource};

pub type ShaderName = String;

pub static UNIFORM_COLOR_WGSL: &str = include_str!("shaders/uniform_color.wgsl");
pub static VERTEX_COLOR_WGSL: &str = include_str!("shaders/vertex_color.wgsl");

pub struct ShaderManager {
    shaders_src: HashMap<ShaderName, ShaderSource<'static>>,
    shaders_modules: HashMap<ShaderName, ShaderModule>,
}

impl ShaderManager {
    pub fn get_shader(&self, name: &str) -> Option<&ShaderModule> {
        self.shaders_modules.get(name)
    }

    pub fn compile_all(&mut self, device: &wgpu::Device) -> Result<(), String> {
        for (name, source) in &self.shaders_src {
            let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(name),
                source: source.clone(),
            });
            self.shaders_modules.insert(name.clone(), shader_module);
        }
        Ok(())
    }
}

impl Default for ShaderManager {
    fn default() -> Self {
        let mut shaders_src = HashMap::new();
        shaders_src.insert(
            "uniform_color".to_string(),
            ShaderSource::Wgsl(UNIFORM_COLOR_WGSL.into()),
        );
        shaders_src.insert(
            "vertex_color".to_string(),
            ShaderSource::Wgsl(VERTEX_COLOR_WGSL.into()),
        );
        ShaderManager {
            shaders_src,
            shaders_modules: HashMap::new(),
        }
    }
}
