use bevy_ecs::system::Resource;


#[derive(Resource)]
pub struct Time {
    pub current: f32,
    pub delta: f32,
}