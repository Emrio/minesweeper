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

pub struct Clusterer<'a, 'b> {
    shadow_minefield: &'a ShadowMinefield<'b>,
    visited: Vec<bool>,
}

impl<'a, 'b> Clusterer<'a, 'b> {
    pub fn from(shadow_minefield: &'a ShadowMinefield<'b>) -> Self {
        let visited = vec![false; shadow_minefield.game.width() * shadow_minefield.game.height()];
        Self {
            shadow_minefield,
            visited,
        }
    }

    pub fn build_cluster(&mut self, pos: Position) -> Vec<ClusterCell> {
        let cluster = self.find_closed_positions(pos);

        cluster
            .into_iter()
            .map(|pos| ClusterCell {
                pos,
                open_neighbours: self.get_open_neighbours(pos),
            })
            .collect()
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
