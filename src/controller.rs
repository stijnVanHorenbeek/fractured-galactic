extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode};

pub struct Controller {
    event_pump: sdl2::EventPump,
}

impl Controller {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        // return result to avoid unwrap
        let event_pump = sdl_context.event_pump().unwrap();
        Controller { event_pump }
    }

    pub fn process_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return false;
                }
                // Event::KeyDown {
                //     keycode: Some(Keycode::J),
                //     ..
                // } => {
                //     zoom_factor *= 1.1;
                // }
                // Event::KeyDown {
                //     keycode: Some(Keycode::K),
                //     ..
                // } => {
                //     zoom_factor /= 1.1;
                // }
                _ => {}
            }
        }
        true
    }
}
