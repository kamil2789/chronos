use chronos::{
    components::{color::RGBA, shape::Shape, texture::TextureComponent},
    scene::Scene,
};
use glam::Vec3;

use crate::assets::texture_ids;

pub fn test_textured_rectangle() -> Scene {
    const TEST_NAME: &str = "2d_textured_rectangle";

    let mut scene = Scene {
        name: String::from(TEST_NAME),
        ..Default::default()
    };

    // Rectangle as 2 triangles (6 vertices), CCW winding
    let rectangle = Shape::new(vec![
        // Triangle 1: bottom-left, bottom-right, top-right
        Vec3::new(-0.5, -0.5, 0.0),
        Vec3::new(0.5, -0.5, 0.0),
        Vec3::new(0.5, 0.5, 0.0),
        // Triangle 2: bottom-left, top-right, top-left
        Vec3::new(-0.5, -0.5, 0.0),
        Vec3::new(0.5, 0.5, 0.0),
        Vec3::new(-0.5, 0.5, 0.0),
    ]);

    let texture = TextureComponent::new(
        texture_ids::WOOD,
        vec![
            [0.0, 1.0], // bottom-left
            [1.0, 1.0], // bottom-right
            [1.0, 0.0], // top-right
            [0.0, 1.0], // bottom-left
            [1.0, 0.0], // top-right
            [0.0, 0.0], // top-left
        ],
    );

    scene.set_background_color(&RGBA::from_hex(0x87_CE_EB_FF));
    scene.entity_manager.create_entity((rectangle, texture));

    scene
}
