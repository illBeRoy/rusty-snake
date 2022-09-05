use crossterm::style::Color;
use rand::seq::SliceRandom;
use rand::{self, Rng};
use std::time::Duration;

use crate::command::Command;
use crate::direction::Direction;
use crate::point::Point;
use crate::render::Renderer;
use crate::snake::Snake;

pub struct Game {
    score: u16,
    game_state: GameState,
    snake: Snake,
    fruit_position: Point,
    board_size: (u16, u16),
    target_fps: u16,
    renderer: Renderer,
    moving_direction: Direction,
}

#[derive(PartialEq)]
enum GameState {
    Playing,
    GameOver,
    Quit,
}

impl Game {
    pub fn new(board_size: (u16, u16), target_fps: u16) -> Self {
        Game {
            snake: Snake::new(
                Point {
                    x: board_size.0 / 2,
                    y: board_size.1 / 2,
                },
                3,
                Direction::Up,
            ),
            board_size,
            score: 0,
            moving_direction: Direction::Up,
            target_fps,
            fruit_position: Point { x: 0, y: 0 },
            game_state: GameState::Playing,
            renderer: Renderer::new(
                (board_size.0, board_size.1 + 2),
                Color::DarkBlue,
                Color::Magenta,
            ),
        }
    }

    pub fn run(&mut self) {
        self.renderer.init().clear();

        self.game_loop();

        self.renderer.exit();
    }

    fn game_loop(&mut self) {
        self.renderer
            .clear()
            .draw_rect_frame(&Point { x: 0, y: 2 }, self.board_size, '#')
            .draw_points(self.snake.borrow_body(), '@');

        self.render_score();
        self.place_fruit_somewhere_on_board();

        while self.game_state != GameState::Quit {
            self.game_state = match self.game_state {
                GameState::Playing => self.playing_state_game_loop(),
                GameState::GameOver => self.playing_state_game_over_loop(),
                GameState::Quit => GameState::Quit,
            };
        }
    }

    fn playing_state_game_loop(&mut self) -> GameState {
        let frame_duration: u16 = 1000u16 / self.target_fps;

        let next_command =
            Command::await_command_from_user_input(Duration::from_millis(frame_duration.into()));

        if let Some(Command::Quit) = next_command {
            return GameState::Quit;
        }

        if let Some(Command::Turn(direction)) = next_command {
            if direction != self.moving_direction && direction != self.moving_direction.opposite() {
                self.moving_direction = direction;
            }
        }

        self.renderer.clear_points(self.snake.borrow_body());

        if self.is_snake_eating_fruit() {
            self.snake
                .move_and_grow_in_direction(self.moving_direction.clone());
            self.score += 100;
            self.render_score();
            self.place_fruit_somewhere_on_board();
        } else {
            self.snake.move_in_direction(self.moving_direction.clone());
        }

        self.renderer.draw_points(self.snake.borrow_body(), '@');
        self.renderer.draw_point(self.snake.borrow_head(), '🐸');

        if self.is_snake_hitting_borders() || self.snake.is_overlapping_self() {
            self.renderer.draw_point(self.snake.borrow_head(), '💀');
            return GameState::GameOver;
        }

        GameState::Playing
    }

    fn playing_state_game_over_loop(&self) -> GameState {
        let mut should_show_blinking_text = true;

        while Command::await_command_from_user_input(Duration::from_millis(500))
            != Some(Command::Quit)
        {
            let text = "Game Over";
            let text_position = Point {
                x: (self.board_size.0 / 2) - ((text.len() as u16) / 2),
                y: (self.board_size.1 / 2) + 2,
            };

            match should_show_blinking_text {
                true => self.renderer.draw_text(text, &text_position),
                false => self
                    .renderer
                    .draw_text(&" ".repeat(text.len()), &text_position),
            };

            should_show_blinking_text = !should_show_blinking_text;
        }

        GameState::Quit
    }

    fn render_score(&self) {
        self.renderer
            .clear_line(0)
            .draw_text(&format!("Score: {}", self.score), &Point { x: 1, y: 0 });
    }

    fn place_fruit_somewhere_on_board(&mut self) {
        loop {
            self.fruit_position = Point {
                x: rand::thread_rng().gen_range(1..(self.board_size.0 - 1)),
                y: rand::thread_rng().gen_range(3..(self.board_size.1)),
            };

            let is_new_fruit_position_inside_snake =
                self.snake.borrow_body().iter().any(|snake_body_part| {
                    snake_body_part.x == self.fruit_position.x
                        && snake_body_part.y == self.fruit_position.y
                });

            if is_new_fruit_position_inside_snake {
                continue;
            }

            let fruits = vec!['🍎', '🍏', '🍑', '🍒', '🍊', '🍓', '🍇', '🍉', '🍌', '🍍'];
            let fruit = fruits.choose(&mut rand::thread_rng()).unwrap().clone();

            self.renderer.draw_point(
                &Point {
                    x: self.fruit_position.x,
                    y: self.fruit_position.y,
                },
                fruit,
            );

            break;
        }
    }

    fn is_snake_eating_fruit(&self) -> bool {
        self.snake.is_bumping_at_point(&self.fruit_position)
    }

    fn is_snake_hitting_borders(&self) -> bool {
        self.snake.borrow_head().x == 0
            || self.snake.borrow_head().x == self.board_size.0
            || self.snake.borrow_head().y == 2
            || self.snake.borrow_head().y == self.board_size.1 + 2
    }
}
