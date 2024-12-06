use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Renderer {
    fn render(&mut self);
}

pub struct SDLRenderer {
    internal: Canvas<Window>,
}

impl SDLRenderer {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self {
            internal: canvas,
        }
    }
}

impl Renderer for SDLRenderer {
    fn render(&mut self) {
        self.internal.set_draw_color(Color::RGB(0, 0, 0));
        self.internal.clear();
        self.internal.set_draw_color(Color::RGB(255, 255, 255));
        self.internal.present();
    }
}
