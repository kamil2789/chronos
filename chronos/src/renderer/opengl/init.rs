use std::num::NonZeroU32;

use crate::{
    renderer::{RendererError, Result},
    window::ChronosWindow,
};
use glutin::{
    config::{Config, ConfigTemplate},
    context::{ContextApi, ContextAttributesBuilder, NotCurrentContext},
    display::Display,
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{Surface, SurfaceAttributes, SurfaceAttributesBuilder, WindowSurface},
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

pub struct RawHandles {
    pub window: RawWindowHandle,
    pub display: RawDisplayHandle,
}

pub fn create_raw_handles(window: &ChronosWindow) -> Result<RawHandles> {
    if let Some(window) = window.get_window() {
        let raw_window = window
            .window_handle()
            .map_err(|e| {
                RendererError::Initialization(format!("Failed to get window handle: {e}"))
            })?
            .as_raw();
        let raw_display = window
            .display_handle()
            .map_err(|e| {
                RendererError::Initialization(format!("Failed to get display handle: {e}"))
            })?
            .as_raw();
        Ok(RawHandles {
            window: raw_window,
            display: raw_display,
        })
    } else {
        Err(RendererError::Initialization("Window not available".into()))
    }
}

pub fn create_display(handles: &RawHandles) -> Result<Display> {
    let display = unsafe {
        glutin::display::Display::new(
            handles.display,
            glutin::display::DisplayApiPreference::WglThenEgl(Some(handles.window)),
        )
        .map_err(|e| RendererError::Initialization(format!("Failed to create display: {e}")))?
    };
    Ok(display)
}

pub fn create_framebuffer_config(display: &Display) -> Result<Config> {
    unsafe {
        display
            .find_configs(ConfigTemplate::default())
            .map_err(|e| {
                RendererError::Initialization(format!("Failed to find framebuffer configs: {e}"))
            })?
            .next()
            .ok_or_else(|| RendererError::Initialization("No framebuffer configs found".into()))
    }
}

pub fn create_surface_attributes(
    window: &ChronosWindow,
    handles: &RawHandles,
) -> Result<SurfaceAttributes<WindowSurface>> {
    let inner_size = window.get_inner_size().unwrap_or_default();

    let width = NonZeroU32::new(inner_size.width.max(1))
        .ok_or_else(|| RendererError::Initialization("Window width must be non-zero".into()))?;

    let height = NonZeroU32::new(inner_size.height.max(1))
        .ok_or_else(|| RendererError::Initialization("Window height must be non-zero".into()))?;

    Ok(SurfaceAttributesBuilder::<WindowSurface>::new().build(handles.window, width, height))
}

pub fn create_surface(
    framebuffer_config: &Config,
    surface_attributes: &SurfaceAttributes<WindowSurface>,
    display: &Display,
) -> Result<Surface<WindowSurface>> {
    unsafe {
        display
            .create_window_surface(framebuffer_config, surface_attributes)
            .map_err(|e| RendererError::Initialization(format!("Failed to create GL surface: {e}")))
    }
}

pub fn create_context(
    handles: &RawHandles,
    display: &Display,
    framebuffer_config: &Config,
) -> Result<NotCurrentContext> {
    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(None))
        .build(Some(handles.window));

    unsafe {
        display
            .create_context(framebuffer_config, &context_attributes)
            .map_err(|e| {
                RendererError::Initialization(format!("Failed to create OpenGL context: {e}"))
            })
    }
}

pub fn make_context_current(
    not_current_context: NotCurrentContext,
    surface: &Surface<WindowSurface>,
) -> Result<glutin::context::PossiblyCurrentContext> {
    not_current_context.make_current(surface).map_err(|e| {
        RendererError::Initialization(format!("Failed to make GL context current: {e}"))
    })
}

pub fn load_gl_functions(display: &Display) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|symbol| {
            std::ffi::CString::new(symbol)
                .map(|c_str| display.get_proc_address(&c_str))
                .unwrap_or(std::ptr::null())
        })
    }
}
