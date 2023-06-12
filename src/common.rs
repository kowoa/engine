use std::time::{Duration, Instant};

use bevy_ecs::{system::Resource, world::World};


#[derive(Resource)]
pub struct Time {
    pub current: f32,
    pub delta: f32,
}

pub fn update_time_res(start_time: Instant, world: &mut World) {
    let mut time_res = world.get_resource_mut::<Time>().unwrap();
    let new_time = Instant::now() - start_time;
    time_res.delta = new_time.as_secs_f32() - time_res.current;
    time_res.current = new_time.as_secs_f32();
}