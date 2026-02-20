#[derive(Clone, PartialEq)]
pub(super) enum CellState {
    Closed,
    Open,
    Flagged,
}

#[derive(Clone)]
pub(super) struct Cell {
    pub(super) has_mine: bool,
    pub(super) state: CellState,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            has_mine: false,
            state: CellState::Closed,
        }
    }
}

#[derive(PartialEq)]
pub enum CellData {
    Closed,
    Open(usize),
    Flagged,
}

pub enum CellConfig {
    Closed,
    Open,
    Mine,
}
