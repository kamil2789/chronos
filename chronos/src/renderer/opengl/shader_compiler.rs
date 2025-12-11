use crate::renderer::{RendererError, Result, ShaderId};
use std::ffi::CString;
use std::ptr;

pub fn compile(vertex_src: &str, fragment_src: &str) -> Result<ShaderId> {
    let vertex_shader_id = compile_shader(vertex_src, gl::VERTEX_SHADER)?;
    let fragment_shader_id = compile_shader(fragment_src, gl::FRAGMENT_SHADER)?;
    let shader_program_id = create_program();
    link_program(shader_program_id, vertex_shader_id, fragment_shader_id)?;
    delete_shader(vertex_shader_id);
    delete_shader(fragment_shader_id);
    Ok(shader_program_id)
}

fn compile_shader(shader_src: &str, shader_type: u32) -> Result<u32> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str_vert = CString::new(shader_src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        check_compile_status(shader)
    }
}

fn check_compile_status(shader_id: u32) -> Result<u32> {
    let mut status = i32::from(gl::TRUE);
    let info_length: usize = 512;
    let mut info_log: Vec<u8> = vec![0; info_length];
    unsafe { gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status) };
    if status == i32::from(gl::TRUE) {
        Ok(shader_id)
    } else {
        unsafe {
            gl::GetShaderInfoLog(
                shader_id,
                info_length.try_into().unwrap(),
                ptr::null_mut(),
                info_log.as_mut_ptr().cast::<i8>(),
            );
        };
        Err(RendererError::CompilationError(String::from(
            std::str::from_utf8(&info_log)
                .unwrap()
                .trim_end_matches('\0'),
        )))
    }
}

fn create_program() -> u32 {
    unsafe { gl::CreateProgram() }
}

fn link_program(shader_id: u32, vertex_id: u32, fragment_id: u32) -> Result<()> {
    unsafe {
        gl::AttachShader(shader_id, vertex_id);
        gl::AttachShader(shader_id, fragment_id);
        gl::LinkProgram(shader_id);
        check_link_status(shader_id)
    }
}

fn check_link_status(shader_id: u32) -> Result<()> {
    let mut status = i32::from(gl::TRUE);
    let info_length: usize = 512;
    let mut info_log: Vec<u8> = vec![0; info_length];
    unsafe { gl::GetProgramiv(shader_id, gl::LINK_STATUS, &mut status) };
    if status == i32::from(gl::TRUE) {
        Ok(())
    } else {
        unsafe {
            gl::GetProgramInfoLog(
                shader_id,
                info_length.try_into().unwrap(),
                ptr::null_mut(),
                info_log.as_mut_ptr().cast::<i8>(),
            );
        };
        Err(RendererError::LinkError(String::from(
            std::str::from_utf8(&info_log)
                .unwrap()
                .trim_end_matches('\0'),
        )))
    }
}

fn delete_shader(shader_id: u32) {
    unsafe {
        gl::DeleteShader(shader_id);
    }
}


#[cfg(test)]
mod tests {
    use super::compile;

    const VERTEX_SHADER_SRC: &str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        void main() {
            gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
        }
    "#;

    const FRAGMENT_SHADER_SRC: &str = r#"
        #version 330 core
        out vec4 FragColor;
        void main() {
            FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
        }
    "#;

    #[test]
    fn test_compile_shader_no_error() {

        let shader_id = compile(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);
        assert!(shader_id.is_ok());
    }

    #[test]
    fn test_compile_shader_invalid_shaders_src_get_err() {
        let mut shader_id = compile(VERTEX_SHADER_SRC, "Invalid data");
        assert!(shader_id.is_err());

        shader_id = compile("Invalid data", FRAGMENT_SHADER_SRC);
        assert!(shader_id.is_err());

        shader_id = compile("", "");
        assert!(shader_id.is_err());
    }
}
