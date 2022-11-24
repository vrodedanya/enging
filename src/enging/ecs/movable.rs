use bevy_ecs::prelude::*;
use sdl2::EventPump;

use crate::enging::vector2d::Vec2d;
use rand::{self, Rng};

#[derive(Component)]
pub struct HasTarget {
    pub target: Vec2d
}

pub fn move_particles(mut particles: Query<(&mut super::rendering::Position, &mut HasTarget)>, time: Res<crate::enging::resources::time::Time>, event_pump: NonSend<EventPump>) {
    for (mut position, mut target) in particles.iter_mut() {
        let position = &mut position.position;
        let distance = position.distance_to(&target.target);
        let prev = position.clone();
        *position += (target.target - prev) / distance * 50.0 * time.dt.as_secs_f32();
        
        let mouse = Vec2d::new(event_pump.mouse_state().x() as f32, event_pump.mouse_state().y() as f32);

        if position.is_in_cirlce(mouse, 35.0) {
            *position = position.get_nearest_point_on_circle(mouse, 35.0);
        }

        if distance < 5.0 || event_pump.mouse_state().left() {
            let mut rnd = rand::thread_rng();
            target.target.x = rnd.gen_range(0.0..2048.0);
            target.target.y = rnd.gen_range(0.0..1080.0);
        }
    }
}