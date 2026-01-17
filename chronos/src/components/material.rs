use crate::components::color::RGBA;

pub enum ShaderType {
    UniformColor,
    VertexColor,
    Textured,
    Custom(String),
}

pub struct Material {
    shader: ShaderType,
    color: Option<RGBA>,
}

impl Material {
    pub fn new(shader: ShaderType, color: RGBA) -> Self {
        Material {
            shader,
            color: Some(color),
        }
    }

    pub fn new_with_uniform_color(color: RGBA) -> Self {
        Material {
            shader: ShaderType::UniformColor,
            color: Some(color),
        }
    }

    pub fn new_with_vertex_color() -> Self {
        Material {
            shader: ShaderType::VertexColor,
            color: None,
        }
    }

    pub fn new_with_custom_shader(shader_name: &str) -> Self {
        Material {
            shader: ShaderType::Custom(shader_name.into()),
            color: None,
        }
    }

    pub fn shader_name(&self) -> &str {
        match &self.shader {
            ShaderType::UniformColor => "uniform_color",
            ShaderType::VertexColor => "vertex_color",
            ShaderType::Textured => "textured",
            ShaderType::Custom(name) => name,
        }
    }

    pub fn get_color(&self) -> Option<&RGBA> {
        self.color.as_ref()
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            shader: ShaderType::UniformColor,
            color: Some(RGBA::from_hex(0xFF_FF_FF_FF)),
        }
    }
}
