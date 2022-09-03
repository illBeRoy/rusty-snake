mod command;
mod direction;
mod point;
mod render;
mod snake;

use std::{thread::sleep, time::Duration};

use crate::direction::Direction;
use crate::point::Point;
use crate::render::Renderer;
use crate::snake::Snake;
use crossterm::style::Color::{DarkBlue, Magenta};

fn main() {
    let mut snake = Snake::new(Point { x: 5, y: 5 }, 3, Direction::Up);
    let mut renderer = Renderer::new((10, 10), DarkBlue, Magenta);

    renderer
        .init()
        .clear()
        .draw_rect_frame(&Point { x: 0, y: 0 }, (10, 10), '#')
        .draw_points(snake.borrow_body(), '@');

    sleep(Duration::from_secs(5));

    renderer.exit();
}
