use crate::enging::errors::*;

pub struct AppComponent {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,

    pub event_pump: sdl2::EventPump,
    pub dt: std::time::Duration,

}

impl AppComponent {
    pub fn new() -> Result<super::AppBuilder, GameError> {
        return Ok(super::AppBuilder::empty());
    }
}