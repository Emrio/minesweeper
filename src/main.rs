use minesweeper::{GameState, MineField, find_next_move};

fn main() {
    println!("Hello, world!");

    let width = 50;
    let height = 50;
    let mine_rate = 0.12;
    let mines = (mine_rate * ((width * height) as f64)) as u32;

    let mut game = MineField::new(width, height, mines);

    while let Some((pos, flag)) = find_next_move(&game) {
        match game.play(pos, flag) {
            GameState::Ongoing => {}
            GameState::Won => {
                println!("{}", game);
                println!("WON!");
                return;
            }
            GameState::Lost => {
                println!("{}", game);
                println!("LOST!");
                return;
            }
        };
        println!("{}", game);
    }
}
