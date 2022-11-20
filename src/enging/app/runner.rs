use crate::enging::errors::*;

pub struct Runner {
}

impl Runner {
    pub fn run(app: &mut super::App) -> Result<(), GameError> {
        app.init()?;

        while app.update()? {}
        return Ok(());
    }
}