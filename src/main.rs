use minesweeper::{GameState, Minesweeper};

fn main() {
    println!("Hello, world!");

    let mut game = Minesweeper::new(212, 212, 5_000);

    loop {
        let x = rand::random_range(0..212);
        let y = rand::random_range(0..212);

        if game.play(x, y, false) == GameState::Lost {
            break;
        }
        println!("{}", game);
    }
    println!("{}", game);
}
