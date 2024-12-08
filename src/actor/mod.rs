use crate::core::render::{Rectangle, SDLDrawable};
use crate::core::vector::Vector2;
use sdl2::keyboard::Scancode;
use sdl2::render::Canvas;
use sdl2::video::Window;

trait Component {
    fn update(&mut self, _delta: f64) {}
}

pub struct RenderComponent {
    drawable: Box<dyn SDLDrawable>,
    transform_component: TransformComponent,
}

impl Component for RenderComponent {
    fn update(&mut self, _: f64) {
        self.drawable
            .set_position(self.transform_component.position);
    }
}

impl SDLDrawable for RenderComponent {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        self.drawable.draw(canvas);
    }

    fn set_position(&mut self, position: Vector2) {
        self.transform_component.position = position;
    }
}

#[derive(Clone, Copy)]
struct TransformComponent {
    position: Vector2,
}

impl Component for TransformComponent {}

pub struct Paddle {
    pub render_component: RenderComponent,
    transform: TransformComponent,
    direction: Vector2,
}

impl Paddle {
    pub fn new(position: &Vector2) -> Self {
        Self {
            render_component: RenderComponent {
                drawable: Box::new(Rectangle {
                    size: Vector2::new(20, 50),
                    position: position.clone(),
                }),
                transform_component: TransformComponent {
                    position: position.clone(),
                },
            },
            transform: TransformComponent {
                position: position.clone(),
            },
            direction: Vector2::zero(),
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.transform.position.y += (self.direction.y as f64 * 100.0f64 * delta) as i32;

        self.render_component.update(delta);
        self.render_component.set_position(self.transform.position);
        self.transform.update(delta);
    }

    pub fn on_key_down(&mut self, code: Scancode) {
        self.direction.y = match code {
            Scancode::Up => -1,
            Scancode::Down => 1,
            _ => 0,
        };
    }

    pub fn on_key_up(&mut self, code: Scancode) {
        match code {
            Scancode::Up | Scancode::Down => self.direction.y = 0,
            _ => {}
        }
    }
}
