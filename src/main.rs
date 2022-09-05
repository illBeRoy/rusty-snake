mod command;
mod direction;
mod game;
mod point;
mod render;
mod snake;

use crate::game::Game;

fn main() {
    const SCREEN_SIZE: (u16, u16) = (50, 27);

    let mut game = Game::new(SCREEN_SIZE, 12);

    game.run();
}
