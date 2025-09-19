use crate::game::Game;
use crate::piece::Piece;
use crate::grid::Grid;
use crate::utils::*;
use crate::field::Field;

// ================================================================================
// Old code, remove when sure everything's been implemented under the new structure
// ================================================================================

pub struct PlacementAndScore {
    pub placement: (usize, usize),
    pub score: i32,
}

pub fn run_bot(game: &Game, piece: &Piece) -> (usize, usize) {
    let min_max_lines = find_true_piece_dimensions(piece);

    let valid_placements = find_valid_placements(game.field.clone(), piece, &min_max_lines.0, &min_max_lines.1, game.player.symbol);

    if valid_placements.is_empty() {
        return (0, 0);
    }

    let evaluated_placements = evaluate_placements(game.field.clone(), piece, valid_placements, &min_max_lines.0, &min_max_lines.1, game.player.symbol);

    get_best_score_placement(&evaluated_placements)
}

pub fn find_true_piece_dimensions(piece: &Piece) -> ((usize, usize), (usize, usize)) {
    // Find the true dimensions of the piece since sometimes pieces have extra empty rows/columns
    // Ignores empty lines between non-empty rows/columns
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    check_for_empty_lines(piece, true, piece.height(), true).iter().for_each(|empty_line_index| empty_rows.push(*empty_line_index));
    check_for_empty_lines(piece, true, piece.width(), false).iter().for_each(|empty_line_index| empty_cols.push(*empty_line_index));
    check_for_empty_lines(piece, false, piece.height(), true).iter().for_each(|empty_line_index| empty_rows.push(*empty_line_index));
    check_for_empty_lines(piece, false, piece.width(), false).iter().for_each(|empty_line_index| empty_cols.push(*empty_line_index));

    let mut min_max_rows: (usize, usize) = (0, piece.height()-1);
    let mut min_max_cols: (usize, usize) = (0, piece.width()-1);

    if empty_rows.len() > 0 {
        empty_rows.sort();
        min_max_rows = get_min_max_lines(&empty_rows, piece.height());
    }
    if empty_cols.len() > 0 {
        empty_cols.sort();
        min_max_cols = get_min_max_lines(&empty_cols, piece.width());
    }

    (min_max_rows, min_max_cols)
}

pub fn find_valid_placements(field: Field, piece: &Piece, min_max_rows: &(usize, usize), min_max_cols: &(usize, usize), player_symbol: (char, char)) -> Vec<PlacementAndScore> {
    let mut valid_placements: Vec<PlacementAndScore> = Vec::new();

    for (row_index, row) in field.cells().iter().enumerate() {
        if row_index + min_max_rows.1 >= field.height() {
            break;
        }
        for (col_index, _) in row.iter().enumerate() {
            if col_index + min_max_cols.1 >= field.width() {
                break;
            }
            if check_for_correct_overlap(field.clone(), piece, (row_index, col_index), min_max_rows, min_max_cols, player_symbol) {
                valid_placements.push(PlacementAndScore { placement: (col_index, row_index), score: 0 });
            }
        }
    }
    
    valid_placements
}

pub fn check_for_correct_overlap(field: Field, piece: &Piece, placement: (usize, usize), min_max_rows: &(usize, usize), min_max_cols: &(usize, usize), player_symbol: (char, char)) -> bool {
    let mut overlap = 0;

    for (row_index, row) in piece.cells().iter().enumerate() {
        if row_index < min_max_rows.0 || row_index > min_max_rows.1 {
            continue;
        }
        for (col_index, c) in row.iter().enumerate() {
            if col_index < min_max_cols.0 || col_index > min_max_cols.1 {
                continue;
            }
            let cell = field.cells()[row_index + placement.0][col_index + placement.1];
            if *c == 'O' && is_player_cell(Some(cell), player_symbol) {
                overlap += 1;
                if overlap > 1 {
                    return false;
                }
            } else if *c == 'O' && is_enemy_cell(Some(cell), player_symbol) {
                return false;
            }
        }
    }
    overlap == 1
}

pub fn evaluate_placements(field: Field, piece: &Piece, mut valid_placements: Vec<PlacementAndScore>, min_max_rows: &(usize, usize), min_max_cols: &(usize, usize), player_symbol: (char, char)) -> Vec<PlacementAndScore> {
    for current_placement in &mut valid_placements {
        for (row_index, row) in piece.cells().iter().enumerate() {
            if row_index < min_max_rows.0 || row_index > min_max_rows.1 {
                continue;
            }
            for (col_index, c) in row.iter().enumerate() {
                if col_index < min_max_cols.0 || col_index > min_max_cols.1 {
                    continue;
                }
                let cell = field.cells()[row_index + current_placement.placement.1][col_index + current_placement.placement.0];
                //let (next_row_cell, prev_row_cell, next_col_cell, prev_col_cell) = get_adjacent_cells(&field, current_placement.placement, row_index, col_index);

                //current_placement.score += do_score_calculation(c, cell, next_row_cell, prev_row_cell, next_col_cell, prev_col_cell, row_index, col_index, field.height(), field.width(), player_symbol);
            }
        }
    }
    valid_placements
}