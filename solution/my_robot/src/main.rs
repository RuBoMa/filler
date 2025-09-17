mod game;
mod field;
mod piece;
mod player;
mod grid;
use crate::{game::Game, player::Player, grid::Grid};

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();

    let p = player::Player::new(&first_line);

    let second_line = lines.next().unwrap().unwrap();
    let mut f = field::Field::new(&second_line);
    f.update(&mut lines);

    let piece_line = lines.next().unwrap().unwrap();
    let mut p = piece::Piece::new(&piece_line);
    p.update(&mut lines);

    loop {
        let next_line = match lines.next() {
            Some(Ok(line)) => line,
            _ => break,
        };

        if next_line.starts_with("Anfield") {
            f.update(&mut lines);
        }
        
        if next_line.starts_with("Piece") {
            let mut p = piece::Piece::new(&next_line);
            p.update(&mut lines);
            // after reading a piece, we should try and place it and print the coordinates
            println!("{}", p);
        }
    }
}
