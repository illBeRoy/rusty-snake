use crossterm::cursor::{Hide, MoveTo};
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, SetSize};
use crossterm::ExecutableCommand;
use std::io::stdout;

use crate::point::{self, Point};

pub struct Renderer {
    dimensions: (u16, u16),
    background_color: Color,
    foreground_color: Color,
    initialized: bool,
}

impl Renderer {
    pub fn new(dimensions: (u16, u16), background_color: Color, foreground_color: Color) -> Self {
        Self {
            dimensions,
            background_color,
            foreground_color,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> &mut Self {
        assert!(!self.initialized, "Renderer already initialized");

        enable_raw_mode().unwrap();

        stdout()
            .execute(SetSize(self.dimensions.0, self.dimensions.1))
            .unwrap()
            .execute(Hide)
            .unwrap();

        self.initialized = true;

        self
    }

    pub fn exit(&mut self) -> &mut Self {
        assert!(self.initialized, "Renderer not initialized");

        disable_raw_mode().unwrap();

        self.initialized = false;

        self
    }

    pub fn clear(&self) -> &Self {
        assert!(self.initialized, "Renderer not initialized");

        stdout()
            .execute(SetBackgroundColor(self.background_color))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap();

        self
    }

    pub fn clear_line(&self, y: u16) -> &Self {
        assert!(self.initialized, "Renderer not initialized");

        stdout()
            .execute(SetBackgroundColor(self.background_color))
            .unwrap()
            .execute(MoveTo(0, y))
            .unwrap()
            .execute(Clear(ClearType::CurrentLine))
            .unwrap();

        self
    }

    pub fn clear_points(&self, points: &Vec<Point>) {
        for point in points {
            self.draw_character_at(point.clone(), ' ');
        }
    }

    pub fn draw_rect_frame(&self, from: &Point, size: (u16, u16), character: char) -> &Self {
        assert!(self.initialized, "Renderer not initialized");

        let top = from.y;
        let left = from.x;
        let right = from.x + size.0;
        let bottom = from.y + size.1;

        for x in left..right {
            self.draw_character_at(Point { x, y: top }, character)
                .draw_character_at(Point { x, y: bottom }, character);
        }

        for y in top..bottom {
            self.draw_character_at(Point { x: left, y }, character)
                .draw_character_at(Point { x: right, y }, character);
        }

        self
    }

    pub fn draw_points(&self, points: &Vec<Point>, character: char) -> &Self {
        assert!(self.initialized, "Renderer not initialized");

        for point in points {
            self.draw_character_at(point.clone(), character);
        }

        self
    }

    pub fn draw_text(&self, text: &str, at: &Point) -> &Self {
        assert!(self.initialized, "Renderer not initialized");

        stdout()
            .execute(MoveTo(at.x, at.y))
            .unwrap()
            .execute(SetForegroundColor(Color::White))
            .unwrap()
            .execute(Print(text))
            .unwrap();

        self
    }

    fn draw_character_at(&self, point: Point, character: char) -> &Self {
        stdout()
            .execute(SetForegroundColor(self.foreground_color))
            .unwrap()
            .execute(MoveTo(point.x, point.y))
            .unwrap()
            .execute(Print(character))
            .unwrap();

        self
    }
}
