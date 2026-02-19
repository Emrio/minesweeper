use crate::{MineField, Position};

mod immediate;
mod random;

pub fn find_next_move(game: &MineField) -> Option<(Position, bool)> {
    immediate::find_immediate_move(game).or_else(|| random::choose_random_move(game))
}
