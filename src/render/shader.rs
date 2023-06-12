use std::{ffi::CString, fs::File, io::Read, ptr};

use gl::types::{GLint, GLchar};
use glam::{Mat4, Mat3};

pub struct Shader {
    pub id: u32
}

impl Shader {
    pub fn new(vert_path: &str, frag_path: &str) -> Shader {
        // read vertex shader source code from filesystem
        let mut vert_file = File::open(vert_path)
            .unwrap_or_else(|_| panic!("failed to open {}", vert_path));
        let mut vert_src = String::new();
        vert_file.read_to_string(&mut vert_src)
            .expect("failed to read vertex shader");
        let vert_src = CString::new(vert_src.as_bytes()).unwrap();
        
        // read fragment shader source code from filesystem
        let mut frag_file = File::open(frag_path)
            .unwrap_or_else(|_| panic!("failed to open {}", frag_path));
        let mut frag_src = String::new();
        frag_file.read_to_string(&mut frag_src)
            .expect("failed to read fragment shader");
        let frag_src = CString::new(frag_src.as_bytes()).unwrap();
        
        // compile shaders
        let shader_program = unsafe {
            // vertex shader
            let vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vert_shader, 1, &vert_src.as_ptr(), ptr::null());
            gl::CompileShader(vert_shader);
            Shader::check_compile_errors(vert_shader, "VERTEX");
            
            // fragment shader
            let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(frag_shader, 1, &frag_src.as_ptr(), ptr::null());
            gl::CompileShader(frag_shader);
            Shader::check_compile_errors(frag_shader, "FRAGMENT");
            
            // shader program
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vert_shader);
            gl::AttachShader(shader_program, frag_shader);
            gl::LinkProgram(shader_program);
            Shader::check_link_errors(shader_program);
            
            // cleanup
            gl::DeleteShader(vert_shader);
            gl::DeleteShader(frag_shader);
            
            shader_program
        };
        
        Shader { id: shader_program }
    }

    unsafe fn check_compile_errors(shader: u32, shader_type: &str) {
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut info_log = Vec::with_capacity(512);
            info_log.resize(512 - 1, 0); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!(
                "shader failed to compile: {}\n--------------------\n{}",
                shader_type,
                std::str::from_utf8(&info_log).unwrap()
            );
        }
        
    }
    
    unsafe fn check_link_errors(shader_program: u32) {
        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut info_log = Vec::with_capacity(512);
            info_log.resize(512 - 1, 0);
            gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("shader program failed to link\n--------------------\n{}", std::str::from_utf8(&info_log).unwrap());
        }
    }

    pub unsafe fn activate(&self) {
        gl::UseProgram(self.id);
    }
    
    pub unsafe fn set_bool(&self, name: &str, value: bool) {
        gl::Uniform1i(self.get_uniform_loc(name), value as i32);
    }
    
    pub unsafe fn set_int(&self, name: &str, value: i32) {
        gl::Uniform1i(self.get_uniform_loc(name), value);
    }
    
    pub unsafe fn set_float(&self, name: &str, value: f32) {
        gl::Uniform1f(self.get_uniform_loc(name), value);
    }
    
    pub unsafe fn set_vec4(&self, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        gl::Uniform4f(self.get_uniform_loc(name), v0, v1, v2, v3);
    }

    pub unsafe fn set_vec3(&self, name: &str, v0: f32, v1: f32, v2: f32) {
        gl::Uniform3f(self.get_uniform_loc(name), v0, v1, v2);
    }
    
    pub unsafe fn set_mat4(&self, name: &str, mat: Mat4) {
        gl::UniformMatrix4fv(self.get_uniform_loc(name), 1, gl::FALSE, &mat.to_cols_array()[0]);
    }

    pub unsafe fn set_mat3(&self, name: &str, mat: Mat3) {
        gl::UniformMatrix3fv(self.get_uniform_loc(name), 1, gl::FALSE, &mat.to_cols_array()[0]);
    }
    
    pub unsafe fn get_uniform_loc(&self, name: &str) -> i32 {
        let name = CString::new(name).unwrap();
        gl::GetUniformLocation(
            self.id,
            name.as_ptr()
        )
    }
}