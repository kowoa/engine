use specs::System;

use crate::renderer::camera::{CameraBundle, Camera};

pub struct SpawnCameraSys;

impl<'a> System<'a> for SpawnCameraSys {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {
        println!("system running");
    }
}
