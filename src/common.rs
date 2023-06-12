use std::time::{Duration, Instant};

use bevy_ecs::system::Resource;

use crate::ecs::Ecs;


#[derive(Resource)]
pub struct Time {
    pub current: Instant,
    pub delta: Duration,
}

pub fn update_time_res(time: Instant, ecs: &mut Ecs) {
    let mut time_res = ecs.get_resource_mut::<Time>().unwrap();
    time_res.delta = time - time_res.current;
    time_res.current = time;
}