use bevy_ecs::prelude::*;

#[derive(Resource, Default)]
pub struct Time {
    pub dt: std::time::Duration,
    pub fps: f32,
    pub time_from_begin: std::time::Duration,
}