use std::collections::HashSet;

use bevy_ecs::system::Resource;
use glam::Vec2;
use winit::event::{WindowEvent, VirtualKeyCode, ElementState, MouseScrollDelta};

use crate::ecs::Ecs;

#[derive(Resource)]
pub struct InputStates {
    first_mouse: bool,
    curr_mouse_pos: Vec2,
    keyholds: HashSet<VirtualKeyCode>,
}

pub struct InputEvent(pub Input);

pub struct Input {
    mouse_pos: Option<Vec2>,
    prev_mouse_pos: Option<Vec2>,
    mouse_scroll_delta: f32,
    keydowns: Option<HashSet<VirtualKeyCode>>,
    keyups: Option<HashSet<VirtualKeyCode>>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            mouse_pos: None,
            prev_mouse_pos: None,
            mouse_scroll_delta: 0.0,
            keydowns: None,
            keyups: None,
        }
    }
}

/// Redirect the window's input events into the Input resource
pub fn process_input_event(
    event: &WindowEvent,
    ecs: &mut Ecs,
    input_res: &mut Input,
    input_changed: &mut bool,
) {
    match event {
        WindowEvent::CursorMoved { position, .. } => {
            *input_changed = true;
            let mut states = ecs.get_resource_mut::<InputStates>().unwrap();
            let pos = Vec2::new(position.x as f32, position.y as f32);
            
            if states.first_mouse {
                states.first_mouse = false;
                states.curr_mouse_pos = pos;
            }
            
            input_res.mouse_pos = Some(pos);
            input_res.prev_mouse_pos = Some(states.curr_mouse_pos);
            states.curr_mouse_pos = pos;
        },
        WindowEvent::MouseWheel { delta, .. } => {
            *input_changed = true;
            match delta {
                MouseScrollDelta::LineDelta(_, y) => input_res.mouse_scroll_delta = *y,
                MouseScrollDelta::PixelDelta(pos) => input_res.mouse_scroll_delta = pos.y as f32,
            }
        },
        WindowEvent::CursorEntered { .. } => {
            *input_changed = true;
            let mut states = ecs.get_resource_mut::<InputStates>().unwrap();
            states.first_mouse = true;
        },
        WindowEvent::KeyboardInput { input, .. } => {
            if let Some(key) = input.virtual_keycode {
                *input_changed = true;
                match input.state {
                    ElementState::Pressed => {
                        input_res.keydowns.get_or_insert(HashSet::new());
                        input_res.keydowns.as_mut().unwrap().insert(key);
                        let mut states = ecs.get_resource_mut::<InputStates>().unwrap();
                        states.keyholds.insert(key);
                    },
                    ElementState::Released => {
                        input_res.keyups.get_or_insert(HashSet::new());
                        input_res.keyups.as_mut().unwrap().insert(key);
                        let mut states = ecs.get_resource_mut::<InputStates>().unwrap();
                        assert!(states.keyholds.remove(&key));
                    },
                }
            }
        }
        _ => ()
    }
}