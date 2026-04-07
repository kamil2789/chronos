use chronos::{configs::EngineConfig, graphic_engine::{ChronosEngine, RendererType}};

use crate::{args_parser::Args, workspace::prepare_working_directory};

mod basic_2d_geometries;

pub fn run(args: &Args) {
    println!("Hello from graphic tests!");
    prepare_working_directory();
    let engine = create_engine(RendererType::Wgpu);
    //let window = Rc::new(create_window(&config));
    //dispatch_tests(&window, args);
}

pub fn create_engine(renderer_type: RendererType) -> ChronosEngine {
    let config = EngineConfig{
        window: Default::default(),
        renderer_type,
    };

    match renderer_type {
        RendererType::Wgpu => {ChronosEngine::new(config)}
    }
}