use crate::renderer::{Renderer, shader_source};
use crate::renderer::{Result, ShaderId};

pub struct Wgpu {}

impl Renderer for Wgpu {
    fn compile_shader(&mut self, _source: &shader_source::ShaderSource) -> Result<ShaderId> {
        unimplemented!("Wgpu shader compilation is not implemented yet");
    }
}