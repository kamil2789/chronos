use std::collections::HashMap;

use wgpu::ShaderModule;

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
    pub fn new(device: &wgpu::Device) -> Self {
        let mut shaders_modules = HashMap::new();
        for (name, source) in SHADER_SOURCES {
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(name),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            });
            shaders_modules.insert(name.to_string(), module);
        }

        Self { shaders_modules }
    }

    pub fn get_shader(&self, name: &str) -> Option<&ShaderModule> {
        self.shaders_modules.get(name)
    }
}
