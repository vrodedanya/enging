mod enging;

use rand::Rng;
use sdl2::pixels::Color;
use bevy_ecs::prelude::*;
use crate::enging::prelude::*;

struct Application{
}

impl enging::app::State for Application{
    fn update(&mut self, context: &mut AppContext) -> Result<(), GameError> {
        let time = context.world.get_resource::<crate::enging::resources::time::Time>().unwrap();
        println!("FPS: {} Since begin: {}", time.fps, time.time_from_begin.as_secs());
        return Ok(());
    }

    fn init(&mut self, context: &mut AppContext) -> Result<(), GameError> {

        let mut rnd = rand::thread_rng();
        for _ in 0..40000 {
            context.world.spawn((
                rendering::Position{position: Vec2d { x: rnd.gen_range(-100.0..2148.0), y:  rnd.gen_range(-100.0..2148.0)}},
                rendering::Renderable::Point(rendering::DrawablePoint{color: Color::RGB(100 + rnd.gen_range(50..150), rnd.gen_range(0..30), rnd.gen_range(0..30))}),
                movable::HasTarget{target: Vec2d { x: rnd.gen_range(0.0..2048.0), y: rnd.gen_range(0.0..1080.0) }}
            ));
        }

        #[derive(StageLabel)]
        pub struct Rendering;
        #[derive(StageLabel)]
        pub struct EventHandling;
        #[derive(StageLabel)]
        pub struct Moving;

        context.schedule.add_stage(EventHandling, SystemStage::single_threaded().with_system(enging::ecs::event_handling::handle_events));
        context.schedule.add_stage(Moving, SystemStage::parallel().with_system(enging::ecs::movable::move_particles));
        context.schedule.add_stage(Rendering, SystemStage::single_threaded().with_system(enging::ecs::rendering::render));

        return Ok(());
    }
}

fn main() -> Result<(), GameError> {
    let mut app = App::new(Box::new(Application
        {}),
        AppContext::new()?
        .accelerated(true) 
        .target_texture(true)
        .centered(true)
        .set_title(String::from("Title"))
        .fullscreen(true)
        .build()?);

    return Runner::run(&mut app);
}
