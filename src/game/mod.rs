mod cell;
mod minefield;
mod position;

#[derive(PartialEq)]
pub enum GameState {
    Ongoing,
    Won,
    Lost,
}

pub use cell::CellData;
pub use minefield::MineField;
pub use position::Position;
