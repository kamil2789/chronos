use crate::tests::basic_2d_geometries::test_2d_two_triangles;
use chronos::scene::Scene;

pub fn collect_wgpu_tests() -> Vec<Scene> {
    vec![test_2d_two_triangles()]
}
