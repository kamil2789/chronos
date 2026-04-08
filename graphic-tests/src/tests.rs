use std::path::Path;

use chronos::graphic_engine::{HeadlessRenderer, RendererType};
use image::{ImageBuffer, Rgba};

use crate::{
    args_parser::{Args, GraphicApi},
    workspace::{self, prepare_working_directory},
};

mod basic_2d_geometries;

const RENDER_WIDTH: u32 = 1280;
const RENDER_HEIGHT: u32 = 720;

pub fn run(args: &Args) {
    println!("Running graphic tests...");
    prepare_working_directory();

    let renderer_types = match args.graphic_api {
        GraphicApi::All | GraphicApi::Wgpu => vec![RendererType::Wgpu],
    };

    for renderer_type in &renderer_types {
        if args.test_name == "All" || args.test_name == "two_triangles" {
            run_test("two_triangles", renderer_type, || {
                basic_2d_geometries::two_triangles_scene()
            });
        }
    }

    println!("All graphic tests finished.");
}

fn run_test(
    test_name: &str,
    renderer_type: &RendererType,
    scene_fn: impl FnOnce() -> chronos::scene::Scene,
) {
    let api_name = match renderer_type {
        RendererType::Wgpu => "wgpu",
    };
    println!("  [{api_name}] Running test: {test_name}");

    let mut renderer = HeadlessRenderer::new(RENDER_WIDTH, RENDER_HEIGHT, renderer_type)
        .expect("Failed to create headless renderer");

    let scene = scene_fn();
    let pixels = renderer
        .render_to_buffer(&scene)
        .expect("Failed to render scene");

    let golden_path = format!(
        "{}{}_{}.png",
        workspace::GOLDEN_DIR,
        test_name,
        api_name
    );

    if !Path::new(&golden_path).exists() {
        save_image(&golden_path, RENDER_WIDTH, RENDER_HEIGHT, &pixels);
        println!("    -> Golden image generated: {golden_path}");
    } else {
        println!("    -> Golden image already exists: {golden_path}");
    }
}

fn save_image(path: &str, width: u32, height: u32, rgba_data: &[u8]) {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    let img: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(width, height, rgba_data.to_vec())
            .expect("Failed to create image buffer");
    img.save(path).expect("Failed to save image");
}