use std::fmt::Display;

#[derive(Clone, PartialEq)]
enum FieldState {
    Closed,
    Open,
    Flagged,
}

#[derive(Clone)]
struct Field {
    has_bomb: bool,
    state: FieldState,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            has_bomb: false,
            state: FieldState::Closed,
        }
    }
}

#[derive(PartialEq)]
pub enum GameState {
    Ongoing,
    Won,
    Lost,
}

pub struct Minesweeper {
    width: usize,
    height: usize,
    map: Vec<Field>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mut bombs: u32) -> Self {
        let size = width * height;

        let mut map = vec![Field::default(); size];

        while bombs > 0 {
            let i = rand::random_range(0..size);

            if !map[i].has_bomb {
                map[i].has_bomb = true;
                bombs -= 1;
            }
        }

        Self { width, height, map }
    }

    pub fn play(&mut self, x: usize, y: usize, flag: bool) -> GameState {
        let field = self.get_field_mut(x, y);

        if flag {
            if field.state == FieldState::Flagged {
                field.state = FieldState::Closed;
            } else {
                field.state = FieldState::Flagged;
            }

            return GameState::Ongoing;
        }

        if field.has_bomb {
            field.state = FieldState::Open;
            return GameState::Lost;
        }

        let mut opener = Opener::new(self);
        opener.open(x, y);

        return GameState::Ongoing;
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neigbhours = Vec::new();

        if x > 0 && y > 0 {
            neigbhours.push((x - 1, y - 1));
        }

        if y > 0 {
            neigbhours.push((x, y - 1));
        }

        if x < self.width - 1 && y > 0 {
            neigbhours.push((x + 1, y - 1));
        }

        if x > 0 {
            neigbhours.push((x - 1, y));
        }

        if x < self.width - 1 {
            neigbhours.push((x + 1, y));
        }

        if x > 0 && y < self.height - 1 {
            neigbhours.push((x - 1, y + 1));
        }

        if y < self.height - 1 {
            neigbhours.push((x, y + 1));
        }

        if x < self.width - 1 && y < self.height - 1 {
            neigbhours.push((x + 1, y + 1));
        }

        neigbhours
    }

    fn get_field(&self, x: usize, y: usize) -> &Field {
        &self.map[y * self.width + x]
    }

    fn get_field_mut(&mut self, x: usize, y: usize) -> &mut Field {
        self.map.get_mut(y * self.width + x).unwrap()
    }

    fn neighbouring_mines(&self, x: usize, y: usize) -> usize {
        self.neighbours(x, y)
            .iter()
            .filter(|(nx, ny)| self.get_field(*nx, *ny).has_bomb)
            .count()
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&"—".repeat(self.width))?;

        for (y, row) in self.map.chunks(self.width).enumerate() {
            f.write_str("\n")?;
            for (x, field) in row.iter().enumerate() {
                match field.state {
                    FieldState::Closed => f.write_str("o")?,
                    FieldState::Flagged => f.write_str("F")?,
                    FieldState::Open if field.has_bomb => f.write_str("X")?,
                    FieldState::Open => {
                        let mines = self.neighbouring_mines(x, y);
                        if mines == 0 {
                            f.write_str(" ")?;
                        } else {
                            f.write_str(&mines.to_string())?;
                        }
                    }
                };

                // if field.has_bomb {
                //     f.write_str("X")?;
                // } else {
                //     let mines = self.neighbouring_mines(x, y);

                // if mines == 0 {
                //     f.write_str(" ")?;
                // } else {
                //     f.write_str(&mines.to_string())?;
                // }
                // }
            }
        }

        Ok(())
    }
}

struct Opener<'a> {
    game: &'a mut Minesweeper,
    visited: Vec<bool>,
}

impl<'a> Opener<'a> {
    fn new(game: &'a mut Minesweeper) -> Self {
        let visited = vec![false; game.map.len()];
        Self { game, visited }
    }

    fn open(&mut self, x: usize, y: usize) {
        if self.visited[x + y * self.game.width] {
            return;
        }

        self.visited[x + y * self.game.width] = true;
        self.game.get_field_mut(x, y).state = FieldState::Open;

        if self.game.neighbouring_mines(x, y) > 0 {
            return;
        }

        for (nx, ny) in self.game.neighbours(x, y) {
            self.open(nx, ny);
        }
    }
}
