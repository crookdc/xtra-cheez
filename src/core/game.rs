use crate::actor;
use crate::core::render::SDLRenderer;
use crate::core::vector::Vector2;
use sdl2::event::Event;
use sdl2::{EventPump, Sdl};

/// The top layer system responsible for calling and managing all other subsystems.
pub struct Game {
    running: bool,
    context: Sdl,
    pub renderer: SDLRenderer,
    left_paddle: actor::Paddle,
    right_paddle: actor::Paddle,
}

impl Game {
    /// Creates a new 'batteries included' instance of Game without the caller needing to set up
    /// SDL dependencies
    pub fn init(title: &String, size: Vector2) -> Self {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let mut window = video
            .window(title.as_str(), size.x as u32, size.y as u32)
            .position_centered()
            .build()
            .unwrap();
        window.show();
        Self {
            running: false,
            context,
            renderer: SDLRenderer::new(window.into_canvas().build().unwrap()),
            left_paddle: actor::Paddle::new(&Vector2::new(0, 175)),
            right_paddle: actor::Paddle::new(&Vector2::new(580, 175)),
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        let mut events = self.context.event_pump().unwrap();
        let timer = self.context.timer().unwrap();
        let mut ticks = timer.ticks64();
        while self.running {
            let delta =
                crate::core::vector::clamp((timer.ticks64() - ticks) as f64 / 1000.0, 0.0, 0.05);
            // Updates the current ticks to allow us to calculate delta time
            ticks = timer.ticks64();
            while timer.ticks64() < ticks + 16 {}

            self.process_input(&mut events);

            // Update game objects
            self.left_paddle.update(delta);
            self.right_paddle.update(delta);

            // Render game objects
            self.renderer.render(&[
                &self.left_paddle.render_component,
                &self.right_paddle.render_component,
            ]);
        }
    }

    fn process_input(&mut self, events: &mut EventPump) {
        // Iterate over all queued input events and process each in order
        for event in events.poll_iter() {
            match event {
                Event::KeyDown { scancode, .. } => {
                    if scancode.is_none() {
                        continue;
                    }
                    self.left_paddle.on_key_down(scancode.unwrap());
                    self.right_paddle.on_key_down(scancode.unwrap());
                }
                Event::KeyUp { scancode, .. } => {
                    if scancode.is_none() {
                        continue;
                    }
                    self.left_paddle.on_key_up(scancode.unwrap());
                    self.right_paddle.on_key_up(scancode.unwrap());
                }
                Event::Quit { .. } => {
                    println!("Received shutdown event, closing game.");
                    self.running = false;
                }
                _ => {}
            }
        }
    }
}
