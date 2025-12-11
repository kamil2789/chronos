mod shader_compiler;

use crate::renderer::{Renderer, Result, ShaderId, shader_source::ShaderSource};

#[derive(Default)]
pub struct OpenGL {}

impl Renderer for OpenGL {
    fn compile_shader(&mut self, source: &ShaderSource) -> Result<ShaderId> {
        shader_compiler::compile(source.get_vertex_shader(), source.get_fragment_shader())
    }
}
