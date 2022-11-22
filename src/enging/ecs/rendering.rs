use bevy_ecs::prelude::*;
use crate::{Vec2d, enging::app::AppComponent};
use sdl2::{pixels::Color, rect::Rect};

#[derive(Component)]
pub struct Position {
    pub position: Vec2d,
    pub angle: f32
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

pub fn render(mut component: NonSendMut<AppComponent>, query: Query<(&Position, &Renderable)>) {
    component.canvas.set_draw_color(Color::BLACK);
    component.canvas.clear();
    for (pos, renderable )in query.iter() {
        match renderable {
            Renderable::Point(point) => {
                component.canvas.set_draw_color(point.color);
                component.canvas.draw_point(pos.position).unwrap();
            },
            Renderable::Rect(rect) => {
                component.canvas.set_draw_color(rect.color);
                component.canvas.draw_rect(Rect::new(pos.position.x as i32, pos.position.y as i32, rect.size.0, rect.size.1)).unwrap();
            },
        }
    }
    component.canvas.present();
}