#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AddressMode {
    #[default]
    ClampToEdge,
    Repeat,
    MirrorRepeat,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FilterMode {
    #[default]
    Linear,
    Nearest,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MipmapFilterMode {
    #[default]
    Nearest,
    Linear,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextureConfig {
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: MipmapFilterMode,
}

impl Default for TextureConfig {
    fn default() -> Self {
        Self {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: MipmapFilterMode::Nearest,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Texture {
    label: String,
    cord_mapping: Vec<[f32; 2]>,
    config: TextureConfig,
}

impl Texture {
    #[must_use]
    pub fn new(label: &str, cord_mapping: Vec<[f32; 2]>) -> Self {
        Self {
            label: label.to_string(),
            cord_mapping,
            config: TextureConfig::default(),
        }
    }

    #[must_use]
    pub fn with_config(mut self, config: TextureConfig) -> Self {
        self.config = config;
        self
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub fn cord_mapping(&self) -> &[[f32; 2]] {
        &self.cord_mapping
    }

    #[must_use]
    pub fn get_config(&self) -> &TextureConfig {
        &self.config
    }
}
