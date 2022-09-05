use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use std::time::Duration;

use crate::direction::Direction;

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Quit,
    Turn(Direction),
}

impl Command {
    pub fn await_command_from_user_input(timeout: Duration) -> Option<Self> {
        if poll(timeout).ok()? {
            let event = read().ok()?;
            return match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => Some(Self::Quit),
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => Some(Self::Turn(Direction::Left)),
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => Some(Self::Turn(Direction::Right)),
                Event::Key(KeyEvent {
                    code: KeyCode::Up, ..
                }) => Some(Self::Turn(Direction::Up)),
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    ..
                }) => Some(Self::Turn(Direction::Down)),
                _ => None,
            };
        }

        None
    }
}
