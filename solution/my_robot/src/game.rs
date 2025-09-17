use crate::field::*;
use crate::piece::*;
use crate::player::*;

#[derive(Debug, Clone)]
pub struct Game<'a>{
    pub player: Player,
    pub field: &'a Field,
    pub pieces: Vec<Piece>,
    pub turns: usize,
}

impl<'a> Game<'a> {
    pub fn new(player: Player, field: &'a Field) -> Self {
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