use bevy::prelude::*;

use crate::renderer::camera::{CameraBundle, Camera};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(CameraBundle {
        camera: Camera::from_position(0.0, 0.0, 3.0),
        ..default()
    });
}

pub fn entity_checker(query: Query<Entity>) {
    println!("{}", query.iter().len());
}