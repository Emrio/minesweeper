use crate::Position;

use super::shadow::ShadowMinefield;

#[derive(Clone)]
pub struct ClusterCell {
    pub pos: Position,
    pub open_neighbours: Vec<Position>,
}

impl std::fmt::Debug for ClusterCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pos.fmt(f)
    }
}

pub struct Cluster<'a, 'b> {
    pub shadow_minefield: &'a mut ShadowMinefield<'b>,
    pub closed_positions: Vec<ClusterCell>,
    pub open_positions: Vec<Position>,
}

impl<'a, 'b> Cluster<'a, 'b> {
    pub fn from(shadow_minefield: &'a mut ShadowMinefield<'b>, initial_pos: Position) -> Self {
        let mut clusterer = Clusterer::from(shadow_minefield);

        let closed_positions: Vec<ClusterCell> = clusterer
            .find_closed_positions(initial_pos)
            .into_iter()
            .map(|pos| ClusterCell {
                pos,
                open_neighbours: clusterer.get_open_neighbours(pos),
            })
            .collect();

        let open_positions: Vec<Position> = closed_positions
            .to_vec()
            .into_iter()
            .flat_map(|cell| cell.open_neighbours)
            .collect();

        Self {
            shadow_minefield,
            closed_positions,
            open_positions,
        }
    }
}

impl<'a, 'b> std::fmt::Display for Cluster<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&"—".repeat(self.shadow_minefield.game.width()))?;

        for (y, row) in self
            .shadow_minefield
            .field
            .chunks(self.shadow_minefield.game.width())
            .enumerate()
        {
            f.write_str("\n")?;
            for (x, cell) in row.iter().enumerate() {
                let pos = (x, y).into();

                if self
                    .closed_positions
                    .iter()
                    .find(|c| c.pos == pos)
                    .is_some()
                {
                    cell.fmt(f)?
                } else if self.open_positions.contains(&pos) {
                    cell.fmt(f)?
                } else {
                    f.write_str(" ")?
                }
            }
        }

        Ok(())
    }
}

struct Clusterer<'a, 'b> {
    shadow_minefield: &'a ShadowMinefield<'b>,
    visited: Vec<bool>,
}

impl<'a, 'b> Clusterer<'a, 'b> {
    fn from(shadow_minefield: &'a ShadowMinefield<'b>) -> Self {
        let visited = vec![false; shadow_minefield.game.width() * shadow_minefield.game.height()];
        Self {
            shadow_minefield,
            visited,
        }
    }

    fn get_open_neighbours(&self, pos: Position) -> Vec<Position> {
        pos.neighbours(self.shadow_minefield.game)
            .into_iter()
            .filter(|neighbour| self.shadow_minefield.get_cell(neighbour).is_open())
            .collect()
    }

    fn find_closed_positions(&mut self, pos: Position) -> Vec<Position> {
        self.visited[pos.to_index(self.shadow_minefield.game.width())] = true;

        let is_closed = self.shadow_minefield.get_cell(&pos).is_closed();
        let is_open = self.shadow_minefield.get_cell(&pos).is_open();

        let mut cluster = Vec::new();
        if is_closed {
            cluster.push(pos)
        } else if is_open {
        } else {
            unreachable!()
        }

        for neighbour in pos.neighbours(self.shadow_minefield.game) {
            if !self.visited[neighbour.to_index(self.shadow_minefield.game.width())]
                && ((self.shadow_minefield.get_cell(&neighbour).is_closed() && is_open)
                    || (self.shadow_minefield.get_cell(&neighbour).is_open() && is_closed))
            {
                cluster.extend(self.find_closed_positions(neighbour))
            }
        }

        cluster
    }
}
