use crate::tests::basic_2d_geometries::test_2d_two_triangles;
use crate::tests::basic_2d_textures::test_textured_rectangle;
use chronos::scene::Scene;

pub fn collect_wgpu_tests() -> Vec<Scene> {
    vec![test_2d_two_triangles(), test_textured_rectangle()]
}
