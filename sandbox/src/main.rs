use chronos::{components::color::RGBA, configs::EngineConfig, game_engine::ChronosEngine};

fn main() {
    let mut engine = ChronosEngine::new(EngineConfig::default());
    println!("Chronos Engine created successfully.");

    engine.set_background_color(&RGBA::from_hex(0xFF_00_00_FF)); //Nie trafia kolor bo na ten moment nie ma renderer'a
    engine.start().unwrap();
}
