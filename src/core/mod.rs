use crate::core::ecs::world::World;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::{EventPump, Sdl};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod ecs;

pub struct Engine {
    running: bool,
    // The currently loaded world, which is equivalent to a game level
    world: World,
    event_pump: EventPump,
    canvas: Canvas<sdl2::video::Window>,

    systems: Vec<fn(&mut World, delta_time: f32)>,
    renderers: Vec<fn(&mut World, &mut Canvas<sdl2::video::Window>)>,
    input_handlers: Vec<fn(&mut World, &Event)>,
}

impl Engine {
    pub fn new(sdl: &mut Sdl) -> Self {
        Self {
            world: World::new(),
            event_pump: sdl.event_pump().unwrap(),
            canvas: sdl
                .video()
                .unwrap()
                .window("psx", 800, 800)
                .build()
                .unwrap()
                .into_canvas()
                .build()
                .unwrap(),
            running: false,
            systems: vec![],
            renderers: vec![],
            input_handlers: vec![],
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
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.renderers
            .iter()
            .for_each(|r| r(&mut self.world, &mut self.canvas));
        self.canvas.present();
    }
}
