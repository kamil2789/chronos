#[derive(Clone, Debug)]
pub struct TextureComponent {
    label: String,
    texture_mapping: Vec<[f32; 2]>,
}

impl TextureComponent {
    #[must_use]
    pub fn new(label: &str, texture_mapping: Vec<[f32; 2]>) -> Self {
        Self {
            label: label.to_string(),
            texture_mapping,
        }
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub fn texture_mapping(&self) -> &[[f32; 2]] {
        &self.texture_mapping
    }
}
