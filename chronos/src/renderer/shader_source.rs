use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ShaderSource {
    vertex_shader: String,
    fragment_shader: String,
}

#[derive(Default)]
pub struct ShaderManager {
    shaders_src: HashMap<String, ShaderSource>,
}

impl ShaderManager {
    #[allow(dead_code)]
    pub fn register_from_str(&mut self, name: &str, vertex_shader: &str, fragment_shader: &str) {
        self.shaders_src.insert(
            name.to_string(),
            ShaderSource::new(vertex_shader, fragment_shader),
        );
    }

    pub fn register_from_source(&mut self, name: &str, shader_src: &ShaderSource) {
        self.shaders_src
            .insert(name.to_string(), shader_src.clone());
    }

    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&ShaderSource> {
        self.shaders_src.get(name)
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.shaders_src.len()
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shaders_src.is_empty()
    }
}

impl ShaderSource {
    #[must_use]
    pub fn new(vertex: &str, fragment: &str) -> Self {
        ShaderSource {
            vertex_shader: vertex.to_string(),
            fragment_shader: fragment.to_string(),
        }
    }

    #[must_use]
    pub fn get_vertex_shader(&self) -> &str {
        &self.vertex_shader
    }

    #[must_use]
    pub fn get_fragment_shader(&self) -> &str {
        &self.fragment_shader
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_manager() {
        let vertex_src = "vertex shader source code";
        let fragment_src = "fragment shader source code";

        let mut manager = ShaderManager::default();
        assert!(manager.is_empty());

        manager.register_from_str("basic", vertex_src, fragment_src);

        assert_eq!(
            manager.get("basic").unwrap().get_vertex_shader(),
            vertex_src
        );
        assert_eq!(
            manager.get("basic").unwrap().get_fragment_shader(),
            fragment_src
        );
        assert_eq!(manager.len(), 1);

        // Same shader should be returned.
        let shader_src = ShaderSource::new(vertex_src, fragment_src);
        manager.register_from_source("basic", &shader_src);
        assert_eq!(
            manager.get("basic").unwrap().get_vertex_shader(),
            vertex_src
        );
        assert_eq!(
            manager.get("basic").unwrap().get_fragment_shader(),
            fragment_src
        );
        assert_eq!(manager.len(), 1);
    }
}
