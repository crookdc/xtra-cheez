use crate::infrastructure::render::Renderer;
use sdl2::event::Event;

pub struct Game {
    running: bool,
    pub renderer: Box<dyn Renderer>,
}

impl Game {
    pub fn new(renderer: Box<dyn Renderer>) -> Self {
        Self {
            running: false,
            renderer,
        }
    }

    pub fn start(&mut self, context: sdl2::Sdl) {
        self.running = true;
        let mut events = context.event_pump().unwrap();
        while self.running {
            // Iterate over all queued input events
            for event in events.poll_iter() {
                match event {
                    // If the Quit event is dispatched, close the game down
                    Event::Quit { .. } => self.running = false,
                    _ => {}
                }
            }
            // TODO: Update all in-scope game objects
            self.renderer.render();
        }
    }
}