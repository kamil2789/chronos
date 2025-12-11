mod shader_compiler;

use glutin::{
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::GlSurface,
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::renderer::{Renderer, Result, ShaderId, shader_source::ShaderSource};

use glow::HasContext;
use std::num::NonZeroU32;

pub struct OpenGL {
    pub gl: glow::Context,
    pub gl_context: glutin::context::PossiblyCurrentContext,
    pub surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
}

pub fn init_opengl(window: &winit::window::Window) -> OpenGL {
    //
    // 1. Pobieramy raw handles do okna i display
    //
    let raw_window = window.window_handle().unwrap().as_raw();
    let raw_display = window.display_handle().unwrap().as_raw();

    //
    // 2. Tworzymy konfigurację kontekstu OpenGL
    //
    let template = glutin::config::ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_depth_size(24)
        .with_stencil_size(8)
        .build();

    let display = unsafe {
        glutin::display::Display::new(
            raw_display,
            glutin::display::DisplayApiPreference::Wgl(Some(raw_window)),
        ).expect("Failed to create WGL display")
    };

    let config = unsafe {
        display
            .find_configs(template)
            .unwrap()
            .next()
            .expect("No GL configs found")
    };

    //
    // 3. Tworzymy atrybuty kontekstu (OpenGL Core)
    //
    let context_attributes = glutin::context::ContextAttributesBuilder::new()
        .with_context_api(glutin::context::ContextApi::OpenGl(None))
        .build(Some(raw_window));

    let not_current_context = unsafe {
        display
            .create_context(&config, &context_attributes)
            .expect("Failed to create OpenGL context")
    };

    //
    // 4. Tworzymy surface powiązany z oknem
    //
    let size = window.inner_size();
    let attrs = glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
        .build(
            raw_window,
            NonZeroU32::new(size.width).unwrap(),
            NonZeroU32::new(size.height).unwrap(),
        );

    let surface = unsafe {
        display
            .create_window_surface(&config, &attrs)
            .expect("Failed to create GL surface")
    };

    //
    // 5. Ustawiamy kontekst jako aktywny (current)
    //
    let gl_context = not_current_context
        .make_current(&surface)
        .expect("Failed to make GL context current");

    //
    // 6. Loader glow
    //
    let gl = unsafe {
        glow::Context::from_loader_function(|symbol| {
            display.get_proc_address(&std::ffi::CString::new(symbol).unwrap()) as *const _
        })
    };

    //
    // 7. Test że GL działa
    //
    unsafe {
        gl.clear_color(0.1, 0.2, 0.3, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);
    }
    surface.swap_buffers(&gl_context).unwrap();

    OpenGL {
        gl,
        gl_context,
        surface,
    }
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
