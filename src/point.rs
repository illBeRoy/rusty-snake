use crate::direction::Direction;

#[derive(Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn transform(&self, dir: &Direction, times: u16) -> Self {
        match dir {
            Direction::Up => Self {
                x: self.x,
                y: self.y - times,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + times,
            },
            Direction::Left => Self {
                x: self.x - times,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + times,
                y: self.y,
            },
        }
    }
}
