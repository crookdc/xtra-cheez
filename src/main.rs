use skrutt::core::game::Game;
use skrutt::core::vector::Vector2;

fn main() {
    let mut game = Game::init(&"Skrutt Game Engine".to_string(), Vector2::new(600, 400));
    game.start();
}
