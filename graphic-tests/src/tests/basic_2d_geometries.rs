use chronos::{
    components::{
        color::{Color, RGBA},
        shape::Shape,
    },
    scene::Scene,
};
use glam::Vec3;

pub fn test_2d_two_triangles() -> Scene {
    const TEST_NAME: &str = "2d_two_triangles";

    let mut scene = Scene {
        name: String::from(TEST_NAME),
        ..Default::default()
    };

    let triangle_one = Shape::new_triangle(
        Vec3::new(0.2, -0.5, 0.0),
        Vec3::new(0.7, -0.5, 0.0),
        Vec3::new(0.45, 0.5, 0.0),
    );
    let color_one = Color::Uniform(RGBA::green());

    let triangle_two = Shape::new_triangle(
        Vec3::new(-0.9, -0.5, 0.0),
        Vec3::new(-0.2, -0.5, 0.0),
        Vec3::new(-0.45, 0.5, 0.0),
    );
    let color_two = Color::PerVertex(vec![
        1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
    ]);

    scene.set_background_color(&RGBA::from_hex(0xAA_BB_CC_FF));
    scene
        .entity_manager
        .create_entity((triangle_one, color_one));
    scene
        .entity_manager
        .create_entity((triangle_two, color_two));

    scene
}
