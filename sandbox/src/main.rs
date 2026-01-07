use chronos::game_engine::ChronosEngine;

fn main() {
    let mut engine = ChronosEngine::new(
        chronos::window::WindowConfig::default(),
        &chronos::game_engine::RendererType::Wgpu,
    )
    .unwrap();
    println!("Chronos Engine created successfully.");

    engine.start().unwrap();
}
