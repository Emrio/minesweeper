use crate::{FieldData, Minesweeper};

pub fn pass(game: &Minesweeper) -> Option<(usize, usize, bool)> {
    let mut closed_positions = Vec::new();

    for y in 0..game.height() {
        for x in 0..game.height() {
            match game.field_data(x, y) {
                FieldData::Flagged => {}
                FieldData::Closed => {
                    closed_positions.push((x, y));
                }
                FieldData::Open(mines_around) => {
                    let neighbours = game.neighbours(x, y);
                    let mut neighbours: (
                        Vec<(usize, usize, FieldData)>,
                        Vec<(usize, usize, FieldData)>,
                    ) = neighbours
                        .into_iter()
                        .map(|(nx, ny)| (nx, ny, game.field_data(nx, ny)))
                        .filter(|(_, _, field)| match field {
                            FieldData::Closed | FieldData::Flagged => true,
                            FieldData::Open(_) => false,
                        })
                        .partition(|(_, _, field)| {
                            if let FieldData::Closed = field {
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

                    let Some((nx, ny, _)) = neighbours.0.pop() else {
                        continue;
                    };

                    if mines_left > 0 {
                        println!("Flagging ({nx}, {ny})");
                    } else {
                        println!("Opening ({nx}, {ny})");
                    }
                    return Some((nx, ny, mines_left > 0));
                }
            }
        }
    }

    if closed_positions.is_empty() {
        return None;
    }

    let i = rand::random_range(0..closed_positions.len());
    let Some((x, y)) = closed_positions.get(i) else {
        println!("Could not find move to make!");
        return None;
    };

    println!("Opening randomly ({x}, {y})");
    return Some((*x, *y, false));
}
