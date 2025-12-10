use crate::renderer::RendererError as Error;
use crate::renderer::Result;
use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ShaderSource {
    vertex_shader: String,
    fragment_shader: String,
}

#[derive(Default)]
pub struct ShaderSourceManager {
    shaders: HashSet<Rc<ShaderSource>>,
}

impl ShaderSourceManager {
    pub fn register_from_str(
        &mut self,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Rc<ShaderSource> {
        let shader = Rc::new(ShaderSource::new(vertex_shader, fragment_shader));
        self.register(shader)
    }

    pub fn register_from_source(&mut self, shader_src: &ShaderSource) -> Rc<ShaderSource> {
        let shader = Rc::new(shader_src.clone());
        self.register(shader)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.shaders.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shaders.is_empty()
    }

    fn register(&mut self, shader: Rc<ShaderSource>) -> Rc<ShaderSource> {
        if let Some(value) = self.shaders.get(&shader) {
            value.clone()
        } else {
            self.shaders.insert(shader.clone());
            shader
        }
    }
}

/// # Errors
///
/// Will return `Err` if the file could not be opened (I/O error) or if the file contains invalid UTF-8 encoding.
pub fn read_src_from_file(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .map_err(|_| Error::ShaderSourceFileError(path.display().to_string()))
}

impl ShaderSource {
    #[must_use]
    pub fn new(vertex: &str, fragment: &str) -> Self {
        ShaderSource {
            vertex_shader: vertex.to_string(),
            fragment_shader: fragment.to_string(),
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if `filename` does not exist or the user does not have
    /// permission to read it.
    pub fn new_from_file(vertex: &Path, fragment: &Path) -> Result<Self> {
        let vertex_shader = read_src_from_file(vertex)?;
        let fragment_shader = read_src_from_file(fragment)?;
        Ok(ShaderSource {
            vertex_shader,
            fragment_shader,
        })
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
    use std::fs::{self, OpenOptions};
    use std::io::Write;

    fn create_file(src: &str, file_name: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)
            .unwrap();
        let write_result = writeln!(file, "{}", src);
        assert!(write_result.is_ok());
    }

    #[test]
    fn test_shader_src_new_from_non_existing_vertex_file_get_error() {
        let vertex_src = "vertex shader source code";
        let fragment_src = "fragment shader source code";
        let vertex_file = "vertex_shader_test1.frag";
        let fragment_file = "fragment_shader_test1.frag";

        create_file(vertex_src, vertex_file);
        create_file(fragment_src, fragment_file);

        let shader =
            ShaderSource::new_from_file(Path::new("/nonExistentPath"), Path::new(fragment_file));
        assert!(shader.is_err());

        if let Err(error) = shader {
            assert!(
                error
                    .to_string()
                    .contains("File could not be opened, path: /nonExistentPath")
            );
        }

        let shader =
            ShaderSource::new_from_file(Path::new(vertex_file), Path::new("/nonExistentPath"));
        assert!(shader.is_err());

        assert!(fs::remove_file(vertex_file).is_ok());
        assert!(fs::remove_file(fragment_file).is_ok());
    }

    #[test]
    fn test_shader_src_new_from_file() {
        let vertex_src = "vertex shader source code";
        let fragment_src = "fragment shader source code";
        let vertex_file = "vertex_shader_test2.frag";
        let fragment_file = "fragment_shader_test2.frag";

        create_file(vertex_src, vertex_file);
        create_file(fragment_src, fragment_file);

        let shader =
            ShaderSource::new_from_file(Path::new(vertex_file), Path::new(fragment_file)).unwrap();

        assert_eq!(shader.get_vertex_shader().trim(), vertex_src);
        assert_eq!(shader.get_fragment_shader().trim(), fragment_src);

        assert!(fs::remove_file(vertex_file).is_ok());
        assert!(fs::remove_file(fragment_file).is_ok());
    }

    #[test]
    fn test_shader_base() {
        let vertex_src = "vertex shader source code";
        let fragment_src = "fragment shader source code";

        let mut base = ShaderSourceManager::default();
        assert!(base.is_empty());

        let shader = base.register_from_str(vertex_src, fragment_src);

        assert_eq!(shader.get_vertex_shader(), vertex_src);
        assert_eq!(shader.get_fragment_shader(), fragment_src);
        assert_eq!(base.len(), 1);

        //same shader should be returned
        let shader_src = ShaderSource::new(vertex_src, fragment_src);
        let shader_two = base.register_from_source(&shader_src);
        assert_eq!(shader_two.get_vertex_shader(), vertex_src);
        assert_eq!(shader_two.get_fragment_shader(), fragment_src);
        assert_eq!(base.len(), 1);

        //Strong count should be 3. One for the base, one for the first shader and one for the second shader
        assert_eq!(Rc::strong_count(&shader_two), 3);
    }
}
