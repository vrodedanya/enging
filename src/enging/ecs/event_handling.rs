use bevy_ecs::prelude::*;

use sdl2::EventPump;
use sdl2::event::*;
use sdl2::keyboard::Keycode;
use crate::enging::resources::app_state::AppState;

pub fn handle_events(mut event_pump: NonSendMut<EventPump>, mut state: ResMut<AppState>) {
    for event in (&mut event_pump).poll_iter() {
        match event {
            Event::Quit{..} => {  
                state.is_running = false;
                break;
            },
            Event::KeyDown { keycode, .. } => {
                if keycode.unwrap() == Keycode::Escape {
                    state.is_running = false;
                }
            }
            _ => {
            }
        }
    }
}