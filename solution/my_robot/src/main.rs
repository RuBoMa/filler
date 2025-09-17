mod game;
mod field;
mod piece;
mod player;
mod grid;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();

    let player = player::Player::new(&first_line);

    let second_line = lines.next().unwrap().unwrap();
    let mut field = field::Field::new(&second_line);
    field.update(&mut lines);

    let g = game::Game::new(player, &field);
    loop {
        let next_line = match lines.next() {
            Some(Ok(line)) => line,
            _ => break,
        };

        if next_line.starts_with("Anfield") {
            field.update(&mut lines);
        }
        
        if next_line.starts_with("Piece") {
            let mut pi = piece::Piece::new(&next_line);
            pi.update(&mut lines);
            println!("0 0\n");
        }
    }
}
