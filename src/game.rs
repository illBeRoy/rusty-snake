use crossterm::style::Color;
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
                    y: board_size.1 / 2 + 2,
                },
                3,
                Direction::Up,
            ),
            board_size,
            score: 0,
            moving_direction: Direction::Up,
            target_fps,
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

        while self.game_state != GameState::Quit {
            self.game_state = match self.game_state {
                GameState::Playing => self.playing_state_game_loop(),
                GameState::GameOver => self.playing_state_game_over_loop(),
                GameState::Quit => GameState::Quit,
            };
        }
    }

    fn render_score(&self) {
        self.renderer
            .clear_line(0)
            .draw_text(&format!("Score: {}", self.score), &Point { x: 1, y: 0 });
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
        self.snake.move_in_direction(self.moving_direction.clone());
        self.renderer.draw_points(self.snake.borrow_body(), '@');

        GameState::Playing
    }

    fn playing_state_game_over_loop(&self) -> GameState {
        GameState::GameOver
    }
}
