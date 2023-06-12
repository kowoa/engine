use std::ffi::CStr;

use bevy_ecs::system::Resource;

use crate::{window::Window, ecs::{Plugin, EcsBuilderState, EcsBuilder}};

use self::{shader::Shader, model::Model};

pub mod camera;
pub mod mesh;
pub mod model;
pub mod shader;
pub mod utils;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build<E>(&self, ecs_builder: EcsBuilder<E>) -> EcsBuilder<E>
        where E: EcsBuilderState {
        todo!()
    }
}

#[derive(Resource)]
pub struct RenderObjs {
    obj_vao: u32,
    light_vao: u32,
    obj_shader: Shader,
    light_shader: Shader,
    num_elems: u32,
    diffuse_map: u32,
    specular_map: u32,
    emission_map: u32,
    model: Model,
}

#[derive(Resource)]
pub struct Renderer {
    program: gl::types::GLuint,
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        unsafe {
            gl::load_with(|symbol| window.get_proc_address(symbol));

            if let Some(renderer) = get_gl_string(gl::RENDERER) {
                println!("Running on {}", renderer.to_string_lossy());
            }
            if let Some(version) = get_gl_string(gl::VERSION) {
                println!("OpenGL Version {}", version.to_string_lossy());
            }

            if let Some(shaders_version) = get_gl_string(gl::SHADING_LANGUAGE_VERSION) {
                println!("Shaders version on {}", shaders_version.to_string_lossy());
            }

            let vertex_shader = create_shader(gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
            let fragment_shader = create_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::LinkProgram(program);

            gl::UseProgram(program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            let mut vao = std::mem::zeroed();
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            let mut vbo = std::mem::zeroed();
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let pos_attrib = gl::GetAttribLocation(program, b"position\0".as_ptr() as *const _);
            let color_attrib = gl::GetAttribLocation(program, b"color\0".as_ptr() as *const _);
            gl::VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl::EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl::EnableVertexAttribArray(color_attrib as gl::types::GLuint);

            Self { program, vao, vbo }
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.program);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::ClearColor(0.1, 0.1, 0.1, 0.9);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }
}

/*
impl Deref for Renderer {
    type Target = gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}
*/

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

unsafe fn create_shader(
    shader: gl::types::GLenum,
    source: &[u8],
) -> gl::types::GLuint {
    let shader = gl::CreateShader(shader);
    gl::ShaderSource(shader, 1, [source.as_ptr().cast()].as_ptr(), std::ptr::null());
    gl::CompileShader(shader);
    shader
}

fn get_gl_string(variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl::GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
];

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";