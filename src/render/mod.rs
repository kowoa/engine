use std::ffi::CStr;

use bevy_ecs::system::{Resource, Commands, Res};

use crate::{window::Window, ecs::{Plugin, EcsBuilderState, EcsBuilder, Incomplete, Render, StartupSingleThreaded}};

use self::{shader::Shader, model::Model, camera::CameraPlugin};

mod camera;
mod mesh;
mod model;
mod shader;
mod utils;
mod systems;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, ecs_builder: EcsBuilder<Incomplete>) -> EcsBuilder<Incomplete> {
        ecs_builder
            .add_plugin(CameraPlugin)
            .add_system(systems::init, StartupSingleThreaded)
            .add_system(systems::draw, Render)
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

pub fn resize(width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
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