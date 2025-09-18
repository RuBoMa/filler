use crate::piece::Piece;
use crate::grid::Grid;
use std::iter::Iterator;
use crate::field::Field;
use crate::bot_logic::PlacementAndScore;

pub fn check_for_empty_lines(piece: &Piece, start_at_min: bool, line_length: usize, checking_rows: bool) -> Vec<usize> {
    let mut empty_lines: Vec<usize> = Vec::new();

    // Box because otherwise the .rev() version is a different type
    let iter: Box<dyn Iterator<Item = usize>> = if start_at_min {
        Box::new(0..line_length) // Top -> Bottom or Left -> Right
    } else {
        Box::new((0..line_length).rev()) // Bottom -> Top or Right -> Left
    };

    for i in iter {
        let is_empty = if checking_rows {
            piece.cells()[i].iter().all(|&c| c == '.')
        } else {
            piece.cells().iter().all(|row| row[i] == '.')
        };
        
        if is_empty {
            empty_lines.push(i);
        } else {
            break;
        }
    }

    empty_lines
}

pub fn get_min_max_lines(empty_lines: &Vec<usize>, piece_size: usize) -> (usize, usize) {
    let mut min_line = 0;
    let mut max_line = piece_size-1;
    let mut previous_empty = None;

    for empty_line in empty_lines {
        if previous_empty.is_some() && empty_line - previous_empty.unwrap() > 1 {
            max_line = empty_line-1;
            break;
        } else if previous_empty.is_none() && *empty_line > 0 {
            max_line = empty_line-1;
            break;
        }
        min_line = *empty_line;
        previous_empty = Some(empty_line);
    }

    if previous_empty.is_some() {
        min_line += 1;
    }

    (min_line, max_line)
}

pub fn get_adjacent_cells(field: &Field, placement: (usize, usize), row_index: usize, col_index: usize) -> (Option<char>, Option<char>, Option<char>, Option<char>) {
    let mut next_row_cell: Option<char> = None;
    let mut prev_row_cell: Option<char> = None;
    let mut next_col_cell: Option<char> = None;
    let mut prev_col_cell: Option<char> = None;

    if placement.1 + row_index > 0 {
        prev_row_cell = Some(field.cells()[placement.1 + row_index - 1][placement.0 + col_index]);
    }
    if placement.1 + row_index + 1 < field.row_count() {
        next_row_cell = Some(field.cells()[placement.1 + row_index + 1][placement.0 + col_index]);
    }
    if placement.0 + col_index > 0 {
        prev_col_cell = Some(field.cells()[placement.1 + row_index][placement.0 + col_index - 1]);
    }
    if placement.0 + col_index + 1 < field.col_count() {
        next_col_cell = Some(field.cells()[placement.1 + row_index][placement.0 + col_index + 1]);
    }

    (next_row_cell, prev_row_cell, next_col_cell, prev_col_cell)
}

pub fn do_score_calculation(c: &char, cell: char, next_row_cell: Option<char>, prev_row_cell: Option<char>, next_col_cell: Option<char>, prev_col_cell: Option<char>, player_symbol: (char, char)) -> i32 {
    let mut score = 0;
    if *c != 'O' && is_player_cell(Some(cell), player_symbol) {
        // Alright to place close to other player cells
        score += 1;
    } else if *c != 'O' && is_enemy_cell(Some(cell), player_symbol) {
        // Good to place close to enemy cells (contesting territory)
        score += 2;
    } else if *c == 'O' && is_enemy_cell(next_row_cell, player_symbol) {
        // Great to place very close to enemy cells (contesting territory even more)
        score += 4;
    } else if *c == 'O' && is_enemy_cell(next_col_cell, player_symbol) {
        // Great to place very close to enemy cells (contesting territory even more)
        score += 4;
    } else if *c == 'O' && is_player_cell(prev_row_cell, player_symbol) {
        // Great to place very close to enemy cells (contesting territory even more)
        score += 4;
    } else if *c == 'O' && is_enemy_cell(prev_col_cell, player_symbol) {
        // Great to place very close to enemy cells (contesting territory even more)
        score += 4;
    }
    score
}

pub fn is_enemy_cell(cell: Option<char>, player_symbol: (char, char)) -> bool {
    if cell.is_none() || cell.unwrap() == '.' || cell.unwrap() == player_symbol.0 || cell.unwrap() == player_symbol.1 {
        return false;
    }
    true
}

pub fn is_player_cell(cell: Option<char>, player_symbol: (char, char)) -> bool {
    if cell.is_some() && (cell.unwrap() == player_symbol.0 || cell.unwrap() == player_symbol.1) {
        return true;
    }
    false
}

pub fn is_empty_cell(cell: Option<char>) -> bool {
    if cell.is_some() && cell.unwrap() == '.' {
        return true;
    }
    false
}

pub fn get_best_score_placement(valid_placements: &Vec<PlacementAndScore>) -> (usize, usize) {
    valid_placements.iter().max_by_key(|placement| placement.score).unwrap().placement
}