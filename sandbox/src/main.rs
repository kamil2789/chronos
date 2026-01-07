use chronos::{game_engine::{ChronosEngine, RendererType}, window::WindowConfig};

fn main() {
    let mut engine = ChronosEngine::new(WindowConfig::default(), RendererType::Wgpu);
    println!("Chronos Engine created successfully.");

    engine.start().unwrap();
}
