mod game;
mod field;
mod piece;
mod player;
mod grid;
mod bot_logic;
mod utils;
mod basic_test;

use std::io::{self, BufRead};
use game::*;

use basic_test::run_test;

fn main() {
    // For testing a specific scenario
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 && args[1] == "test" {
        if args.len() > 2 && (args[2] == "p1" || args[2] == "p2") {
            println!("Running test...");
            run_test(&args[2]);
        } else {
            println!("Invalid arguments, must run with p1 or p2 as second argument");
        }
        return;
    }

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let second_line = lines.next().unwrap().unwrap();

    let (player, enemy) = Player::new(&first_line);
    let mut g = Game::new(
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

            //let (x, y) = bot_logic::run_bot(&g, &p);
            let (x, y) = g.place_piece(p);
            println!("{} {}", x, y);
        }
    }
}
