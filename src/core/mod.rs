use crate::core::ecs::world::World;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod ecs;
pub mod render;
pub struct Renderer {}

pub struct Engine {
    running: bool,
    world: World,
    event_pump: EventPump,
    window: Window,

    systems: Vec<fn(&mut World, delta_time: f32)>,
    renderers: Vec<fn(&mut World, &mut Canvas<Window>)>,
    input_handlers: Vec<fn(&mut World, &Event)>,
}

impl Engine {
    pub fn new(sdl: &mut Sdl, window: Window) -> Self {
        Self {
            world: World::new(),
            event_pump: sdl.event_pump().unwrap(),
            running: false,
            systems: vec![],
            renderers: vec![],
            input_handlers: vec![],
            window,
        }
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn register_system(&mut self, system: fn(&mut World, f32)) {
        self.systems.push(system);
    }

    pub fn register_renderer(
        &mut self,
        renderer: fn(&mut World, &mut Canvas<sdl2::video::Window>),
    ) {
        self.renderers.push(renderer);
    }

    pub fn run(&mut self) {
        let mut before = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        self.running = true;
        while self.running {
            let mut delta_time = 0.0_f32;
            while delta_time < 0.016 {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                delta_time = (now - before) as f32 / 1000.0_f32
            }
            before = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            self.process_input();
            self.systems
                .iter()
                .for_each(|system| system(&mut self.world, delta_time));
            self.render();
        }
    }

    pub fn register_input_handler(&mut self, handler: fn(&mut World, &Event)) {
        self.input_handlers.push(handler);
    }

    pub fn process_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.running = false,
                _ => self
                    .input_handlers
                    .iter()
                    .for_each(|handler| handler(&mut self.world, &event)),
            }
        }
    }

    pub fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.window.gl_swap_window();
    }
}
