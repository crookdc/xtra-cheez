use crate::infrastructure::render::{Rectangle, SDLDrawable};
use crate::transform::Vector2;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fmt::Debug;

trait Component {
    fn update(&mut self, delta: f64) {}
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
    rotation: Vector2,
    scale: Vector2,
}

impl Component for TransformComponent {}

pub struct Paddle {
    pub render_component: RenderComponent,
    transform: TransformComponent,
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
                    rotation: Vector2::new(0, 0),
                    scale: Vector2::new(1, 1),
                },
            },
            transform: TransformComponent {
                position: position.clone(),
                rotation: Vector2::new(0, 0),
                scale: Vector2::new(1, 1),
            },
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.render_component.update(delta);
        self.render_component.set_position(self.transform.position);
        self.transform.update(delta);
    }

    pub fn on_key_down(&mut self, delta: f64, key_code: Keycode) {
        match key_code {
            Keycode::UP => {
                self.transform.position.y -= (200.0 * delta) as i32;
            }
            _ => {}
        }
    }
}
