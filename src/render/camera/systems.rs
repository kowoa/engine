use bevy_ecs::{system::{Commands, Query, Res}, prelude::EventReader};
use glam::Vec3;
use winit::event::VirtualKeyCode;

use crate::{input::{InputEvent, InputStates}, common::Time};

use super::{CameraBundle, Camera, CameraMovement};


pub fn spawn(mut commands: Commands) {
    commands.spawn(CameraBundle {
        camera: Camera::from_position(0.0, 0.0, 3.0),
        ..Default::default()
    });
}

pub fn process_input(
    mut cam_qry: Query<(&mut Camera, &mut CameraMovement)>,
    mut input_rdr: EventReader<InputEvent>,
    time: Res<Time>,
    states: Res<InputStates>,
) {
    let (mut cam, mut cam_move) = cam_qry.single_mut();
    
    for evt in input_rdr.iter() {
        let input = &evt.0;
        
        // zoom
        if input.mouse_scroll_delta != 0.0 {
            cam.process_mouse_scroll(input.mouse_scroll_delta);
        }
        // turning
        /*
        if let (Some(mouse_pos), Some(prev_mouse_pos)) = (input.mouse_pos, input.prev_mouse_pos) {
            cam.process_mouse_movement(mouse_pos, prev_mouse_pos, &cam_move, &time);
        }
        */
    }
}

pub fn process_movement_input(
    mut cam_qry: Query<(&mut Camera, &CameraMovement)>,
    time: Res<Time>,
    states: Res<InputStates>,
) {
    let (mut cam, movement) = cam_qry.single_mut();
    
    let mut local_move_dir = Vec3::ZERO;
    let keyholds = &states.keyholds;
    if keyholds.contains(&VirtualKeyCode::W) {
        local_move_dir.z -= 1.0;
    }
    if keyholds.contains(&VirtualKeyCode::S) {
        local_move_dir.z += 1.0;
    }
    if keyholds.contains(&VirtualKeyCode::A) {
        local_move_dir.x -= 1.0;
    }
    if keyholds.contains(&VirtualKeyCode::D) {
        local_move_dir.x += 1.0;
    }
    
    cam.process_movement(local_move_dir, movement, &time);
}

pub fn process_rotation_input(
    mut cam_qry: Query<(&mut Camera, &CameraMovement)>,
    time: Res<Time>,
    states: Res<InputStates>,
) {
    let (mut cam, movement) = cam_qry.single_mut();
    
    let mut pitch = 0.0;
    let mut yaw = 0.0;
    let keyholds = &states.keyholds;
    if keyholds.contains(&VirtualKeyCode::Up) {
        pitch += 1.0;
    }
    if keyholds.contains(&VirtualKeyCode::Down) {
        pitch -= 1.0;
    }
    if keyholds.contains(&VirtualKeyCode::Left) {
        yaw -= 1.0;
    }
    if keyholds.contains(&VirtualKeyCode::Right) {
        yaw += 1.0;
    }
    
    cam.process_rotation(pitch, yaw, movement, &time);
}
