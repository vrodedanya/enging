use crate::enging::errors::*;

pub trait State {
    fn update(&mut self, component: &mut super::AppComponent) -> Result<bool, GameError>;

    fn draw(&mut self, component: &mut super::AppComponent) -> Result<(), GameError>;

    fn init(&mut self, component: &mut super::AppComponent) -> Result<(), GameError>;
}