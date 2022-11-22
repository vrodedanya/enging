use crate::enging::errors::*;

pub struct App {
    state: Box<dyn super::State>
}

impl App {
    pub fn new(state: Box<dyn super::State>) -> App {
        return App {
            state
        };
    }
    pub fn init(&mut self) -> Result<(), GameError> {
        self.state.init()?;
        return Ok(());
    }
    pub fn update(&mut self) -> Result<bool, GameError> {
        let result = self.state.update();
        return result;
    }
}