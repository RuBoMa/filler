use crate::game::Game;
use crate::piece::Piece;
use crate::grid::Grid;
use crate::utils::check_for_empty_lines;
use crate::field::Field;
use crate::utils::get_min_max_lines;

pub fn run_bot(game: &Game, piece: &Piece) -> (usize, usize) {
    let min_max_lines = find_true_piece_dimensions(piece);

    let valid_placements = find_valid_placements(game.field.clone(), piece, &min_max_lines.0, &min_max_lines.1, game.player.symbol);
    /* println!("Found {} valid placements", valid_placements.len()); */

    if valid_placements.is_empty() {
        /* println!("No valid placements found, returning (0, 0)"); */
        return (0, 0);
    }

/*     for placement in &valid_placements {
        println!("Found valid placement at ({}, {})", placement.0, placement.1);
    } */

    (valid_placements[valid_placements.len()-1].0, valid_placements[valid_placements.len()-1].1)
}

pub fn find_true_piece_dimensions(piece: &Piece) -> ((usize, usize), (usize, usize)) {
    // Find the true dimensions of the piece since sometimes pieces have extra empty rows/columns
    // Ignores empty lines between non-empty rows/columns
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    check_for_empty_lines(piece, true, piece.row_count(), true).iter().for_each(|empty_line_index| empty_rows.push(*empty_line_index));
    check_for_empty_lines(piece, true, piece.col_count(), false).iter().for_each(|empty_line_index| empty_cols.push(*empty_line_index));
    check_for_empty_lines(piece, false, piece.row_count(), true).iter().for_each(|empty_line_index| empty_rows.push(*empty_line_index));
    check_for_empty_lines(piece, false, piece.col_count(), false).iter().for_each(|empty_line_index| empty_cols.push(*empty_line_index));

    let mut min_max_rows: (usize, usize) = (0, piece.row_count()-1);
    let mut min_max_cols: (usize, usize) = (0, piece.col_count()-1);

    if empty_rows.len() > 0 {
        empty_rows.sort();
        min_max_rows = get_min_max_lines(&empty_rows, piece.row_count());
    }
    if empty_cols.len() > 0 {
        empty_cols.sort();
        min_max_cols = get_min_max_lines(&empty_cols, piece.col_count());
    }

    (min_max_rows, min_max_cols)
}

pub fn find_valid_placements(field: Field, piece: &Piece, min_max_rows: &(usize, usize), min_max_cols: &(usize, usize), player_symbol: (char, char)) -> Vec<(usize, usize)> {
    let mut valid_placements: Vec<(usize, usize)> = Vec::new();

    for (row_index, row) in field.cells().iter().enumerate() {
        if row_index + min_max_rows.1 >= field.row_count() {
            break;
        }
        for (col_index, _) in row.iter().enumerate() {
            if col_index + min_max_cols.1 >= field.col_count() {
                break;
            }
            if check_for_correct_overlap(field.clone(), piece, (row_index, col_index), min_max_rows, min_max_cols, player_symbol) {
                /* println!("Found valid placement at ({}, {})", row_index, col_index); */
                valid_placements.push((col_index, row_index));
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
            if (*c == 'O') && 
            (cell == player_symbol.0 || cell == player_symbol.1) {
                overlap += 1;
                if overlap > 1 {
                    return false;
                }
            } else if *c == 'O' && cell != '.' {
                return false;
            }
        }
    }
    overlap == 1
}