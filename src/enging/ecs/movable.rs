use bevy_ecs::prelude::*;

use crate::enging::{app::AppComponent, vector2d::Vec2d};

pub fn move_particles(component: NonSend<AppComponent>, mut query: Query<&mut super::rendering::Position>) {
    let center = ((component.canvas.window().size().0 / 2) as f32, (component.canvas.window().size().1 / 2) as f32);
    for mut pos in query.iter_mut() {
        let self_angle = pos.angle;
        pos.position.rotate_with_moved_center( self_angle * component.dt.as_secs_f32(), (component.event_pump.mouse_state().x() as f32, 
        component.event_pump.mouse_state().y() as f32));
        if component.event_pump.mouse_state().left() {
            let mouse_point = Vec2d::new(component.event_pump.mouse_state().x() as f32, component.event_pump.mouse_state().y() as f32);
            let distance = pos.position.distance_to(&mouse_point);
            let prev = pos.position;
            pos.position += (mouse_point - prev) / distance * 0.1;
        }
        if component.event_pump.mouse_state().right() {
            let mouse_point = Vec2d::new(component.event_pump.mouse_state().x() as f32, component.event_pump.mouse_state().y() as f32);
            let distance = pos.position.distance_to(&mouse_point);
            let prev = pos.position;
            pos.position -= (mouse_point - prev) / distance * 0.1;
        }
    }
}