use chronos::game_engine::ChronosEngine;

fn main() {
    let _ = ChronosEngine::new(
        chronos::window::WindowConfig::default(),
        &chronos::game_engine::RendererType::Wgpu,
    )
    .unwrap();
    println!("Chronos Engine created successfully.");
}
