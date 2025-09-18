pub use crate::field::*;
pub use crate::piece::*;
pub use crate::player::*;

#[derive(Debug, Clone)]
pub struct Game{
    pub player: Player,
    pub enemy: Player,
    pub field: Field,
    pub pieces: Vec<Piece>,
    pub turns: usize,
}


#[derive(Debug, Clone)]
pub struct Placement {
    pub pos: Pos,
    pub score: i32,
    pub piece: Piece,
}

impl Game {
    pub fn new(player: Player, enemy: Player, field: Field) -> Self {
        Self{
            player,
            enemy,
            field,
            pieces: Vec::new(),
            turns: 0,
        }
    }

    pub fn place_piece(&mut self, p: Piece) -> (i32, i32) {
        self.turns += 1;
        
        if p.trimmed_size.height > self.field.size.height || p.trimmed_size.width > self.field.size.width {
            return (0, 0);
        }

        let mut best: Option<Placement> = None;

        for y in p.offset.0..=self.field.size.height - p.trimmed_size.height {
            for x in p.offset.1..=self.field.size.width - p.trimmed_size.width {
                if let Some(p_valid) = self.check_placement(&p, Pos { x, y }) {
                    if best.as_ref().map_or(true, |b| p_valid.score > b.score) {
                        best = Some(p_valid);
                    }
                }
            }
        }

        self.pieces.push(p);
        if let Some(best_pos) = best {
            self.player.score += 1;
            (best_pos.pos.x as i32 - best_pos.piece.offset.1 as i32, 
                best_pos.pos.y as i32 - best_pos.piece.offset.0 as i32)
        } else {
            (0, 0)
        }
    }
    
    pub fn check_placement(&self, piece: &Piece, pos: Pos) -> Option<Placement> {
        let mut overlap = 0;

        for (dy, row) in piece.trimmed_cells.iter().enumerate() {
            for (dx, c) in row.iter().enumerate() {

                let cell = self.field.cells[dy + pos.y][dx + pos.x];
                if *c == 'O' && self.player.is_mine(&cell) {
                    overlap += 1;
                    if overlap > 1 {
                        return None;
                    }
                } else if *c == 'O' && self.enemy.is_mine(&cell) {
                    return None;
                }
            }
        }
        if overlap != 1 { return None }
        
        let mut score = 0;
        for (dy, row) in piece.trimmed_cells.iter().enumerate() {
            for (dx, &piece_cell) in row.iter().enumerate() {
                score += self.get_cell_score(piece_cell, Pos{ y: pos.y + dy, x: pos.x + dx });
            }
        }
        Some(Placement { pos, score, piece: piece.to_owned() })
    }

    pub fn get_cell_score(&self, piece_cell: char, pos: Pos) -> i32 {
        let will_place_here = piece_cell != '.';

        match will_place_here {
            false => {
                let cur_cell = self.field.cells[pos.y][pos.x];
                if self.player.is_mine(&cur_cell) { 1 }
                else if self.enemy.is_mine(&cur_cell) { 2 }
                else { 0 }
            },
            true => {
                if pos.y as i32 - 1 >= 0 && self.enemy.is_mine( &self.field.cells[pos.y-1][pos.x] ) { 4 }
                else if  pos.x as i32 - 1 >= 0 && self.enemy.is_mine( &self.field.cells[pos.y][pos.x-1] ) { 4 } 
                else if  pos.y + 1 < self.field.size.height && self.enemy.is_mine( &self.field.cells[pos.y+1][pos.x] ) { 4 } 
                else if  pos.x + 1 < self.field.size.width && self.enemy.is_mine( &self.field.cells[pos.y][pos.x+1] ) { 4 } 
                else { 0 }
            },
        }
    }

}