mod enging;
use enging::{errors::*, app::*};
use sdl2::event::*;

struct Application {
}

impl enging::app::State for Application {
    fn update(&self, component: &mut AppComponent) -> Result<bool, enging::errors::GameError> {
        for event in component.event_pump.poll_iter() {
            match event {
                Event::Quit{..} => { return Ok(false) },
                _ => {}
            }
        }
        return Ok(true);
    }
}

fn main() -> Result<(), GameError> {
    let component = AppComponent::new()?
        .accelerated(true) 
        .target_texture(true)
        .centered(true)
        .set_title(String::from("Title"))
        .fullscreen(true)
        .build()?;
    let mut app = App::new(Box::new(Application{}), component);

    return Runner::run(&mut app);
}
