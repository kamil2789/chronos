use glam::{Mat4, Quat, Vec3};

pub struct Transform {
    matrix: Mat4,
}

pub struct TransformBuilder {
    translation: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

impl TransformBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }

    #[must_use]
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self
    }

    #[must_use]
    pub fn with_rotation(mut self, rotation: Vec3) -> Self {
        self.rotation = rotation;
        self
    }

    #[must_use]
    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    #[must_use]
    pub fn build(self) -> Transform {
        Transform::new(self.translation, self.rotation, self.scale)
    }
}

impl Transform {
    #[must_use]
    pub fn new(translation: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        let matrix = Self::apply_translation(&translation)
            * Self::apply_rotation(&rotation)
            * Self::apply_scale(&scale);
        Self { matrix }
    }

    #[must_use]
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            matrix: Mat4::from_translation(translation),
        }
    }

    #[must_use]
    pub fn from_rotation(rotation: Vec3) -> Self {
        Self {
            matrix: Self::apply_rotation(&rotation),
        }
    }

    #[must_use]
    pub fn from_rotation_axis_angle(axis: Vec3, angle_degrees: f32) -> Self {
        Self {
            matrix: Mat4::from_axis_angle(axis.normalize(), angle_degrees.to_radians()),
        }
    }

    #[must_use]
    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            matrix: Mat4::from_scale(scale),
        }
    }

    #[must_use]
    pub fn identity() -> Self {
        Self {
            matrix: Mat4::IDENTITY,
        }
    }

    #[must_use]
    pub fn matrix(&self) -> Mat4 {
        self.matrix
    }

    #[must_use]
    fn apply_translation(translation: &Vec3) -> Mat4 {
        Mat4::from_translation(*translation)
    }

    #[must_use]
    fn apply_rotation(rotation: &Vec3) -> Mat4 {
        let quat = Quat::from_euler(
            glam::EulerRot::XYZ,
            rotation.x.to_radians(),
            rotation.y.to_radians(),
            rotation.z.to_radians(),
        );
        Mat4::from_quat(quat)
    }

    #[must_use]
    fn apply_scale(scale: &Vec3) -> Mat4 {
        Mat4::from_scale(*scale)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

impl Default for TransformBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::AbsDiffEq;
    use glam::{vec3, vec4};

    const EPSILON: f32 = 0.0001;

    #[test]
    fn test_default_transform() {
        let result = Transform::default();
        assert_eq!(result.matrix, Mat4::IDENTITY);
    }

    #[test]
    fn test_identity() {
        let result = Transform::identity();
        assert_eq!(result.matrix, Mat4::IDENTITY);
    }

    #[test]
    fn test_new_builder() {
        let builder = TransformBuilder::new();
        assert_eq!(builder.translation, Vec3::ZERO);
        assert_eq!(builder.rotation, Vec3::ZERO);
        assert_eq!(builder.scale, Vec3::ONE);
    }

    #[test]
    fn test_builder() {
        let result = TransformBuilder::new()
            .with_translation(vec3(1.0, 2.0, 3.0))
            .with_rotation(vec3(4.0, 5.0, 6.0))
            .with_scale(vec3(7.0, 8.0, 9.0))
            .build();

        let expected = Transform::new(
            vec3(1.0, 2.0, 3.0),
            vec3(4.0, 5.0, 6.0),
            vec3(7.0, 8.0, 9.0),
        );

        assert!(result.matrix().abs_diff_eq(expected.matrix(), EPSILON));
    }

    #[test]
    fn test_transform_translate() {
        let result = Transform::from_translation(vec3(1.0, 2.0, 3.0));
        let expected = Mat4::from_translation(vec3(1.0, 2.0, 3.0));
        assert_eq!(result.matrix(), expected);
    }

    #[test]
    fn test_transform_scale() {
        let result = Transform::from_scale(vec3(7.0, 8.0, 9.0));
        let expected = Mat4::from_scale(vec3(7.0, 8.0, 9.0));
        assert_eq!(result.matrix(), expected);
    }

    #[test]
    fn test_transform_x_axis_rotation_30_degrees() {
        let transform = Transform::from_rotation(vec3(30.0, 0.0, 0.0));
        let matrix = transform.matrix();

        let vertex = vec4(2.0, 3.0, 4.0, 1.0);
        let result = matrix * vertex;

        let expected = vec4(2.0, 0.5980762, 4.964102, 1.0);
        assert!(result.abs_diff_eq(expected, EPSILON));
    }

    #[test]
    fn test_rotation_axis_angle() {
        let transform = Transform::from_rotation_axis_angle(vec3(1.0, 1.5, 3.0), 4.0);
        let result = transform.matrix();

        // Expected matrix for rotation around normalized axis (1, 1.5, 3) by 4 degrees
        assert!(result.w_axis.w.abs_diff_eq(&1.0, EPSILON));
        assert!(result.w_axis.x.abs_diff_eq(&0.0, EPSILON));
        assert!(result.w_axis.y.abs_diff_eq(&0.0, EPSILON));
        assert!(result.w_axis.z.abs_diff_eq(&0.0, EPSILON));
    }

    #[test]
    fn test_combined_transform() {
        let result = Transform::new(
            vec3(1.0, 2.0, 3.0),
            vec3(0.0, 0.0, 0.0),
            vec3(1.0, 1.0, 1.0),
        );

        let expected = Mat4::from_translation(vec3(1.0, 2.0, 3.0));
        assert_eq!(result.matrix(), expected);
    }
}
