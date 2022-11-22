use bevy_ecs::prelude::*;

use crate::enging::app::AppComponent;
use sdl2::event::*;

pub fn handle_events(mut res: NonSendMut<AppComponent>) {
    let mut is_running = true;
    for event in (&mut res.event_pump).poll_iter() {
        match event {
            Event::Quit{..} => {  
                is_running = false;
                break;
            }
            _ => {

            }
        }
    }
    if !is_running {
        res.is_running = false;
    }
}