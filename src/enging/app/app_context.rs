use crate::enging::errors::*;
use bevy_ecs::prelude::*;

pub struct AppContext {
    pub world: World,
    pub schedule: Schedule
}

impl AppContext {
    pub fn new() -> Result<super::AppBuilder, GameError> {
        return Ok(super::AppBuilder::empty());
    }
}