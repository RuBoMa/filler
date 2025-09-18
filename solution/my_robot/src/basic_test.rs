use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::player::Player;
use crate::field::Field;
use crate::game::Game;
use crate::piece::Piece;
use crate::bot_logic;

pub fn run_test() {
    let file = File::open("test_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let _first_line = lines.next().unwrap().unwrap(); // $$$ exec p1
    let _second_line = lines.next().unwrap().unwrap(); // $$$ exec p2

    let field_header = lines.next().unwrap().unwrap(); // Anfield 40 30:
    let mut field = Field::new(&field_header);
    field.update(&mut lines);

    let piece_header = lines.next().unwrap().unwrap(); // Piece 5 5:
    let mut piece = Piece::new(&piece_header);
    piece.update(&mut lines);

    let player = Player::new("Player1 @ $");
    let game_instance = Game::new(player, field);

    let (x, y) = bot_logic::run_bot(&game_instance, &piece);
    println!("{} {}\n", x, y);
}