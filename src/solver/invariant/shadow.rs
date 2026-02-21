use std::fmt::Display;

use color_eyre::owo_colors::OwoColorize;

use super::cluster::ClusterCell;
use crate::{CellData, MineField, Position};

#[derive(Debug)]
pub(super) enum ShadowCell {
    Closed,
    Open { mines: usize, mines_left: usize },
    ShadowOpen,
    Flagged,
    ShadowFlagged,
}

impl ShadowCell {
    pub fn is_open(&self) -> bool {
        self.get_mines_left().is_some()
    }

    pub fn is_shadow_open(&self) -> bool {
        match self {
            ShadowCell::ShadowOpen => true,
            _ => false,
        }
    }

    pub fn get_mines_left(&self) -> Option<usize> {
        match self {
            ShadowCell::Open {
                mines: _,
                mines_left,
            } => Some(*mines_left),
            _ => None,
        }
    }

    pub fn is_closed(&self) -> bool {
        match self {
            ShadowCell::Closed => true,
            _ => false,
        }
    }

    // pub fn is_flagged(&self) -> bool {
    //     match self {
    //         ShadowCell::Flagged | ShadowCell::ShadowFlagged => true,
    //         _ => false,
    //     }
    // }

    pub fn is_shadow_flagged(&self) -> bool {
        match self {
            ShadowCell::ShadowFlagged => true,
            _ => false,
        }
    }
}

impl Display for ShadowCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShadowCell::Closed => f.write_str("o"),
            ShadowCell::Flagged => f.write_str(&"F".black().to_string()),
            ShadowCell::ShadowFlagged => f.write_str("F"),
            ShadowCell::Open {
                mines: _,
                mines_left,
            } => {
                let string = match mines_left {
                    0 => " ".black().into_styled(),
                    1 => "1".bright_blue().into_styled(),
                    2 => "2".green().into_styled(),
                    3 => "3".red().into_styled(),
                    4 => "4".blue().into_styled(),
                    5 => "5".yellow().into_styled(),
                    6 => "6".cyan().into_styled(),
                    7 => "7".black().into_styled(),
                    8 => "8".bright_black().into_styled(),
                    _ => unreachable!("Invalid value of mines left"),
                };
                f.write_str(&string.to_string())
            }
            ShadowCell::ShadowOpen => f.write_str("+"),
        }
    }
}

pub(super) struct ShadowMinefield<'a> {
    pub(super) game: &'a MineField,
    pub(super) field: Vec<ShadowCell>,
}

impl<'a> ShadowMinefield<'a> {
    pub fn new(game: &'a MineField) -> Self {
        let mut field = Vec::new();
        for y in 0..game.height() {
            for x in 0..game.width() {
                let pos = (x, y).into();
                field.push(match game.cell_data(pos) {
                    CellData::Closed => ShadowCell::Closed,
                    CellData::Open(mines) => ShadowCell::Open {
                        mines,
                        mines_left: mines - Self::mines_flagged(pos, game),
                    },
                    CellData::Flagged => ShadowCell::Flagged,
                });
            }
        }
        Self { game, field }
    }

    fn mines_flagged(pos: Position, game: &MineField) -> usize {
        pos.neighbours(game)
            .into_iter()
            .filter_map(|neighbour| match game.cell_data(neighbour) {
                CellData::Flagged => Some(()),
                _ => None,
            })
            .count()
    }

    pub fn get_cell(&self, pos: &Position) -> &ShadowCell {
        &self.field[pos.to_index(self.game.width())]
    }

    fn get_cell_mut(&mut self, pos: &Position) -> Option<&mut ShadowCell> {
        self.field.get_mut(pos.to_index(self.game.width()))
    }

    // pub fn get_remaining_mines(&self, pos: Position) -> usize {
    //     let total_mines = self.game.cell_data(pos).get_mines_count().unwrap();
    //     let flagged_mines = pos
    //         .neighbours(self.game)
    //         .into_iter()
    //         .map(|neighbour| self.get_cell(&neighbour))
    //         .filter(|cell| cell.is_flagged())
    //         .count();
    //     // .filter_map(|neighbour| self.get_cell(neighbour).get_mines_count())

    //     total_mines - flagged_mines
    // }

    pub fn get_closed_positions(&self, pos: &Position) -> Vec<Position> {
        pos.neighbours(self.game)
            .into_iter()
            .filter(|neighbour| self.get_cell(neighbour).is_closed())
            .collect()
    }

    pub fn flag(&mut self, cell: &ClusterCell) -> bool {
        if !self.get_cell(&cell.pos).is_closed() {
            panic!("Position cannot be shadow flagged")
        }

        for neighbour in &cell.open_neighbours {
            if let Some(0) = self.get_cell(neighbour).get_mines_left() {
                return false;
            }
        }

        for neighbour in &cell.open_neighbours {
            if let Some(ShadowCell::Open {
                mines: _,
                mines_left,
            }) = self.get_cell_mut(neighbour)
            {
                *mines_left -= 1;
            }
        }

        let cell = self.get_cell_mut(&cell.pos).unwrap();
        *cell = ShadowCell::ShadowFlagged;

        true
    }

    pub fn unflag(&mut self, cell: &ClusterCell) {
        if !self.get_cell(&cell.pos).is_shadow_flagged() {
            panic!("Position cannot be shadow unflagged")
        }

        for neighbour in &cell.open_neighbours {
            if let Some(ShadowCell::Open {
                mines: _,
                mines_left,
            }) = self.get_cell_mut(neighbour)
            {
                *mines_left += 1;
            }
        }

        let cell = self.get_cell_mut(&cell.pos).unwrap();
        *cell = ShadowCell::Closed;
    }

    pub fn open(&mut self, cell: &ClusterCell) -> bool {
        if !self.get_cell(&cell.pos).is_closed() {
            panic!("Position cannot be shadow opened")
        }

        for neighbour in &cell.open_neighbours {
            if let ShadowCell::Open {
                mines: _,
                mines_left,
            } = self.get_cell(neighbour)
                && *mines_left > 0
                && self.get_closed_positions(neighbour).len() < *mines_left
            {
                return false;
            }
        }

        let cell = self.get_cell_mut(&cell.pos).unwrap();
        *cell = ShadowCell::ShadowOpen;

        true
    }

    pub fn unopen(&mut self, cell: &ClusterCell) {
        if !self.get_cell(&cell.pos).is_shadow_open() {
            panic!("Position cannot be shadow opened")
        }

        let cell = self.get_cell_mut(&cell.pos).unwrap();
        *cell = ShadowCell::Closed
    }
}

impl<'a> Display for ShadowMinefield<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&"—".repeat(self.game.width()))?;

        for row in self.field.chunks(self.game.width()) {
            f.write_str("\n")?;
            for cell in row.iter() {
                cell.fmt(f)?
            }
        }

        Ok(())
    }
}
