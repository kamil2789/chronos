use chronos::{
    components::{
        color::{Color, RGBA},
        shape::Shape,
    },
    configs::EngineConfig,
    game_engine::ChronosEngine,
    scene::Scene,
};
use glam::Vec3;

fn main() {
    let mut engine = ChronosEngine::new(EngineConfig::default());
    println!("Chronos Engine created successfully.");

    let mut scene = Scene::default();

    let triangle_one = Shape::new_triangle(
        Vec3::new(0.2, -0.5, 0.0),
        Vec3::new(0.7, -0.5, 0.0),
        Vec3::new(0.45, 0.5, 0.0),
    );
    let color_one = Color::Uniform(RGBA::green());

    let triangle_two = Shape::new_triangle(
        Vec3::new(-0.9, -0.5, 0.0),  // lewy dolny
        Vec3::new(-0.2, -0.5, 0.0),  // prawy dolny
        Vec3::new(-0.45, 0.5, 0.0),  // górny środek
    );
    let color_two = Color::PerVertex(vec![
        1.0, 0.0, 0.0, 1.0,  // Czerwony (lewy dolny)
        0.0, 1.0, 0.0, 1.0,  // Zielony (prawy dolny)
        0.0, 0.0, 1.0, 1.0,  // Niebieski (górny)
    ]);

    scene.set_background_color(&RGBA::from_hex(0xAA_BB_CC_FF));
    scene.entity_manager.create_entity((triangle_one, color_one));
    scene.entity_manager.create_entity((triangle_two, color_two));
    engine.register_scene(scene);
    engine.start().unwrap();
}
