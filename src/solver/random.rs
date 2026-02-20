use rand::seq::IndexedRandom;

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

    let Some(pos) = closed_positions.choose(&mut rand::rng()) else {
        println!("Could not find move to make!");
        return None;
    };

    println!("Opening randomly {pos}");
    Some((*pos, false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CellConfig, MineField};

    #[test]
    fn test_random_move_with_closed_cells() {
        let game = MineField::from([
            [CellConfig::Mine, CellConfig::Closed, CellConfig::Closed],
            [CellConfig::Open, CellConfig::Open, CellConfig::Mine],
            [CellConfig::Closed, CellConfig::Open, CellConfig::Closed],
        ]);

        let next_move = choose_random_move(&game);

        match next_move {
            None => panic!("Random move should find a move when there are closed cells"),
            Some((pos, false)) if game.cell_data(pos).is_closed() => {}
            _ => panic!("Invalid move"),
        }
    }

    #[test]
    fn test_random_move_without_closed_cells() {
        let game = MineField::from([
            [CellConfig::Flagged, CellConfig::Flagged, CellConfig::Open],
            [CellConfig::Open, CellConfig::Open, CellConfig::Flagged],
            [CellConfig::Open, CellConfig::Open, CellConfig::Open],
        ]);

        let next_move = choose_random_move(&game);

        assert_eq!(next_move, None)
    }
}
