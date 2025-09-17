use crate::field::*;
use crate::piece::*;
use crate::player::*;

#[derive(Debug, Clone)]
pub struct Game{
    pub player: Player,
    pub field: Field,
    pub pieces: Vec<Piece>,
    pub turns: usize,
}

impl Game {
    pub fn new(player: Player, field: Field) -> Self {
        Self{
            player,
            field,
            pieces: Vec::new(),
            turns: 0,
        }
    }

    pub fn process_input(&mut self, line: &str) -> Option<(usize, usize)>{
        None
    }
}