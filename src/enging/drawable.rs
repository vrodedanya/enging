use sdl2::{render::Canvas, video::Window};

pub trait Draw {
    fn draw(&self, canvas: Canvas<Window>) -> Result<(), super::errors::GameError> ;
}