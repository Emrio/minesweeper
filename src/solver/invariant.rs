use crate::{CellData, MineField, Position};

#[derive(Debug)]
enum ShadowCell {
    Closed,
    Open(usize),
    ShadowOpen,
    Flagged,
    ShadowFlagged,
}

impl ShadowCell {
    // pub fn is_open(&self) -> bool {
    //     self.get_mines_count().is_some()
    // }

    // pub fn get_mines_count(&self) -> Option<usize> {
    //     if let ShadowCell::Open(mines) = self {
    //         Some(*mines)
    //     } else {
    //         None
    //     }
    // }

    pub fn is_closed(&self) -> bool {
        if let ShadowCell::Closed = self {
            true
        } else {
            false
        }
    }

    pub fn is_flagged(&self) -> bool {
        match self {
            ShadowCell::Flagged | ShadowCell::ShadowFlagged => true,
            _ => false,
        }
    }
}

struct ShadowMinefield<'a> {
    game: &'a MineField,
    field: Vec<ShadowCell>,
}

impl<'a> ShadowMinefield<'a> {
    fn new(game: &'a MineField) -> Self {
        let mut field = Vec::new();
        for y in 0..game.height() {
            for x in 0..game.width() {
                let pos = (x, y).into();
                field.push(match game.cell_data(pos) {
                    CellData::Closed => ShadowCell::Closed,
                    CellData::Open(mines) => ShadowCell::Open(mines),
                    CellData::Flagged => ShadowCell::Flagged,
                });
            }
        }
        Self { game, field }
    }

    fn get_cell(&self, pos: Position) -> &ShadowCell {
        &self.field[pos.to_index(self.game.width())]
    }

    fn get_cell_mut(&mut self, pos: Position) -> Option<&mut ShadowCell> {
        self.field.get_mut(pos.to_index(self.game.width()))
    }

    pub fn get_remaining_mines(&self, pos: Position) -> usize {
        let total_mines = self.game.cell_data(pos).get_mines_count().unwrap();
        let flagged_mines = pos
            .neighbours(self.game)
            .into_iter()
            .map(|neighbour| self.get_cell(neighbour))
            .filter(|cell| cell.is_flagged())
            .count();
        // .filter_map(|neighbour| self.get_cell(neighbour).get_mines_count())

        total_mines - flagged_mines
    }

    pub fn get_closed_positions(&self, pos: Position) -> Vec<Position> {
        pos.neighbours(self.game)
            .into_iter()
            .filter(|neighbour| self.get_cell(*neighbour).is_closed())
            .collect()
    }

    pub fn flag(&mut self, pos: Position) -> bool {
        match self.field.get(pos.to_index(self.game.width())) {
            None => panic!("Invalid position"),
            Some(ShadowCell::Open(_))
            | Some(ShadowCell::ShadowOpen)
            | Some(ShadowCell::ShadowFlagged)
            | Some(ShadowCell::Flagged) => {
                panic!("Position cannot be shadow flagged")
            }
            Some(ShadowCell::Closed) => {}
        }

        for neighbour in pos.neighbours(self.game) {
            match self.get_cell(neighbour) {
                ShadowCell::Open(0) => return false,
                _ => {}
            }
        }

        for neighbour in pos.neighbours(self.game) {
            let cell = self.get_cell_mut(neighbour);

            match cell {
                Some(ShadowCell::Open(n)) => {
                    *n -= 1;
                }
                _ => {}
            }
        }

        let cell = self.get_cell_mut(pos).unwrap();
        *cell = ShadowCell::ShadowFlagged;

        true
    }

    pub fn unflag(&mut self, pos: Position) {
        match self.field.get(pos.to_index(self.game.width())) {
            None => panic!("Invalid position"),
            Some(ShadowCell::Open(_))
            | Some(ShadowCell::ShadowOpen)
            | Some(ShadowCell::Closed)
            | Some(ShadowCell::Flagged) => {
                panic!("Position cannot be shadow unflagged")
            }
            Some(ShadowCell::ShadowFlagged) => {}
        }

        for neighbour in pos.neighbours(self.game) {
            let cell = self.get_cell_mut(neighbour);

            match cell {
                Some(ShadowCell::Open(n)) => {
                    *n += 1;
                }
                _ => {}
            }
        }

        let cell = self.get_cell_mut(pos).unwrap();
        *cell = ShadowCell::Closed;
    }

