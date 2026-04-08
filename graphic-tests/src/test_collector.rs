use crate::tests::basic_2d_geometries::test_2d_two_triangles;
use chronos::scene::Scene;

pub fn collect_wgpu_tests() -> Vec<Scene> {
    let mut tests = Vec::new();

    tests.push(test_2d_two_triangles());

    tests
}
