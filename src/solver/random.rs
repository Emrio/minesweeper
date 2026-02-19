use crate::{CellData, MineField, Position};

/// choose a random closed cell and open it
pub(super) fn choose_random_move(game: &MineField) -> Option<(Position, bool)> {
    let mut closed_positions = Vec::new();

    for y in 0..game.height() {
        for x in 0..game.height() {
            let pos = (x, y).into();

            match game.cell_data(pos) {
                CellData::Closed => {
                    closed_positions.push(pos);
                }
                _ => {}
            }
        }
    }

    if closed_positions.is_empty() {
        return None;
    }

    let i = rand::random_range(0..closed_positions.len());
    let Some(pos) = closed_positions.get(i) else {
        println!("Could not find move to make!");
        return None;
    };

    println!("Opening randomly {pos}");
    Some((*pos, false))
}
