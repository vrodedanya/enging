use crate::enging::errors::*;
use super::AppContext;

pub trait State {
    fn update(&mut self, context: &mut AppContext) -> Result<(), GameError>;

    fn init(&mut self, context: &mut AppContext) -> Result<(), GameError>;
}