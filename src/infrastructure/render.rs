use crate::transform::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait SDLDrawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
    fn set_position(&mut self, position: Vector2) {}
}

pub struct SDLRenderer {
    internal: Canvas<Window>,
}

impl SDLRenderer {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self { internal: canvas }
    }
}

impl SDLRenderer {
    pub fn render(&mut self, drawables: &[&dyn SDLDrawable]) {
        self.internal.set_draw_color(Color::RGB(0, 0, 200));
        self.internal.clear();
        self.internal.set_draw_color(Color::RGB(255, 255, 255));
        for drawable in drawables {
            drawable.draw(&mut self.internal);
        }
        self.internal.present();
    }
}

pub struct Rectangle {
    pub position: Vector2,
    pub size: Vector2,
}

impl SDLDrawable for Rectangle {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas
            .draw_rect(Rect::new(
                self.position.x,
                self.position.y,
                self.size.x as u32,
                self.size.y as u32,
            ))
            .unwrap()
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }
}
