use skrutt::infrastructure::game::Game;
use skrutt::infrastructure::render::SDLRenderer;

fn main() {
    let context = sdl2::init().unwrap();

    let video = context.video().unwrap();
    let mut window = video
        .window("Skrutt Game Engine", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    window.show();

    let canvas = window.into_canvas().build().unwrap();
    let mut game = Game::new(Box::new(SDLRenderer::new(canvas)));
    game.start(context);
}
