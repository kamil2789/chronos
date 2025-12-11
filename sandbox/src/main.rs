use chronos::game_engine::ChronosEngine;

fn main() {
    let engine = ChronosEngine::start(
        chronos::window::WindowConfig::default(),
        chronos::game_engine::RendererType::OpenGL,
    )
    .unwrap();
}
