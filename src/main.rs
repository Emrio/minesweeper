use minesweeper::{GameState, Minesweeper, pass};

fn main() {
    println!("Hello, world!");

    let width = 12;
    let height = 12;
    let bomb_rate = 0.2;
    let bombs = (bomb_rate * ((width * height) as f64)) as u32;

    let mut game = Minesweeper::new(width, height, bombs);

    while let Some((pos, flag)) = pass(&game) {
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
