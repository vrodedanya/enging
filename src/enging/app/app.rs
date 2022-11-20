use crate::enging::errors::*;

pub struct App {
    pub component: super::AppComponent,
    state: Box<dyn super::State>,
}

impl App {
    pub fn new(state: Box<dyn super::State>, component: super::AppComponent) -> App {
        return App {
            component,
            state
        };
    }
    pub fn init(&mut self) -> Result<(), GameError> {
        self.state.init(&mut self.component)?;
        return Ok(());
    }
    pub fn update(&mut self) -> Result<bool, GameError> {
        let start = std::time::Instant::now();
        let result = self.state.update(&mut self.component);
        self.state.draw(&mut self.component)?;
        self.component.dt = start.elapsed();
        return result;
    }
}