    pub fn open(&mut self, pos: Position) -> bool {
        match self.field.get(pos.to_index(self.game.width())) {
            None => panic!("Invalid position"),
            Some(ShadowCell::Open(_))
            | Some(ShadowCell::ShadowOpen)
            | Some(ShadowCell::ShadowFlagged)
            | Some(ShadowCell::Flagged) => {
                panic!("Position cannot be shadow opened")
            }
            Some(ShadowCell::Closed) => {}
        }

        for neighbour in pos.neighbours(self.game) {
            println!(
                "Making sure neighbour {neighbour} is fine... {:?}",
                self.get_cell(neighbour)
            );
            match self.get_cell(neighbour) {
                ShadowCell::Open(mines)
                    if *mines > 0 && self.get_closed_positions(neighbour).len() < *mines =>
                {
                    println!(
                        "Cell {neighbour} has {mines} mines left! It has {} closed positions in its vicinity",
                        self.get_closed_positions(neighbour).len()
                    );
                    return false;
                }
                _ => {}
            }
        }

        let cell = self.get_cell_mut(pos).unwrap();
        *cell = ShadowCell::ShadowOpen;

        true
    }

    pub fn unopen(&mut self, pos: Position) {
        match self.field.get(pos.to_index(self.game.width())) {
            None => panic!("Invalid position"),
            Some(ShadowCell::Open(_))
            | Some(ShadowCell::ShadowFlagged)
            | Some(ShadowCell::Closed)
            | Some(ShadowCell::Flagged) => {
                panic!("Position cannot be shadow opened")
            }
            Some(ShadowCell::ShadowOpen) => {}
        }

        let cell = self.get_cell_mut(pos).unwrap();
        *cell = ShadowCell::Closed;
    }
}

fn find_suitable_mine_disposition(
    pos: Position,
    game: &mut ShadowMinefield,
) -> Vec<Vec<(Position, bool)>> {
    let mines = game.get_remaining_mines(pos);
    println!("Remaining mines: {mines}");
    let closed_positions = game.get_closed_positions(pos);

    if mines == 0 {
        for opened_position in closed_positions.clone() {
            if !game.open(opened_position) {
                println!("Unsuccessful opening of position {opened_position}");
                return vec![];
            }

            println!("Successful opening of position {opened_position}");
        }

        for opened_position in closed_positions.clone() {
            game.unopen(opened_position);
        }

        return vec![
            closed_positions
                .into_iter()
                .map(|position| (position, false))
                .collect(),
        ];
    }

    println!("Closed positions: {closed_positions:?}");

    let mut valid_tuples = Vec::new();

    for flagged_position in closed_positions {
        if game.flag(flagged_position) {
            println!("Shadow flag successful {flagged_position}");
            let mut sub_solutions = find_suitable_mine_disposition(pos, game);

            for sub_solution in sub_solutions.iter_mut() {
                sub_solution.push((flagged_position, true));
            }

            valid_tuples.extend(sub_solutions);

            game.unflag(flagged_position);
        } else {
            println!("Shadow flag unsuccessful {flagged_position}")
        }
    }

    valid_tuples
}

fn find_invariant_mine(
    pos: Position,
    game: &MineField,
    solutions: Vec<Vec<(Position, bool)>>,
) -> Option<(Position, bool)> {
    let neighbours = pos.neighbours(game);

    for neighbour in neighbours {
        if solutions
            .iter()
            .all(|solution| solution.contains(&(neighbour, true)))
        {
            println!("Deciding to play ({neighbour}, true)");
            return Some((neighbour, true));
        }

        if solutions
            .iter()
            .all(|solution| solution.contains(&(neighbour, false)))
        {
            println!("Deciding to play ({neighbour}, false)");
            return Some((neighbour, false));
        }
    }

    None
}

fn try_find_mine_around(pos: Position, game: &MineField) -> Option<(Position, bool)> {
    println!("{}", game);
    println!("Trying to find something for {pos}");

    let mut shadow_minefield = ShadowMinefield::new(game);

    let valid_solutions = find_suitable_mine_disposition(pos, &mut shadow_minefield);

    println!(
        "valid solutions: {:?} ({})",
        valid_solutions,
        valid_solutions.len()
    );

    find_invariant_mine(pos, game, valid_solutions)
}

pub(super) fn find_invariant_move(game: &MineField) -> Option<(Position, bool)> {
    for y in 0..game.height() {
        for x in 0..game.height() {
            let pos = (x, y).into();

            match game.cell_data(pos) {
                CellData::Flagged | CellData::Closed | CellData::Open(0) => {}
                CellData::Open(_) => {
                    if let Some(next_move) = try_find_mine_around(pos, game) {
                        println!("[invariant] Playing {next_move:?}");
                        return Some(next_move);
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
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
            Some((pos, true)) if pos == (2, 1).into() => {}
            // Some((pos, false)) if pos == (1, 1).into() => {}
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
            Some((pos, true)) if pos == (1, 1).into() => {}
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
}
