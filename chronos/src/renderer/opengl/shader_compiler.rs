use crate::renderer::{RendererError, Result, ShaderId};
use glow::HasContext;

pub fn compile(gl: &glow::Context, vertex_src: &str, fragment_src: &str) -> Result<ShaderId> {
    let vertex_shader_id = compile_shader(gl, vertex_src, glow::VERTEX_SHADER)?;
    let fragment_shader_id = compile_shader(gl, fragment_src, glow::FRAGMENT_SHADER)?;
    let shader_program_id = create_program(gl);
    link_program(gl, shader_program_id, vertex_shader_id, fragment_shader_id)?;
    delete_shader(gl, vertex_shader_id);
    delete_shader(gl, fragment_shader_id);
    Ok(ShaderId::OpenGL(shader_program_id))
}

fn compile_shader(gl: &glow::Context, shader_src: &str, shader_type: u32) -> Result<glow::Shader> {
    unsafe {
        let shader = gl
            .create_shader(shader_type)
            .map_err(RendererError::Compilation)?;

        gl.shader_source(shader, shader_src);
        gl.compile_shader(shader);

        check_compile_status(gl, shader)
    }
}

fn check_compile_status(gl: &glow::Context, shader: glow::Shader) -> Result<glow::Shader> {
    unsafe {
        if gl.get_shader_compile_status(shader) {
            Ok(shader)
        } else {
            let info_log = gl.get_shader_info_log(shader);
            Err(RendererError::Compilation(info_log))
        }
    }
}

fn create_program(gl: &glow::Context) -> glow::Program {
    unsafe { gl.create_program().expect("Failed to create program") }
}

fn link_program(
    gl: &glow::Context,
    program: glow::Program,
    vertex: glow::Shader,
    fragment: glow::Shader,
) -> Result<()> {
    unsafe {
        gl.attach_shader(program, vertex);
        gl.attach_shader(program, fragment);
        gl.link_program(program);
        check_link_status(gl, program)
    }
}

fn check_link_status(gl: &glow::Context, program: glow::Program) -> Result<()> {
    unsafe {
        if gl.get_program_link_status(program) {
            Ok(())
        } else {
            let info_log = gl.get_program_info_log(program);
            Err(RendererError::Link(info_log))
        }
    }
}

fn delete_shader(gl: &glow::Context, shader: glow::Shader) {
    unsafe {
        gl.delete_shader(shader);
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::compile;
    use crate::test_utils::get_opengl_api;

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
    #[serial]
    fn test_compile_shader_no_error() {
        let opengl = get_opengl_api();

        let res = compile(&opengl.gl, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);

        assert!(res.is_ok());
    }

    #[test]
    #[serial]
    #[ignore]
    fn test_compile_shader_error() {
        let opengl = get_opengl_api();

        let res = compile(
            &opengl.gl,
            VERTEX_SHADER_SRC,
            "Some bad fragment shader code",
        );

        assert!(res.is_err());
    }
}
