mod game;
mod field;
mod piece;
mod player;
mod grid;
mod bot_logic;
mod utils;
mod basic_test;

use std::io::{self, BufRead};

use basic_test::run_test;

fn main() {
    // For testing a specific scenario
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 && args[1] == "test" {
        run_test();
        return;
    }

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let player = player::Player::new(&first_line);

    let second_line = lines.next().unwrap().unwrap();
    let mut field = field::Field::new(&second_line);
    field.update(&mut lines);

    let mut game_instance = game::Game::new(player, field);
    loop {
        let next_line = match lines.next() {
            Some(Ok(line)) => line,
            _ => break,
        };

        if next_line.starts_with("Anfield") {
            game_instance.update_field(&mut lines);
        }
        
        if next_line.starts_with("Piece") {
            let mut piece = piece::Piece::new(&next_line);
            piece.update(&mut lines);

            let (x, y) = bot_logic::run_bot(&game_instance, &piece);
            println!("{} {}", x, y);
        }
    }
}
