use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::player::Player;
use crate::field::Field;
use crate::game::Game;
use crate::piece::Piece;
use crate::bot_logic;

pub fn run_test(player_number: &str) {
    if player_number == "p1" {
        println!("Running test for p1 (@ a)");
    } else {
        println!("Running test for p2 ($ s)");
    }

    let file = File::open("test_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let field_header = lines.next().unwrap().unwrap();
    let mut field = Field::new(&field_header);
    field.update(&mut lines);

    let piece_header = lines.next().unwrap().unwrap();
    let mut piece = Piece::new(&piece_header);
    piece.update(&mut lines);

    let player = Player::new(&format!("Player1 {}", player_number));
    let game_instance = Game::new(player, field);

    let (x, y) = bot_logic::run_bot(&game_instance, &piece);
    println!("{} {}\n", x, y);
}