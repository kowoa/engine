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
