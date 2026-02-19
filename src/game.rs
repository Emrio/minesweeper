use std::fmt::Display;

use color_eyre::owo_colors::OwoColorize;

#[derive(Clone, PartialEq)]
enum FieldState {
    Closed,
    Open,
    Flagged,
}

#[derive(PartialEq)]
pub enum FieldData {
    Closed,
    Open(usize),
    Flagged,
}

#[derive(Clone)]
struct Field {
    has_bomb: bool,
    state: FieldState,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            has_bomb: false,
            state: FieldState::Closed,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Into<Position> for (usize, usize) {
    fn into(self) -> Position {
        Position {
            x: self.0,
            y: self.1,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

#[derive(PartialEq)]
pub enum GameState {
    Ongoing,
    Won,
    Lost,
}

pub struct Minesweeper {
    started: bool,
    width: usize,
    height: usize,
    map: Vec<Field>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mut bombs: u32) -> Self {
        let size = width * height;

        if bombs as usize >= size {
            panic!("Need at least one free spot");
        }

        let mut map = vec![Field::default(); size];

        while bombs > 0 {
            let i = rand::random_range(0..size);

            if !map[i].has_bomb {
                map[i].has_bomb = true;
                bombs -= 1;
            }
        }

        Self {
            started: true,
            width,
            height,
            map,
        }
    }

    fn move_bomb_somewhere_else(&mut self, initial: Position) {
        loop {
            let pos = Position {
                x: rand::random_range(0..self.width),
                y: rand::random_range(0..self.height),
            };

            if pos == initial {
                continue;
            }

            let field = self.get_field_mut(pos);
            if !field.has_bomb {
                field.has_bomb = true;
                break;
            }
        }

        let field = self.get_field_mut(initial);
        field.has_bomb = false;
    }

    pub fn play(&mut self, pos: Position, flag: bool) -> GameState {
        if flag {
            let field = self.get_field_mut(pos);
            if field.state == FieldState::Flagged {
                field.state = FieldState::Closed;
            } else {
                field.state = FieldState::Flagged;
            }

            return GameState::Ongoing;
        }

        let field = self.get_field(pos);
        if field.has_bomb && self.started {
            self.move_bomb_somewhere_else(pos);
        }

        self.started = false;

        let field = self.get_field_mut(pos);

        if field.has_bomb {
            field.state = FieldState::Open;
            return GameState::Lost;
        }

        let mut opener = Opener::new(self);
        opener.open(pos);

        return GameState::Ongoing;
    }

    pub fn neighbours(&self, pos: Position) -> Vec<Position> {
        let mut neigbhours = Vec::new();

        let (x, y) = (pos.x, pos.y);

        if x > 0 && y > 0 {
            neigbhours.push((x - 1, y - 1).into());
        }

        if y > 0 {
            neigbhours.push((x, y - 1).into());
        }

        if x < self.width - 1 && y > 0 {
            neigbhours.push((x + 1, y - 1).into());
        }

        if x > 0 {
            neigbhours.push((x - 1, y).into());
        }

        if x < self.width - 1 {
            neigbhours.push((x + 1, y).into());
        }

        if x > 0 && y < self.height - 1 {
            neigbhours.push((x - 1, y + 1).into());
        }

        if y < self.height - 1 {
            neigbhours.push((x, y + 1).into());
        }

        if x < self.width - 1 && y < self.height - 1 {
            neigbhours.push((x + 1, y + 1).into());
        }

        neigbhours
    }

    fn get_field(&self, pos: Position) -> &Field {
        &self.map[pos.y * self.width + pos.x]
    }

    fn get_field_mut(&mut self, pos: Position) -> &mut Field {
        self.map.get_mut(pos.y * self.width + pos.x).unwrap()
    }

    fn neighbouring_mines(&self, pos: Position) -> usize {
        self.neighbours(pos)
            .into_iter()
            .filter(|neighbour| self.get_field(*neighbour).has_bomb)
            .count()
    }

    pub fn field_data(&self, pos: Position) -> FieldData {
        let field = self.get_field(pos);
        match field.state {
            FieldState::Closed => FieldData::Closed,
            FieldState::Flagged => FieldData::Flagged,
            FieldState::Open => FieldData::Open(self.neighbouring_mines(pos)),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&"—".repeat(self.width))?;

        for (y, row) in self.map.chunks(self.width).enumerate() {
            f.write_str("\n")?;
            for (x, field) in row.iter().enumerate() {
                match field.state {
                    FieldState::Closed => f.write_str("o")?,
                    FieldState::Flagged => f.write_str(&"F".black().to_string())?,
                    FieldState::Open if field.has_bomb => {
                        f.write_str(&"X".on_red().blink().to_string())?
                    }
                    FieldState::Open => {
                        let mines = self.neighbouring_mines((x, y).into());
                        let string = match mines {
                            0 => " ".black().into_styled(),
                            1 => "1".bright_blue().into_styled(),
                            2 => "2".green().into_styled(),
                            3 => "3".red().into_styled(),
                            4 => "4".blue().into_styled(),
                            5 => "5".yellow().into_styled(),
                            6 => "6".cyan().into_styled(),
                            7 => "7".black().into_styled(),
                            8 => "8".bright_black().into_styled(),
                            _ => unreachable!("Invalid value of mines"),
                        };
                        f.write_str(&string.to_string())?;
                    }
                };
            }
        }

        Ok(())
    }
}

struct Opener<'a> {
    game: &'a mut Minesweeper,
    visited: Vec<bool>,
}

impl<'a> Opener<'a> {
    fn new(game: &'a mut Minesweeper) -> Self {
        let visited = vec![false; game.map.len()];
        Self { game, visited }
    }

    fn open(&mut self, pos: Position) {
        if self.visited[pos.x + pos.y * self.game.width] {
            return;
        }

        self.visited[pos.x + pos.y * self.game.width] = true;
        self.game.get_field_mut(pos).state = FieldState::Open;

        if self.game.neighbouring_mines(pos) > 0 {
            return;
        }

        for neighbour in self.game.neighbours(pos) {
            self.open(neighbour);
        }
    }
}
