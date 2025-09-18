pub use crate::field::*;
pub use crate::piece::*;
pub use crate::player::*;

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

    pub fn place_piece(&mut self, p: Piece) -> (usize, usize) {
        self.turns += 1;
        let (sym_1, sym_2) = self.player.symbol; // get player symbols
        // use piece.trimmed.rows and piece.timmed.cols to get sessions of the grid
        // maybe a field fn get_section(x, y, height, width)

        // check number of symbols in section
        // if section.symbol count < 1, move to next section
        // if section.symbol count + piece.symbol_count > height * width, move to next section
        // else try placing piece, if number of overlap symbol == 1, ok. else move to next section

        // return section x, y minus peice offset
        (0, 0)
    }
}