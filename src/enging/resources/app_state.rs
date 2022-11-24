use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct AppState {
    pub is_running: bool,
    pub window_size: (u32, u32)
}

impl AppState {
    pub fn new(is_running: bool, window_size: (u32, u32)) -> AppState {
        AppState {
            is_running,
            window_size
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self { 
            is_running: true,
            window_size: (0, 0)
        }
    }
}