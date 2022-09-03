mod command;
mod direction;
mod point;
mod snake;

use crate::direction::Direction;
use crate::point::Point;
use crate::snake::Snake;

fn main() {
    let mut snake = Snake::new(Point { x: 10, y: 10 }, 3, Direction::Up);
    println!("{:?}", snake);
    snake.move_in_direction(Direction::Left);
    println!("{:?}", snake);
    snake.move_and_grow_in_direction(Direction::Up);
    println!("{:?}", snake);
}
