use crate::piece::Piece;
use crate::grid::Grid;
use crate::field::Field;
use crate::game::{Pos, Placement};

use std::iter::Iterator;

pub fn check_for_empty_lines(
    piece: &Piece,
    start_at_min: bool,
    line_length: usize,
    checking_rows: bool,
) -> Vec<usize> {
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
    let mut max_line = piece_size - 1;
    let mut previous_empty = None;

    for empty_line in empty_lines {
        if previous_empty.is_some() && empty_line - previous_empty.unwrap() > 1 {
            max_line = empty_line - 1;
            break;
        } else if previous_empty.is_none() && *empty_line > 0 {
            max_line = empty_line - 1;
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

pub fn get_average_pos(field: &Field, player_symbol: (char, char), looking_for_enemy: bool) -> Pos {
    let mut average_pos: Pos = Pos { y: 0, x: 0 };
    let mut all_pos: Vec<Pos> = Vec::new();
    let mut cell_count: usize = 0;
    for y in 0..field.size.height {
        for x in 0..field.size.width {
            let cell = Some(field.cells[y][x]);
            if looking_for_enemy && is_enemy_cell(cell, player_symbol) {
                all_pos.push(Pos { y, x });
                cell_count += 1;
            } else if !looking_for_enemy && is_player_cell(cell, player_symbol) {
                all_pos.push(Pos { y, x });
                cell_count += 1;
            }
        }
    }
    for pos in all_pos.iter() {
        average_pos.x += pos.x;
        average_pos.y += pos.y;
    }
    average_pos.x /= cell_count;
    average_pos.y /= cell_count;
    average_pos
}


pub fn get_adjacent_cells(field: &Field, placement: &Pos) -> (Option<char>, Option<char>, Option<char>, Option<char>) {
    let mut prev_y_cell: Option<char> = None;
    let mut next_y_cell: Option<char> = None;
    let mut prev_x_cell: Option<char> = None;
    let mut next_x_cell: Option<char> = None;

    if placement.y > 0 {
        prev_y_cell = Some(field.cells()[placement.y - 1][placement.x]);
    }
    if placement.y + 1 < field.height() {
        next_y_cell = Some(field.cells()[placement.y + 1][placement.x]);
    }
    if placement.x > 0 {
        prev_x_cell = Some(field.cells()[placement.y][placement.x - 1]);
    }
    if placement.x + 1 < field.width() {
        next_x_cell = Some(field.cells()[placement.y][placement.x + 1]);
    }

    (prev_y_cell, next_y_cell, prev_x_cell, next_x_cell)
}

pub fn do_score_calculation(
    c: &char,
    cell: char,
    next_row_cell: Option<char>,
    prev_row_cell: Option<char>,
    next_col_cell: Option<char>,
    prev_col_cell: Option<char>,
    row_index: usize,
    col_index: usize,
    field_row_count: usize,
    field_col_count: usize,
    player_symbol: (char, char),
) -> i32 {
    let mut score = 0;
    let is_on_edge = if row_index == 0
        || row_index == field_row_count - 1
        || col_index == 0
        || col_index == field_col_count - 1
    {
        true
    } else {
        false
    };
    let will_place_here = if *c == 'O' { true } else { false };
    if !will_place_here && is_player_cell(Some(cell), player_symbol) {
        // Alright to place close to other player cells
        score += 1;
    } else if !will_place_here && is_enemy_cell(Some(cell), player_symbol) {
        // Good to place close to enemy cells (contesting territory)
        score += 2;
    } else if will_place_here && is_enemy_cell(next_row_cell, player_symbol) {
        // Great to place very close to enemy cells (contesting territory even more)
        score += 4;
    } else if will_place_here && is_enemy_cell(next_col_cell, player_symbol) {
        // Great to place very close to enemy cells (contesting territory even more)
        score += 4;
    } else if will_place_here && is_enemy_cell(prev_row_cell, player_symbol) {
        // Great to place very close to enemy cells (contesting territory even more)
        score += 4;
    } else if will_place_here && is_enemy_cell(prev_col_cell, player_symbol) {
        // Great to place very close to enemy cells (contesting territory even more)
        score += 4;
    }

    // Currently reduces score very slightly against terminator, but might be useful if built more properly
    /* if will_place_here && is_on_edge &&
    (is_enemy_cell(next_row_cell, player_symbol) || is_enemy_cell(next_col_cell, player_symbol) || is_enemy_cell(prev_row_cell, player_symbol) || is_enemy_cell(prev_col_cell, player_symbol)) {
        // Cutting off enemy entirely
        score += 10;
    } */

    score
}

pub fn evaluate_placements(field: &Field, mut valid_placements: Vec<Placement>, enemy_pos: Pos, current_turn: usize, player_symbol: (char, char), prev_pieces: &Vec<Piece>) -> Placement {
    let has_touched_enemy_cell = check_if_touching_enemy_cell(&field, player_symbol);
    if !has_touched_enemy_cell {
        evaluate_placement_for_enemy_distance(&field, &mut valid_placements, enemy_pos, current_turn);
    } else {
        evaluate_placement_for_wall_distance(&field, &mut valid_placements);
    }
    let found_enclosing_cells = evaluate_placement_for_enclosing_cells(&field, &mut valid_placements, player_symbol);
    if !found_enclosing_cells {
        evaluate_placement_for_perfect_fit(&field, &mut valid_placements, current_turn, prev_pieces, player_symbol);
    }

    valid_placements.into_iter().max_by_key(|placement| placement.score).unwrap()
}

pub fn check_if_touching_enemy_cell(field: &Field, player_symbol: (char, char)) -> bool {
    for y in 0..field.height() {
        for x in 0..field.width() {
            let cell = field.cells[y][x];
            if is_enemy_cell(Some(cell), player_symbol) {
                let adjacent_cells = get_adjacent_cells(field, &Pos { y, x });
                if (adjacent_cells.0.is_some() && is_player_cell(adjacent_cells.0, player_symbol)) ||
                (adjacent_cells.1.is_some() && is_player_cell(adjacent_cells.1, player_symbol)) ||
                (adjacent_cells.2.is_some() && is_player_cell(adjacent_cells.2, player_symbol)) ||
                (adjacent_cells.3.is_some() && is_player_cell(adjacent_cells.3, player_symbol)) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn evaluate_placement_for_enemy_distance(field: &Field, placements: &mut Vec<Placement>, enemy_pos: Pos, current_turn: usize) {
    // Evaluating whether the placement is closing in on the enemy or not
    // Less important late-game (high current_turn)
    let mut best_placements: Vec<(f32, usize)> = Vec::new();
    let mut current_score_addition = ((36.0 * (2.0 as f32).powf(-0.15 * current_turn as f32)) as i32).max(4);
    let mut score_list = Vec::new();

    while current_score_addition > 1 {
        score_list.push(current_score_addition);
        current_score_addition /= 2;
    }

    for (i, placement) in placements.iter().enumerate() {
        let center = get_center_of_piece(field, &placement.pos, &placement.piece);
        let distance = (((center.x).abs_diff(enemy_pos.x).pow(2) + (center.y).abs_diff(enemy_pos.y).pow(2)) as f32).sqrt();
        best_placements.push((distance, i));
    }

    best_placements.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    best_placements.truncate(score_list.len());

    let mut score_index = 0;
    for (_, index) in best_placements {
        // println!("Distance calc adding {} score to placement at pos {:?}", score_list[score_index], placements[index].pos);
        placements[index].score += score_list[score_index];
        score_index += 1;
    }
}

pub fn evaluate_placement_for_wall_distance(field: &Field, placements: &mut Vec<Placement>) {
    // Evaluating whether the placement is closing in on the wall or not
    for placement in placements {
        let max_points = 24;
        let center = get_center_of_piece(field, &placement.pos, &placement.piece);

        // Will add a check for whether or not it'd be preferable to move to the wall or not
        // Right now this reduces score a lot, but next time I work on this I'll add checks for whether or not closing in on the wall actually would cut off the enemy or not

        let distance_to_wall_up = (center.y).abs_diff(0);
        let distance_to_wall_down = (center.y).abs_diff(field.height() - 1);
        let distance_to_wall_left = (center.x).abs_diff(0);
        let distance_to_wall_right = (center.x).abs_diff(field.width() - 1);

        let distance_to_wall = distance_to_wall_up.min(distance_to_wall_down).min(distance_to_wall_left).min(distance_to_wall_right);

        placement.score += max_points / (distance_to_wall + 1) as i32;
    }
}

pub fn evaluate_placement_for_perfect_fit(field: &Field, placements: &mut Vec<Placement>, current_turn: usize, prev_pieces: &Vec<Piece>, player_symbol: (char, char)) {
    // Evaluating whether the placement perfectly fills gaps in the field
    // More important late-game (high current_turn)
    let current_score_addition = ((1.07 as f32).powf(current_turn as f32) as i32).min(50);

    for placement in placements {
        let mut is_perfect_fit = true;
        let top_left: Pos = placement.pos.clone();
        let bottom_right: Pos = Pos { x: placement.pos.x + placement.piece.size.width, y: placement.pos.y + placement.piece.size.height };
        'piece_loop: for y in top_left.y..bottom_right.y {
            for x in top_left.x..bottom_right.x {
                if y >= field.height() || x >= field.width() {
                    continue;
                }
                let field_cell = field.cells[y][x];
                let piece_cell = placement.piece.cells()[y - top_left.y][x - top_left.x];
                if field_cell == '.' && piece_cell == '.' {
                    is_perfect_fit = false;
                    break 'piece_loop;
                }
            }
        }
        if is_perfect_fit {
            let chance_next_piece_would_fit = check_if_prev_pieces_would_fit(field, &placement.piece, &placement.pos, prev_pieces, player_symbol);
            let score_multiplier = chance_next_piece_would_fit.powf(4.0);
            // println!("Perfect fit adding {} score to placement at pos {:?}", current_score_addition, placement.pos);
            placement.score += (current_score_addition as f32 * score_multiplier) as i32;
        }
    }
}

pub fn evaluate_placement_for_enclosing_cells(field: &Field, placements: &mut Vec<Placement>, player_symbol: (char, char)) -> bool {
    // Evaluating whether any of the cells are enclosing other cells
    let mut found_enclosing_cells = false;
    let enclosing_score_addition = 12;

    let enclosing_positions = get_enclosing_positions(field, player_symbol);

    for placement in placements {
        let top_left: Pos = placement.pos.clone();
        let bottom_right: Pos = Pos { x: placement.pos.x + placement.piece.size.width, y: placement.pos.y + placement.piece.size.height };
        for y in top_left.y..bottom_right.y {
            for x in top_left.x..bottom_right.x {
                let piece_cell = placement.piece.cells()[y - top_left.y][x - top_left.x];
                if y >= field.height() || x >= field.width() || piece_cell != 'O' {
                    continue;
                }
                let mut min_score_divider = 3;
                for enclosing_position in &enclosing_positions {
                    if enclosing_position.0.y == y && enclosing_position.0.x == x {
                        if enclosing_position.1 < min_score_divider {
                            min_score_divider = enclosing_position.1;
                            if min_score_divider == 1 {
                                found_enclosing_cells = true;
                                break;
                            }
                        }
                    }
                }
                // println!("Enclosing cells adding {} score to placement at pos {:?}", enclosing_score_addition / min_score_divider as i32, placement.pos);
                placement.score += enclosing_score_addition / min_score_divider as i32;
            }
        }
    }
    found_enclosing_cells
}

pub fn is_enemy_cell(cell: Option<char>, player_symbol: (char, char)) -> bool {
    if cell.is_none()
        || cell.unwrap() == '.'
        || cell.unwrap() == player_symbol.0
        || cell.unwrap() == player_symbol.1
    {
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

pub fn get_center_of_piece(field: &Field, placement: &Pos, piece: &Piece) -> Pos {
    let mut x = placement.x + (piece.trimmed_size.width + piece.offset.1)/2;
    let mut y = placement.y + (piece.trimmed_size.height + piece.offset.0)/2;
    while x >= field.width() {
        x -= 1;
    }
    while y >= field.height() {
        y -= 1;
    }
    while x < 0 {
        x += 1;
    }
    while y < 0 {
        y += 1;
    }
    Pos { y, x }
}

pub fn get_enclosing_positions(field: &Field, player_symbol: (char, char)) -> Vec<(Pos, usize)> {
    // Finding all positions that would enclose an enemy cell, as well as a score divider (starting from 1, going up to 3) (how many cells are between the current cell and the enemy cell)
    let mut enclosing_positions: Vec<(Pos, usize)> = Vec::new();
    for y in 0..field.height() {
        for x in 0..field.width() {
            let cell = field.cells[y][x];
            if is_enemy_cell(Some(cell), player_symbol) {
                for steps_away_index in 1..=3 {
                    if y as i32 - steps_away_index as i32 >= 0 {
                        let above_cell = field.cells[y - steps_away_index][x];
                        if above_cell == '.' {
                            enclosing_positions.push((Pos { y: y - steps_away_index, x }, steps_away_index));
                        }
                    }
                    if y + steps_away_index < field.height() {
                        let below_cell = field.cells[y + steps_away_index][x];
                        if below_cell == '.' {
                            enclosing_positions.push((Pos { y: y + steps_away_index, x }, steps_away_index));
                        }
                    }
                    if x as i32 - steps_away_index as i32 >= 0 {
                        let left_cell = field.cells[y][x - steps_away_index];
                        if left_cell == '.' {
                            enclosing_positions.push((Pos { y, x: x - steps_away_index }, steps_away_index));
                        }
                    }
                    if x + steps_away_index < field.width() {
                        let right_cell = field.cells[y][x + steps_away_index];
                        if right_cell == '.' {
                            enclosing_positions.push((Pos { y, x: x + steps_away_index }, steps_away_index));
                        }
                    }
                }
            }
        }
    }
    enclosing_positions
}

pub fn check_if_prev_pieces_would_fit(field: &Field, piece: &Piece, pos: &Pos, prev_pieces: &Vec<Piece>, player_symbol: (char, char)) -> f32 {
    // Checks whether any of the previous pieces would fit in the field if the current piece was placed here in order to predict whether the current placement is risky or not
    // This obviously can't predict perfectly, which is why I return a score multiplier that roughly represents the percentage of the next piece fitting
    let score_multiplier: f32;
    let mut next_turn_field = field.clone();
    let total_prev_pieces = prev_pieces.len();
    let mut placable_prev_pieces = 0;

    for y in 0..next_turn_field.height() {
        for x in 0..next_turn_field.width() {
            if (y as i32 - pos.y as i32) < 0 || (x as i32 - pos.x as i32) < 0 || y - pos.y >= piece.size.height || x - pos.x >= piece.size.width {
                continue;
            }
            let piece_cell = piece.cells()[y - pos.y][x - pos.x];
            if piece_cell == 'O' {
                next_turn_field.cells_mut()[y][x] = player_symbol.0;
            }
        }
    }

    for prev_piece in prev_pieces {
        let mut would_fit = false;
        'field_y_loop: for field_y in 0..next_turn_field.height() {
            'field_x_loop: for field_x in 0..next_turn_field.width() {
                let mut overlap = 0;
                for (piece_y, row) in prev_piece.trimmed_cells.iter().enumerate() {
                    for (piece_x, c) in row.iter().enumerate() {
                        if field_y + piece_y >= next_turn_field.height() || field_x + piece_x >= next_turn_field.width() {
                            continue;
                        }
                        let cell = next_turn_field.cells[field_y + piece_y][field_x + piece_x];
                        if *c == 'O' && is_player_cell(Some(cell), player_symbol) {
                            overlap += 1;
                            if overlap > 1 {
                                continue 'field_x_loop;
                            }
                        } else if *c == 'O' && is_enemy_cell(Some(cell), player_symbol) {
                            continue 'field_x_loop;
                        }
                    }
                }
                if overlap == 1 {
                    would_fit = true;
                    break 'field_y_loop;
                }
            }
        }
        if would_fit {
            placable_prev_pieces += 1;
        }
    }

    score_multiplier = placable_prev_pieces as f32 / total_prev_pieces as f32;
    score_multiplier
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Piece;

    fn create_test_piece(cells: Vec<Vec<char>>) -> Piece {
        let height = cells.len();
        let width = if height > 0 { cells[0].len() } else { 0 };

        Piece {
            size: crate::grid::Size { width, height },
            cells,
            trimmed_size: crate::grid::Size {
                width: 0,
                height: 0,
            },
            trimmed_cells: vec![],
            symbol_count: 0,
            offset: (0, 0),
        }
    }

    #[test]
    fn test_check_for_empty_lines_basic() {
        // Test basic functionality: empty rows from top and columns from left
        let cells = vec![
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
            vec!['O', '.', '.'],
        ];
        let piece = create_test_piece(cells);

        // Should find 2 empty rows from top
        let empty_lines = check_for_empty_lines(&piece, true, piece.height(), true);
        assert_eq!(empty_lines, vec![0, 1]);

        // Should find no empty columns from left (first column has 'O')
        let empty_lines = check_for_empty_lines(&piece, true, piece.width(), false);
        assert_eq!(empty_lines, Vec::<usize>::new());
    }

    #[test]
    fn test_check_for_empty_lines_stops_at_non_empty() {
        // Should stop when encountering first non-empty line
        let cells = vec![
            vec!['.', '.', '.'],
            vec!['O', '.', '.'], // Non-empty row - should stop here
            vec!['.', '.', '.'], // This empty row should be ignored
        ];
        let piece = create_test_piece(cells);

        let empty_lines = check_for_empty_lines(&piece, true, piece.height(), true);
        assert_eq!(empty_lines, vec![0]); // Only first row, stops at second
    }
}
