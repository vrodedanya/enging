pub fn sdl_error_to_game_error (error_message: String) -> GameError {
    return GameError::SdlError(error_message.clone());
}


#[derive(Debug, Clone)]
pub enum GameError {
    SdlError(String),
}