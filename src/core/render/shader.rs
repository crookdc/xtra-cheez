use glam::Mat4;
use std::ffi::CString;
use std::{fs, io};

#[derive(Clone)]
pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn from_source_files(vertex_path: &str, fragment_path: &str) -> io::Result<Shader> {
        unsafe {
            let vertex_shader = compile_shader_from_file(gl::VERTEX_SHADER, &vertex_path)?;
            let fragment_shader = compile_shader_from_file(gl::FRAGMENT_SHADER, &fragment_path)?;
            let id = create_program(vertex_shader, fragment_shader)?;
            gl::DeleteProgram(vertex_shader);
            gl::DeleteProgram(fragment_shader);
            Ok(Self { id })
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_bool(&self, name: &str, value: bool) {
        gl::Uniform1i(self.get_uniform_location(name), value as i32);
    }

    pub unsafe fn set_int(&self, name: &str, value: i32) {
        gl::Uniform1i(self.get_uniform_location(name), value);
    }

    pub unsafe fn set_f32(&self, name: &str, value: f32) {
        gl::Uniform1f(self.get_uniform_location(name), value);
    }

    pub unsafe fn set_mat4(&self, name: &str, value: &Mat4) {
        gl::UniformMatrix4fv(
            self.get_uniform_location(name),
            1,
            gl::FALSE,
            &value.to_cols_array()[0],
        )
    }

    unsafe fn get_uniform_location(&self, name: &str) -> i32 {
        let c_str = CString::new(name).unwrap();
        gl::GetUniformLocation(self.id, c_str.as_ptr())
    }
}

pub unsafe fn set_mat4(id: u32, name: &str, value: &Mat4) {
    gl::UniformMatrix4fv(
        get_uniform_location(id, name),
        1,
        gl::FALSE,
        &value.to_cols_array()[0],
    )
}

pub unsafe fn set_int(id: u32, name: &str, value: i32) {
    gl::Uniform1i(get_uniform_location(id, name), value)
}

pub unsafe fn get_uniform_location(id: u32, name: &str) -> i32 {
    let c_str = CString::new(name).unwrap();
    gl::GetUniformLocation(id, c_str.as_ptr())
}

unsafe fn create_program(vertex_shader_id: u32, fragment_shader_id: u32) -> io::Result<u32> {
    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader_id);
    gl::AttachShader(shader_program, fragment_shader_id);
    gl::LinkProgram(shader_program);

    let mut success = 0;
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
        v.set_len(log_len.try_into().unwrap());
        return Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&v).parse::<String>().unwrap(),
        ));
    }
    Ok(shader_program)
}

unsafe fn compile_shader_from_file(
    shader_type: gl::types::GLenum,
    source_file: &str,
) -> io::Result<gl::types::GLuint> {
    let fragment_src = fs::read_to_string(source_file)?;
    compile_shader(shader_type, &fragment_src)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

unsafe fn compile_shader(
    shader_type: gl::types::GLenum,
    src: &str,
) -> Result<gl::types::GLuint, String> {
    let shader = gl::CreateShader(shader_type);
    gl::ShaderSource(
        shader,
        1,
        &(src.as_bytes().as_ptr().cast()),
        &(src.len().try_into().unwrap()),
    );
    gl::CompileShader(shader);

    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl::GetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
        v.set_len(log_len.try_into().unwrap());
        return Err(String::from_utf8_lossy(&v).parse().unwrap());
    }
    Ok(shader)
}
