use bevy_ecs::prelude::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::Vec2d;
use sdl2::{pixels::Color, rect::Rect};

#[derive(Component)]
pub struct Position {
    pub position: Vec2d
}

pub struct DrawablePoint {
    pub color: Color,
}

pub struct DrawableRect {
    pub color: Color,
    pub size: (u32, u32),
}

#[derive(Component)]
pub enum Renderable {
    Point(DrawablePoint),
    Rect(DrawableRect),
}

pub fn render(mut canvas: NonSendMut<Canvas<Window>>, query: Query<(&Position, &Renderable)>) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    for (pos, renderable )in query.iter() {
        match renderable {
            Renderable::Point(point) => {
                canvas.set_draw_color(point.color);
                canvas.draw_point(pos.position).unwrap();
            },
            Renderable::Rect(rect) => {
                canvas.set_draw_color(rect.color);
                canvas.draw_rect(Rect::new(pos.position.x as i32, pos.position.y as i32, rect.size.0, rect.size.1)).unwrap();
            },
        }
    }
    canvas.present();
}