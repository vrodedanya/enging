use bevy_ecs::schedule::Stage;

use crate::enging::errors::*;

use super::{AppContext, State};

pub struct App {
    state: Box<dyn State>,
    context: AppContext,
    begin_time: std::time::Instant
}

impl App {
    pub fn new(state: Box<dyn State>, context: AppContext) -> App {
        return App {
            state,
            context,
            begin_time: std::time::Instant::now()
        };
    }
    pub fn init(&mut self) -> Result<(), GameError> {
        self.state.init(&mut self.context)?;
        return Ok(());
    }
    pub fn update(&mut self) -> Result<bool, GameError> {
        let start = std::time::Instant::now();
        self.state.update(&mut self.context)?;
        self.context.schedule.run(&mut self.context.world);
        let mut time = self.context.world.get_resource_mut::<crate::enging::utils::time::Time>().unwrap();
        time.dt = start.elapsed();
        time.fps = 1.0 / time.dt.as_secs_f32();
        time.time_from_begin = self.begin_time.elapsed();
        
        return Ok(self.context.world.get_resource::<crate::enging::utils::app_state::AppState>().unwrap().is_running.clone());
    }
}