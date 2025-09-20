pub use crate::field::*;
pub use crate::piece::*;
pub use crate::player::*;
pub use crate::utils::*;

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

        let enemy_pos = get_average_pos(&self.field, self.player.symbol, true);
        // let player_pos = get_average_pos(&self.field, self.player.symbol, false);

        // Changed back to keeping a full list of placements for the sake of evaluating scores of possible placements relative to each other
        // One of the main reasons for this is to be able to evaluate how much closer a placement is getting to the enemy relative to possible placements starting from other positions
        let mut possible_placements: Vec<Placement> = Vec::new();

        for y in p.offset.0..=self.field.size.height - p.trimmed_size.height {
            for x in p.offset.1..=self.field.size.width - p.trimmed_size.width {
                if let Some(p_valid) = self.check_placement(&p, Pos { x, y }) {
                    possible_placements.push(p_valid);
                }
            }
        }

        if possible_placements.is_empty() {
            return (0, 0);
        }

        let best: Placement =  evaluate_placements(&self.field, possible_placements, enemy_pos, self.turns, self.player.symbol, &self.pieces);

        self.pieces.push(p);
        self.player.score += 1;
        (best.pos.x as i32 - best.piece.offset.1 as i32, best.pos.y as i32 - best.piece.offset.0 as i32)
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

    pub fn get_cell_score(&self, piece_cell: char, cell_pos: Pos) -> i32 {
        let will_place_here = piece_cell == 'O';

        // For clarity
        let (prev_y_cell, next_y_cell, prev_x_cell, next_x_cell) = get_adjacent_cells(&self.field, &cell_pos);

        match will_place_here {
            false => {
                let cur_cell = self.field.cells[cell_pos.y][cell_pos.x];
                if self.player.is_mine(&cur_cell) { 1 }
                else if self.enemy.is_mine(&cur_cell) { 2 }
                else { 0 }
            },
            true => {
                if prev_y_cell.is_some() &&
                    self.enemy.is_mine( &prev_y_cell.unwrap() ) { 4 }

                else if next_y_cell.is_some() &&
                    self.enemy.is_mine( &next_y_cell.unwrap() ) { 4 } 

                else if prev_x_cell.is_some() &&
                    self.enemy.is_mine( &prev_x_cell.unwrap() ) { 4 }

                else if next_x_cell.is_some() &&
                    self.enemy.is_mine( &next_x_cell.unwrap() ) { 4 } 
                else { 0 }
            },
        }
    }

}