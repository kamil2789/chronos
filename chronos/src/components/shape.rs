use glam::Vec3;

#[derive(Clone, PartialEq, Debug)]
pub struct Shape {
    vertices: Vec<Vec3>,
}

impl Shape {
    #[must_use]
    pub fn new(vertices: Vec<Vec3>) -> Self {
        Self { vertices }
    }

    #[must_use]
    pub fn new_triangle(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        Self {
            vertices: vec![v1, v2, v3],
        }
    }

    #[must_use]
    pub fn new_rectangle(v1: Vec3, v2: Vec3, v3: Vec3, v4: Vec3) -> Self {
        Self {
            vertices: vec![v1, v2, v3, v4],
        }
    }

    #[must_use]
    pub fn new_circle(center: Vec3, radius: f32, segments: usize) -> Self {
        let mut vertices = Vec::with_capacity(segments + 1);

        vertices.push(center);

        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            vertices.push(Vec3::new(x, y, center.z));
        }

        Self { vertices }
    }

    #[must_use]
    pub fn get_vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_new_with_vertices() {
        let vertices = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];
        let shape = Shape::new(vertices.clone());
        assert_eq!(shape.get_vertices().len(), 3);
        assert_eq!(shape.get_vertices(), &vertices);
    }

    #[test]
    fn test_new_triangle() {
        let v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(1.0, 0.0, 0.0);
        let v3 = Vec3::new(0.0, 1.0, 0.0);
        let triangle = Shape::new_triangle(v1, v2, v3);

        assert_eq!(triangle.get_vertices().len(), 3);
        assert_eq!(triangle.get_vertices()[0], v1);
        assert_eq!(triangle.get_vertices()[1], v2);
        assert_eq!(triangle.get_vertices()[2], v3);
    }

    #[test]
    fn test_new_rectangle() {
        let v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(1.0, 0.0, 0.0);
        let v3 = Vec3::new(1.0, 1.0, 0.0);
        let v4 = Vec3::new(0.0, 1.0, 0.0);
        let rectangle = Shape::new_rectangle(v1, v2, v3, v4);

        assert_eq!(rectangle.get_vertices().len(), 4);
        assert_eq!(rectangle.get_vertices()[0], v1);
        assert_eq!(rectangle.get_vertices()[1], v2);
        assert_eq!(rectangle.get_vertices()[2], v3);
        assert_eq!(rectangle.get_vertices()[3], v4);
    }

    #[test]
    fn test_new_circle() {
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let segments = 8;
        let circle = Shape::new_circle(center, radius, segments);

        assert_eq!(circle.get_vertices().len(), segments + 1);
        assert_eq!(circle.get_vertices()[0], center);
    }

    #[test]
    fn test_circle_radius() {
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let segments = 16;
        let circle = Shape::new_circle(center, radius, segments);

        // Check if first point is at correct distance from center
        let first_point = circle.get_vertices()[1];
        let distance =
            ((first_point.x - center.x).powi(2) + (first_point.y - center.y).powi(2)).sqrt();
        assert_relative_eq!(distance, radius, epsilon = 0.0001);
    }

    #[test]
    fn test_circle_z_coordinate() {
        let center = Vec3::new(1.0, 2.0, 3.0);
        let radius = 2.0;
        let segments = 12;
        let circle = Shape::new_circle(center, radius, segments);

        for vertex in circle.get_vertices() {
            assert_eq!(vertex.z, center.z);
        }
    }

    #[test]
    fn test_get_vertices() {
        let vertices = vec![Vec3::new(1.0, 2.0, 3.0)];
        let shape = Shape::new(vertices.clone());
        let returned_vertices = shape.get_vertices();

        assert_eq!(returned_vertices, &vertices);
    }
}
