mod cluster;
mod shadow;

use crate::{CellData, MineField, Position};
use cluster::{ClusterCell, Clusterer};
use shadow::ShadowMinefield;

fn generate_suitable_cluster_solutions(
    shadow_minefield: &mut ShadowMinefield,
    open_positions: &Vec<Position>,
    cluster: &mut Vec<ClusterCell>,
) -> Vec<Vec<bool>> {
    let Some(cell) = cluster.pop() else {
        // println!("{shadow_minefield}");
        if open_positions
            .into_iter()
            .all(|pos| shadow_minefield.get_cell(pos).get_mines_left() == Some(0))
        {
            // println!("Valid solution!");
            return vec![Vec::new()];
        }

        // println!("invalid solution!");
        return vec![];
    };

    let mut solutions = Vec::new();

    if shadow_minefield.flag(&cell) {
        let mut sub_solutions =
            generate_suitable_cluster_solutions(shadow_minefield, open_positions, cluster);

        for sub_solution in sub_solutions.iter_mut() {
            sub_solution.push(true);
        }

        solutions.extend(sub_solutions);

        shadow_minefield.unflag(&cell);
    }

    if shadow_minefield.open(&cell) {
        let mut sub_solutions =
            generate_suitable_cluster_solutions(shadow_minefield, open_positions, cluster);

        for sub_solution in sub_solutions.iter_mut() {
            sub_solution.push(false);
        }

        solutions.extend(sub_solutions);

        shadow_minefield.unopen(&cell);
    }

    cluster.push(cell);

    solutions
}

fn find_invariant_solution(
    cluster: Vec<ClusterCell>,
    solutions: Vec<Vec<bool>>,
) -> Option<(Position, bool)> {
    // let neighbours = pos.neighbours(game);

    for (index, cell) in cluster.iter().enumerate() {
        if solutions.iter().all(|solution| solution[index]) {
            // println!("Deciding to play ({}, true)", cell.pos);
            return Some((cell.pos, true));
        }

        if solutions.iter().all(|solution| !solution[index]) {
            // println!("Deciding to play ({}, false)", cell.pos);
            return Some((cell.pos, false));
        }
    }

    None
}

fn build_cluster_and_find_move(
    shadow_minefield: &mut ShadowMinefield,
    initial_pos: Position,
) -> Option<(Position, bool)> {
    // println!("Initial");
    // println!("{shadow_minefield}");
    let mut clusterer = Clusterer::from(&shadow_minefield);
    let mut cluster = clusterer.build_cluster(initial_pos);

    let open_positions: Vec<Position> = cluster
        .to_vec()
        .into_iter()
        .flat_map(|cell| cell.open_neighbours)
        .collect();

    if cluster.is_empty() {
        return None;
    }

    let valid_solutions =
        generate_suitable_cluster_solutions(shadow_minefield, &open_positions, &mut cluster);
    println!("Cluster: {:?}", cluster);
    println!("Valid solutions: {valid_solutions:?}");

    find_invariant_solution(cluster, valid_solutions)
}

pub(super) fn find_invariant_move(game: &MineField) -> Option<(Position, bool)> {
    let mut shadow_minefield = ShadowMinefield::new(game);

    for y in 0..game.height() {
        for x in 0..game.height() {
            let pos = (x, y).into();

            match game.cell_data(pos) {
                CellData::Flagged | CellData::Closed | CellData::Open(0) => {}
                CellData::Open(_) => {
                    if let Some(next_move) = build_cluster_and_find_move(&mut shadow_minefield, pos)
                    {
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
mod tests;
