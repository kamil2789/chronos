use chronos::{configs::EngineConfig, game_engine::ChronosEngine};

fn main() {
    let mut engine = ChronosEngine::new(EngineConfig::default());
    println!("Chronos Engine created successfully.");

    engine.start().unwrap();
}
