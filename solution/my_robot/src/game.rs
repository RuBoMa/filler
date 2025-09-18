use crate::field::*;
use crate::piece::*;
use crate::player::*;
use std::io::Error;

#[derive(Debug, Clone)]
pub struct Game {
    pub player: Player,
    pub field: Field,
    pub pieces: Vec<Piece>,
    pub current_turn: usize,
}

impl Game {
    pub fn new(player: Player, field: Field) -> Self {
        Self{
            player,
            field,
            pieces: Vec::new(),
            current_turn: 0,
        }
    }

/*     pub fn process_input(&mut self, line: &str) -> Option<(usize, usize)>{
        None
    } */

    pub fn update_field<I: Iterator<Item = Result<String, Error>>>(&mut self, lines: &mut I) {
        self.field.update(lines);
    }
}