mod enging;

use enging::{errors::*, app::*, vector2d::Vec2d};
use rand::Rng;
use sdl2::{event::*, pixels::Color, rect::Rect, sys::Window};
use bevy_ecs::prelude::*;

struct Application{
    world: World,
    schedule: Schedule
}

impl enging::app::State for Application{
    fn update(&mut self) -> Result<bool, enging::errors::GameError> {
        let start = std::time::Instant::now();
        self.schedule.run(&mut self.world);
        let mut component = self.world.get_non_send_resource_mut::<AppComponent>().unwrap();
        component.dt = start.elapsed();
        return Ok(component.is_running);
    }

    fn init(&mut self) -> Result<(), GameError> {
        self.world.insert_non_send_resource(AppComponent::new()?
        .accelerated(true) 
        .target_texture(true)
        .centered(true)
        .set_title(String::from("Title"))
        .fullscreen(true)
        .build()?);
        use enging::ecs::rendering::*;
        let mut rnd = rand::thread_rng();
        for _ in 0..4000 {
            self.world.spawn((
                Position{position: Vec2d { x: rnd.gen_range(-100.0..2148.0), y:  rnd.gen_range(-100.0..2148.0)}, angle: rnd.gen_range(0.1..1.8)},
                Renderable::Point(DrawablePoint{color: Color::RGB(100 + rnd.gen_range(50..150), rnd.gen_range(0..30), rnd.gen_range(0..30))})
            ));
        }

        #[derive(StageLabel)]
        pub struct Rendering;
        #[derive(StageLabel)]
        pub struct EventHandling;
        #[derive(StageLabel)]
        pub struct Moving;

        self.schedule.add_stage(EventHandling, SystemStage::single_threaded().with_system(enging::ecs::event_handling::handle_events));
        self.schedule.add_stage(Moving, SystemStage::parallel().with_system(enging::ecs::movable::move_particles));
        self.schedule.add_stage(Rendering, SystemStage::single_threaded().with_system(enging::ecs::rendering::render));

        return Ok(());
    }
}

fn main() -> Result<(), GameError> {
    let mut app = App::new(Box::new(Application
        {
            world: World::default(),
            schedule: Schedule::default()
        }));

    return Runner::run(&mut app);
}
