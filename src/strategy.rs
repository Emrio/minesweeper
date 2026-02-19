use crate::{FieldData, Minesweeper, Position};

pub fn pass(game: &Minesweeper) -> Option<(Position, bool)> {
    immediate_move(game).or_else(|| random_move(game))
}

fn immediate_move(game: &Minesweeper) -> Option<(Position, bool)> {
    for y in 0..game.height() {
        for x in 0..game.height() {
            let pos = (x, y).into();

            match game.field_data(pos) {
                FieldData::Flagged | FieldData::Closed => {}
                FieldData::Open(mines_around) => {
                    let neighbours = game.neighbours(pos);
                    let mut neighbours: (Vec<(Position, FieldData)>, Vec<(Position, FieldData)>) =
                        neighbours
                            .into_iter()
                            .map(|neighbour| (neighbour, game.field_data(neighbour)))
                            .filter(|(_, field)| match field {
                                FieldData::Closed | FieldData::Flagged => true,
                                FieldData::Open(_) => false,
                            })
                            .partition(|(_, field)| {
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

    return None;
}

fn random_move(game: &Minesweeper) -> Option<(Position, bool)> {
    let mut closed_positions = Vec::new();

    for y in 0..game.height() {
        for x in 0..game.height() {
            let pos = (x, y).into();

            match game.field_data(pos) {
                FieldData::Closed => {
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
    return Some((*pos, false));
}
