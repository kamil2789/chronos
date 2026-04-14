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
pub struct TextureSamplerConfig {
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: MipmapFilterMode,
}

impl Default for TextureSamplerConfig {
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
pub struct TextureComponent {
    label: String,
    texture_mapping: Vec<[f32; 2]>,
    sampler_config: TextureSamplerConfig,
}

impl TextureComponent {
    #[must_use]
    pub fn new(label: &str, texture_mapping: Vec<[f32; 2]>) -> Self {
        Self {
            label: label.to_string(),
            texture_mapping,
            sampler_config: TextureSamplerConfig::default(),
        }
    }

    #[must_use]
    pub fn with_sampler_config(mut self, config: TextureSamplerConfig) -> Self {
        self.sampler_config = config;
        self
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub fn texture_mapping(&self) -> &[[f32; 2]] {
        &self.texture_mapping
    }

    #[must_use]
    pub fn sampler_config(&self) -> &TextureSamplerConfig {
        &self.sampler_config
    }
}
