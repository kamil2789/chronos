mod init;
mod shader_compiler;

use glow::Context;
use glutin::{
    context::PossiblyCurrentContext,
    surface::{self, Surface},
};

use crate::{
    renderer::{Renderer, Result, ShaderId, shader_source::ShaderSource},
    window::ChronosWindow,
};

#[allow(dead_code)]
pub struct OpenGL {
    pub gl: Context,
    pub gl_context: PossiblyCurrentContext,
    pub surface: Surface<surface::WindowSurface>,
}

pub fn init_opengl(window: &ChronosWindow) -> Result<OpenGL> {
    let handles = init::create_raw_handles(window)?;
    let display = init::create_display(&handles)?;
    let framebuffer_config = init::create_framebuffer_config(&display)?;
    let surface_attributes = init::create_surface_attributes(window, &handles)?;
    let context = init::create_context(&handles, &display, &framebuffer_config)?;

    let surface = init::create_surface(&framebuffer_config, &surface_attributes, &display)?;
    let gl_context = init::make_context_current(context, &surface)?;
    let gl = init::load_gl_functions(&display);

    Ok(OpenGL {
        gl,
        gl_context,
        surface,
    })
}

impl Renderer for OpenGL {
    fn compile_shader(&mut self, source: &ShaderSource) -> Result<ShaderId> {
        shader_compiler::compile(
            &self.gl,
            source.get_vertex_shader(),
            source.get_fragment_shader(),
        )
    }
}

unsafe impl Sync for OpenGL {}
unsafe impl Send for OpenGL {}
