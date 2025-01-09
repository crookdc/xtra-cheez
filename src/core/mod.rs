use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use std::collections::HashSet;

pub mod ecs;
pub mod physics;
pub mod render;

pub struct Keymap(pub HashSet<Scancode>);

impl Keymap {
    pub fn consume(&mut self, event: &Event) {
        if let Event::KeyDown { scancode, .. } = event {
            self.0.insert(scancode.unwrap());
        } else if let Event::KeyUp { scancode, .. } = event {
            self.0.remove(&scancode.unwrap());
        }
    }

    pub fn pressed(&self, scancode: Scancode) -> bool {
        self.0.contains(&scancode)
    }

    pub fn axis(&self, pos: Scancode, neg: Scancode) -> f32 {
        let mut axis = 0.0;
        if self.pressed(pos) {
            axis += 1.0;
        }
        if self.pressed(neg) {
            axis -= 1.0;
        }
        axis
    }
}

pub struct Mouse(pub i32, pub i32);

impl Mouse {
    pub fn consume(&mut self, event: &Event) {
        if let Event::MouseMotion { xrel, yrel, .. } = event {
            self.0 = *xrel;
            self.1 = *yrel;
        } else {
            self.0 = 0;
            self.1 = 0;
        }
    }

    pub fn reset(&mut self) {
        self.0 = 0;
        self.1 = 0;
    }
}

pub fn radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}
