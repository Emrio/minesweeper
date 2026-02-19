use std::fmt::Display;

use color_eyre::owo_colors::OwoColorize;

#[derive(Clone, PartialEq)]
enum CellState {
    Closed,
    Open,
    Flagged,
}

#[derive(PartialEq)]
pub enum CellData {
    Closed,
    Open(usize),
    Flagged,
}

#[derive(Clone)]
struct Cell {
    has_mine: bool,
    state: CellState,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            has_mine: false,
            state: CellState::Closed,
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

pub struct MineField {
    started: bool,
    width: usize,
    height: usize,
    field: Vec<Cell>,
}

impl MineField {
    pub fn new(width: usize, height: usize, mut mines: u32) -> Self {
        let size = width * height;

        if mines as usize >= size {
            panic!("Need at least one free spot");
        }

        let mut field = vec![Cell::default(); size];

        while mines > 0 {
            let i = rand::random_range(0..size);

            if !field[i].has_mine {
                field[i].has_mine = true;
                mines -= 1;
            }
        }

        Self {
            started: true,
            width,
            height,
            field,
        }
    }

    fn move_mine_somewhere_else(&mut self, initial: Position) {
        loop {
            let pos = Position {
                x: rand::random_range(0..self.width),
                y: rand::random_range(0..self.height),
            };

            if pos == initial {
                continue;
            }

            let cell = self.get_cell_mut(pos);
            if !cell.has_mine {
                cell.has_mine = true;
                break;
            }
        }

        let cell = self.get_cell_mut(initial);
        cell.has_mine = false;
    }

    pub fn play(&mut self, pos: Position, flag: bool) -> GameState {
        if flag {
            let cell = self.get_cell_mut(pos);
            if cell.state == CellState::Flagged {
                cell.state = CellState::Closed;
            } else {
                cell.state = CellState::Flagged;
            }

            return GameState::Ongoing;
        }

        let cell = self.get_cell(pos);
        if cell.has_mine && self.started {
            self.move_mine_somewhere_else(pos);
        }

        self.started = false;

        let cell = self.get_cell_mut(pos);

        if cell.has_mine {
            cell.state = CellState::Open;
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

    fn get_cell(&self, pos: Position) -> &Cell {
        &self.field[pos.y * self.width + pos.x]
    }

    fn get_cell_mut(&mut self, pos: Position) -> &mut Cell {
        self.field.get_mut(pos.y * self.width + pos.x).unwrap()
    }

    fn neighbouring_mines(&self, pos: Position) -> usize {
        self.neighbours(pos)
            .into_iter()
            .filter(|neighbour| self.get_cell(*neighbour).has_mine)
            .count()
    }

    pub fn cell_data(&self, pos: Position) -> CellData {
        let cell = self.get_cell(pos);
        match cell.state {
            CellState::Closed => CellData::Closed,
            CellState::Flagged => CellData::Flagged,
            CellState::Open => CellData::Open(self.neighbouring_mines(pos)),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Display for MineField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&"—".repeat(self.width))?;

        for (y, row) in self.field.chunks(self.width).enumerate() {
            f.write_str("\n")?;
            for (x, cell) in row.iter().enumerate() {
                match cell.state {
                    CellState::Closed => f.write_str("o")?,
                    CellState::Flagged => f.write_str(&"F".black().to_string())?,
                    CellState::Open if cell.has_mine => {
                        f.write_str(&"X".on_red().blink().to_string())?
                    }
                    CellState::Open => {
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
    game: &'a mut MineField,
    visited: Vec<bool>,
}

impl<'a> Opener<'a> {
    fn new(game: &'a mut MineField) -> Self {
        let visited = vec![false; game.field.len()];
        Self { game, visited }
    }

    fn open(&mut self, pos: Position) {
        if self.visited[pos.x + pos.y * self.game.width] {
            return;
        }

        self.visited[pos.x + pos.y * self.game.width] = true;
        self.game.get_cell_mut(pos).state = CellState::Open;

        if self.game.neighbouring_mines(pos) > 0 {
            return;
        }

        for neighbour in self.game.neighbours(pos) {
            self.open(neighbour);
        }
    }
}
