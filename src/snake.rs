use std::cmp::max;

use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
}

impl Snake {
    pub fn new(head: Point, length: u8, direction: Direction) -> Self {
        let body: Vec<Point> = (0..max(length, 1))
            .into_iter()
            .map(|i| head.transform(&direction.opposite(), i.into()))
            .collect();

        Snake { body, direction }
    }

    pub fn borrow_body(&self) -> &Vec<Point> {
        &self.body
    }

    pub fn move_in_direction(&mut self, direction: Direction) {
        self.move_in_direction_and_maybe_grow(direction, false)
    }

    pub fn move_and_grow_in_direction(&mut self, direction: Direction) {
        self.move_in_direction_and_maybe_grow(direction, true)
    }

    fn move_in_direction_and_maybe_grow(&mut self, direction: Direction, should_grow: bool) {
        let new_head = self.body.first().unwrap().transform(&direction, 1);

        self.body.insert(0, new_head);

        if !should_grow {
            self.body.pop();
        }

        self.direction = direction;
    }
}
