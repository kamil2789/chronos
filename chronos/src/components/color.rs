pub struct RGBA {
    rgb: glam::U8Vec3,
    alpha: f32,
}

pub enum Color {
    Uniform(RGBA),
    PerVertex(Vec<f32>),
}

impl RGBA {
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8, alpha: f32) -> Self {
        Self {
            rgb: glam::U8Vec3::new(r, g, b),
            alpha: alpha.clamp(0.0, 1.0),
        }
    }

    #[must_use]
    pub fn from_hex(hex: u32) -> Self {
        let bytes = hex.to_be_bytes();
        Self {
            rgb: glam::U8Vec3::new(bytes[0], bytes[1], bytes[2]),
            alpha: f32::from(bytes[3]) / 255.0,
        }
    }

    #[must_use]
    pub fn empty() -> Self {
        Self {
            rgb: glam::U8Vec3::ZERO,
            alpha: 1_f32,
        }
    }

    #[must_use]
    pub fn get(&self) -> (u8, u8, u8, f32) {
        (self.rgb.x, self.rgb.y, self.rgb.z, self.alpha)
    }
}

impl Color {
    #[must_use]
    pub fn uniform(color: RGBA) -> Self {
        Self::Uniform(color)
    }

    #[must_use]
    pub fn per_vertex(colors: Vec<f32>) -> Self {
        Self::PerVertex(colors)
    }

    #[must_use]
    pub fn is_uniform(&self) -> bool {
        matches!(self, Self::Uniform(_))
    }

    #[must_use]
    pub fn get_uniform_color(&self) -> Option<&RGBA> {
        if let Self::Uniform(color) = self {
            Some(color)
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_vertex_colors(&self) -> Option<&Vec<f32>> {
        if let Self::PerVertex(colors) = self {
            Some(colors)
        } else {
            None
        }
    }
}

impl Default for RGBA {
    fn default() -> Self {
        Self {
            rgb: glam::U8Vec3::ONE,
            alpha: 1.0,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Uniform(RGBA::default())
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, RGBA};

    #[test]
    fn test_rgba_new() {
        let color = RGBA::new(255, 128, 64, 0.5);
        assert_eq!(color.get(), (255, 128, 64, 0.5));
    }

    #[test]
    fn test_rgba_from_hex() {
        let red = RGBA::from_hex(0xFF_00_00_FF);
        assert_eq!(red.get(), (255, 0, 0, 1.0));

        let magnolia = RGBA::from_hex(0xF8_F4_FF_FF);
        assert_eq!(magnolia.get(), (248, 244, 255, 1.0));
    }

    #[test]
    fn test_rgba_empty() {
        let empty = RGBA::empty();
        assert_eq!(empty.get(), (0, 0, 0, 1.0));
    }

    #[test]
    fn test_color_uniform() {
        let uniform_color = Color::uniform(RGBA::new(0, 255, 0, 1.0));
        assert!(uniform_color.is_uniform());
        assert_eq!(
            uniform_color.get_uniform_color().unwrap().get(),
            (0, 255, 0, 1.0)
        );
    }

    #[test]
    fn test_color_per_vertex() {
        let per_vertex_color = Color::per_vertex(vec![1.0, 0.0, 0.0, 1.0]);
        assert_eq!(
            per_vertex_color.get_vertex_colors().unwrap(),
            &vec![1.0, 0.0, 0.0, 1.0]
        );
        assert!(!per_vertex_color.is_uniform());
    }
}
