use chronos::game_engine::ChronosEngine;

fn main() {
    let mut engine = ChronosEngine::new(chronos::window::WindowConfig::default()).unwrap();
    println!("Chronos Engine created successfully.");

    engine.start().unwrap();
}
