mod enging;
use enging::{errors::*, app::*, vector2d::Vec2d};
use sdl2::{event::*, pixels::Color};

struct Application {
    pos: Vec2d
}

impl enging::app::State for Application {
    fn update(&mut self, component: &mut AppComponent) -> Result<bool, enging::errors::GameError> {
        for event in component.event_pump.poll_iter() {
            match event {
                Event::Quit{..} => { return Ok(false) },
                Event::MouseMotion { 
                    x, 
                    y,
                    .. 
                } => {
                    self.pos.x = x as f32;
                    self.pos.y = y as f32;
                },
                _ => {}
            }
        }
        return Ok(true);
    }

    fn init(&mut self, _: &mut AppComponent) -> Result<(), GameError> {
        return Ok(());
    }

    fn draw(&mut self, component: &mut enging::app::AppComponent) -> Result<(), GameError> {
        component.canvas.set_draw_color(Color::RGB(0, 0, 0));
        component.canvas.clear();

        component.canvas.set_draw_color(Color::GREEN);
        component.canvas.draw_line((0, 0), (self.pos.x as i32, self.pos.y as i32)).map_err(sdl_error_to_game_error)?;
        component.canvas.draw_line((
            component.canvas.window().size().0 as i32, 0), 
            (self.pos.x as i32, self.pos.y as i32)).map_err(sdl_error_to_game_error)?;
        component.canvas.draw_line((
            component.canvas.window().size().0 as i32, component.canvas.window().size().1 as i32), 
            (self.pos.x as i32, self.pos.y as i32)).map_err(sdl_error_to_game_error)?;
        component.canvas.draw_line((
            0, component.canvas.window().size().1 as i32), 
            (self.pos.x as i32, self.pos.y as i32)).map_err(sdl_error_to_game_error)?;

        component.canvas.present();
        return Ok(());
    }
}

fn main() -> Result<(), GameError> {
    let component = AppComponent::new()?
        .accelerated(true) 
        .target_texture(true)
        .centered(true)
        .set_title(String::from("Title"))
        .resizable(true)
        .build()?;
    let mut app = App::new(Box::new(Application{pos: Vec2d::new(0.0, 0.0)}), component);

    return Runner::run(&mut app);
}
