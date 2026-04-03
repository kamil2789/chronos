use std::collections::HashMap;

use wgpu::ShaderModule;

use crate::renderer::{RendererError, Result};

pub type ShaderName = String;

static UNIFORM_COLOR_WGSL: &str = include_str!("shaders/uniform_color.wgsl");
static VERTEX_COLOR_WGSL: &str = include_str!("shaders/vertex_color.wgsl");

static SHADER_SOURCES: [(&str, &str); 2] = [
    ("uniform_color", UNIFORM_COLOR_WGSL),
    ("vertex_color", VERTEX_COLOR_WGSL),
];

pub struct ShaderManager {
    shaders_modules: HashMap<ShaderName, ShaderModule>,
}

impl ShaderManager {
    pub fn new(device: &wgpu::Device) -> Result<Self> {
        let mut shaders_modules = HashMap::new();
        for (name, source) in SHADER_SOURCES {
            let error_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(name),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            });
            if let Some(err) = pollster::block_on(error_scope.pop()) {
                return Err(RendererError::Shader(format!("'{}': {}", name, err)));
            }
            shaders_modules.insert(name.to_string(), module);
        }

        Ok(Self { shaders_modules })
    }

    pub fn get_shader(&self, name: &str) -> Option<&ShaderModule> {
        self.shaders_modules.get(name)
    }
}
