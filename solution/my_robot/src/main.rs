mod game;
mod field;
mod piece;
mod player;
mod grid;

use std::io::{self, BufRead};
use game::*;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let second_line = lines.next().unwrap().unwrap();

    let (player, enemy) = Player::new(&first_line);
    let mut g = game::Game::new(
        player, enemy,
        Field::new(&second_line));
    
    g.field.update(&mut lines);
    
    loop {
        let next_line = match lines.next() {
            Some(Ok(line)) => line,
            _ => break,
        };

        if next_line.starts_with("Anfield") {
            g.field.update(&mut lines);
        }
        
        if next_line.starts_with("Piece") {
            let mut p = Piece::new(&next_line);
            p.update(&mut lines);
            let (x, y) = g.place_piece(p);
            println!("{} {}", x, y);
        }
    }
}
