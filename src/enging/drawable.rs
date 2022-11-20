use sdl2::{render::Canvas, video::Window, pixels::Color};
use super::errors::*;

pub struct Colored<'a, TYPE: Drawable> {
    pub t: &'a TYPE,
    pub color: Color,
}

impl<'a, TYPE: Drawable> Colored<'a, TYPE> {
    pub fn new(t: &'a TYPE, color: Color) -> Self{
        Self {
            t,
            color
        }
    }
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), GameError>;
    fn draw_colored(&self, canvas: &mut Canvas<Window>, color: Color) -> Result<(), GameError> {
        canvas.set_draw_color(color);
        Ok(self.draw(canvas)?)
    }
}

impl<'a, TYPE: Drawable> Drawable for Colored<'a, TYPE> {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), GameError> {
        Ok(self.t.draw_colored(canvas, self.color)?)
    }
}

impl Drawable for sdl2::rect::Rect {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), GameError>  {
        canvas.fill_rect(*self).map_err(sdl_error_to_game_error)?;
        return Ok(());
    }
}

impl Drawable for sdl2::rect::Point {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), GameError>  {
        canvas.draw_point(*self).map_err(sdl_error_to_game_error)?;
        return Ok(());
    }
}

impl Drawable for super::vector2d::Vec2d {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), GameError>  {
        canvas.draw_point(sdl2::rect::Point::new(self.x as i32, self.y as i32)).map_err(sdl_error_to_game_error)?;
        return Ok(());
    }
}