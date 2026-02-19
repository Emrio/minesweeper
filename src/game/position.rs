use std::fmt::Display;

use crate::MineField;

#[derive(PartialEq, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl Into<Position> for (usize, usize) {
    fn into(self) -> Position {
        Position {
            x: self.0,
            y: self.1,
        }
    }
}

impl Position {
    pub(super) fn random(width: usize, height: usize) -> Self {
        Self {
            x: rand::random_range(0..width),
            y: rand::random_range(0..height),
        }
    }

    pub(super) fn to_index(self, width: usize) -> usize {
        self.x + self.y * width
    }

    pub fn neighbours(self, context: &MineField) -> Vec<Position> {
        let mut neigbhours = Vec::new();

        let (x, y) = (self.x, self.y);

        if x > 0 && y > 0 {
            neigbhours.push((x - 1, y - 1).into());
        }

        if y > 0 {
            neigbhours.push((x, y - 1).into());
        }

        if x < context.width() - 1 && y > 0 {
            neigbhours.push((x + 1, y - 1).into());
        }

        if x > 0 {
            neigbhours.push((x - 1, y).into());
        }

        if x < context.width() - 1 {
            neigbhours.push((x + 1, y).into());
        }

        if x > 0 && y < context.height() - 1 {
            neigbhours.push((x - 1, y + 1).into());
        }

        if y < context.height() - 1 {
            neigbhours.push((x, y + 1).into());
        }

        if x < context.width() - 1 && y < context.height() - 1 {
            neigbhours.push((x + 1, y + 1).into());
        }

        neigbhours
    }
}
