use crate::{CellData, MineField, Position};

/// when a cell's neighbouring mines are all flagged, open all remaining closed neighbours
/// when a cell's closed neighbourhood size is equal to its mine own count, all these neighbours are bombs, flag them
pub(super) fn find_immediate_move(game: &MineField) -> Option<(Position, bool)> {
    for y in 0..game.height() {
        for x in 0..game.height() {
            let pos = (x, y).into();

            match game.cell_data(pos) {
                CellData::Flagged | CellData::Closed => {}
                CellData::Open(mines_around) => {
                    let neighbours = pos.neighbours(game);
                    let mut neighbours: (Vec<(Position, CellData)>, Vec<(Position, CellData)>) =
                        neighbours
                            .into_iter()
                            .map(|neighbour| (neighbour, game.cell_data(neighbour)))
                            .filter(|(_, cell)| match cell {
                                CellData::Closed | CellData::Flagged => true,
                                CellData::Open(_) => false,
                            })
                            .partition(|(_, cell)| {
                                if let CellData::Closed = cell {
                                    true
                                } else {
                                    false
                                }
                            });

                    let closed_positions = neighbours.0.len();
                    let mines_flagged = neighbours.1.len();

                    let mines_left = mines_around - mines_flagged;

                    if !(mines_left == 0 || closed_positions == mines_left) {
                        continue;
                    }

                    let Some((neighbour, _)) = neighbours.0.pop() else {
                        continue;
                    };

                    if mines_left > 0 {
                        println!("Flagging {neighbour}");
                    } else {
                        println!("Opening {neighbour}");
                    }
                    return Some((neighbour, mines_left > 0));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CellConfig, MineField};

    #[test]
    fn test_immediate_move_1bo() {
        let game = MineField::from([[CellConfig::Open, CellConfig::Mine, CellConfig::Open]]);

        let next_move = find_immediate_move(&game);

        assert_eq!(next_move, Some((Position::from(1, 0), true)));
    }

    #[test]
    fn test_immediate_move_use_flags() {
        let game = MineField::from([
            [CellConfig::Open, CellConfig::Mine, CellConfig::Closed],
            [CellConfig::Open, CellConfig::Flagged, CellConfig::Closed],
        ]);

        let next_move = find_immediate_move(&game);

        assert_eq!(next_move, Some((Position::from(1, 0), true)));
    }
}
