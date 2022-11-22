use crate::enging::errors::*;

pub trait State {
    fn update(&mut self) -> Result<bool, GameError>;

    fn init(&mut self) -> Result<(), GameError>;
}