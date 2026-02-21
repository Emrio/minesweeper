use crate::{CellConfig, MineField, solver::invariant::find_invariant_move};

#[test]
fn solve_121() {
    let game = MineField::from([
        [CellConfig::Open, CellConfig::Open, CellConfig::Open],
        [CellConfig::Mine, CellConfig::Closed, CellConfig::Mine],
    ]);

    let next_move = find_invariant_move(&game);

    match next_move {
        None => panic!("Invariant solver should find a move for 121"),
        Some((pos, true)) if pos == (0, 1).into() => {}
        Some((pos, false)) if pos == (1, 1).into() => {}
        Some((pos, true)) if pos == (2, 1).into() => {}
        _ => panic!("Invalid move"),
    }
}

#[test]
fn solve_111() {
    let game = MineField::from([
        [CellConfig::Open, CellConfig::Open, CellConfig::Open],
        [CellConfig::Closed, CellConfig::Mine, CellConfig::Closed],
    ]);

    let next_move = find_invariant_move(&game);

    match next_move {
        None => panic!("Invariant solver should find a move for 111"),
        Some((pos, false)) if pos == (0, 1).into() => {}
        Some((pos, true)) if pos == (1, 1).into() => {}
        Some((pos, false)) if pos == (2, 1).into() => {}
        _ => panic!("Invalid move"),
    }
}

#[test]
fn solve_121_angle() {
    let game = MineField::from([
        [CellConfig::Mine, CellConfig::Closed, CellConfig::Closed],
        [CellConfig::Open, CellConfig::Open, CellConfig::Mine],
        [CellConfig::Open, CellConfig::Open, CellConfig::Closed],
    ]);

    let next_move = find_invariant_move(&game);

    match next_move {
        None => panic!("Invariant solver should find a move for 121 angled"),
        Some((pos, false)) if pos == (2, 0).into() => {}
        _ => panic!("Invalid move"),
    }
}
