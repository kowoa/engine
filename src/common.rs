use std::time::{Duration, Instant};

use bevy_ecs::{system::Resource, world::World};


#[derive(Resource)]
pub struct Time {
    pub current: Instant,
    pub delta: Duration,
}

pub fn update_time_res(time: Instant, world: &mut World) {
    let mut time_res = world.get_resource_mut::<Time>().unwrap();
    time_res.delta = time - time_res.current;
    time_res.current = time;
}