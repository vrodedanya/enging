pub struct Runner {
}

impl Runner {
    pub fn run(app: &mut App) -> Result<(), super::errors::GameError> {
        let mut is_running = true;

        while is_running {
            let start = std::time::Instant::now();
            is_running = app.update()?;
            let component = &mut app.get_component();
            component.draw()?;
            component.dt = start.elapsed();
        }
        return Ok(());
    }
}

pub struct AppComponent {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,

    pub event_pump: sdl2::EventPump,
    pub dt: std::time::Duration,

}

pub struct AppBuilder {
    title: String,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    is_centered: bool,
    is_fullscreen: bool,
    is_borderless: bool,
    is_resizable: bool,
    is_accelerated: bool,
    is_target_texture: bool,
}

impl AppBuilder {
    fn empty() -> AppBuilder {
        AppBuilder {
            title: String::from(""),
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            is_centered: false,
            is_fullscreen: false,
            is_borderless: false,
            is_resizable: false,
            is_accelerated: false,
            is_target_texture: false,
        }
    }
    pub fn set_title(mut self, title: String) -> AppBuilder {
        self.title = title;
        return self;
    }
    pub fn set_size(mut self, width: u32, height: u32) -> AppBuilder {
        self.width = width;
        self.height = height;
        return self;
    }
    pub fn set_position(mut self, x: i32, y: i32) -> AppBuilder {
        self.x = x;
        self.y = y;
        return self;
    }
    pub fn centered(mut self, flag: bool) -> AppBuilder {
        self.is_centered = flag;
        return self;
    }
    pub fn resizable(mut self, flag: bool) -> AppBuilder {
        self.is_resizable = flag;
        return self;
    }
    pub fn fullscreen(mut self, flag: bool) -> AppBuilder {
        self.is_fullscreen = flag;
        return self;
    }
    pub fn borderless(mut self, flag: bool) -> AppBuilder {
        self.is_borderless = flag;
        return self;
    }
    pub fn accelerated(mut self, flag: bool) -> AppBuilder {
        self.is_accelerated = flag;
        return self;
    }
    pub fn target_texture(mut self, flag: bool) -> AppBuilder {
        self.is_target_texture = flag;
        return self;
    }

    pub fn build(&mut self) -> Result<AppComponent, super::errors::GameError> {
        use super::errors::sdl_error_to_game_error;
        let sdl_context = sdl2::init().map_err(sdl_error_to_game_error)?;
        let video_subsystem = sdl_context.video().map_err(sdl_error_to_game_error)?;
        if self.width == 0 || self.height == 0 {
            let display = video_subsystem.current_display_mode(0).map_err(sdl_error_to_game_error)?;
            self.width = display.w as u32;
            self.height = display.h as u32;
        }
        let mut window = video_subsystem.window(&self.title, self.width, self.height);
        if self.is_borderless {
            window.borderless();
        }
        if self.is_centered {
            window.position_centered();
        } else {
            window.position(self.x, self.y);
        }
        if self.is_fullscreen {
            window.fullscreen();
        }
        if self.is_resizable {
            window.resizable();
        }

        let window = match window.build() {
            Ok(window) => window,
            Err(error) => return Err(super::errors::GameError::SdlError(error.to_string()))
        };
        let mut canvas = window.into_canvas();
        if self.is_accelerated {
            canvas = canvas.accelerated();
        }
        if self.is_target_texture {
            canvas = canvas.target_texture();
        }
            
        let canvas = match canvas.build() {
            Ok(canvas) => canvas,
            Err(error) => return Err(super::errors::GameError::SdlError(error.to_string())),
        };

        let event_pump = sdl_context.event_pump().map_err(sdl_error_to_game_error)?;
        let component = AppComponent {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            dt: std::time::Duration::new(0 ,0)
        };
        return Ok(component);
    }
}

impl AppComponent {
    pub fn new() -> Result<AppBuilder, super::errors::GameError> {
        return Ok(AppBuilder::empty());
    }

    fn draw(&self) -> Result<(), super::errors::GameError> {
        return Ok(());
    }
}

pub struct App {
    pub component: AppComponent,
    state: Box<dyn State>,
}

impl App {
    pub fn new(state: Box<dyn State>, component: AppComponent) -> App {
        return App {
            component,
            state
        };
    }
    fn update(&mut self) -> Result<bool, super::errors::GameError> {
        return self.state.update(&mut self.component);
    }

    fn get_component(&mut self) -> &mut AppComponent {
        return &mut self.component;
    }
}

pub trait State {
    fn update(&self, component: &mut AppComponent) -> Result<bool, super::errors::GameError>;
}