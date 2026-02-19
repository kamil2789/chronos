use std::collections::HashMap;

use wgpu::{ShaderModule, ShaderSource};

pub type ShaderName = String;

pub struct ShaderManager {
    shaders_src: HashMap<ShaderName, ShaderSource<'static>>,
    shaders_modules: HashMap<ShaderName, ShaderModule>,
}

impl ShaderManager {
    pub fn get_shader(&self, name: &str) -> Option<&ShaderModule> {
        self.shaders_modules.get(name)
    }

    pub fn compile_all(&mut self, device: &wgpu::Device) {
        for (name, source) in &self.shaders_src {
            let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(name),
                source: source.clone(),
            });
            self.shaders_modules.insert(name.clone(), shader_module);
        }
    }
}

macro_rules! shaders_src {
    ($($name:literal => $file:literal),* $(,)?) => {
        HashMap::from([
            $(
                (
                    $name.to_string(),
                    ShaderSource::Wgsl(include_str!(concat!("shaders/", $file)).into()),
                ),
            )*
        ])
    };
}

impl Default for ShaderManager {
    fn default() -> Self {
        ShaderManager {
            shaders_src: shaders_src! {
                "uniform_color" => "uniform_color.wgsl",
                "vertex_color"  => "vertex_color.wgsl",
            },
            shaders_modules: HashMap::new(),
        }
    }
}
