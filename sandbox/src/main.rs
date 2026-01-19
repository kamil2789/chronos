use chronos::{
    components::color::RGBA, configs::EngineConfig, game_engine::ChronosEngine, scene::Scene,
};

fn main() {
    let mut engine = ChronosEngine::new(EngineConfig::default());
    println!("Chronos Engine created successfully.");

    let mut scene = Scene::default();
    scene.set_background_color(&RGBA::from_hex(0xFF_00_00_FF));
    engine.register_scene(scene);
    engine.start().unwrap();
}
