use crate::piece::Piece;
use crate::grid::Grid;
use crate::field::Field;
use crate::game::{Pos, Placement};

use std::iter::Iterator;

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

pub fn evaluate_placements(field: &Field, mut valid_placements: Vec<Placement>, enemy_pos: Pos, current_turn: usize, player_symbol: (char, char), prev_pieces: &Vec<Piece>) -> Placement {
    let has_touched_enemy_cell = check_if_touching_enemy_cell(&field, player_symbol);
    if !has_touched_enemy_cell {
        evaluate_placement_for_enemy_distance(&field, &mut valid_placements, enemy_pos, current_turn);
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

pub fn evaluate_placement_for_perfect_fit(field: &Field, placements: &mut Vec<Placement>, current_turn: usize, _prev_pieces: &Vec<Piece>, _player_symbol: (char, char)) {
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
            // println!("Perfect fit adding {} score to placement at pos {:?}", current_score_addition, placement.pos);
            placement.score += (current_score_addition as f32) as i32;
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

pub fn get_center_of_piece(field: &Field, placement: &Pos, piece: &Piece) -> Pos {
    let mut x = placement.x + (piece.trimmed_size.width + piece.offset.1)/2;
    let mut y = placement.y + (piece.trimmed_size.height + piece.offset.0)/2;
    while x >= field.width() {
        x -= 1;
    }
    while y >= field.height() {
        y -= 1;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Size;

    fn create_test_field() -> Field {
        Field {
            size: Size {
                width: 4,
                height: 4,
            },
            cells: vec![
                vec!['.', 'a', '.', '.'],
                vec!['.', '.', '.', '.'],
                vec!['.', '.', 's', '.'],
                vec!['.', '.', '.', '.'],
            ],
        }
    }

    fn create_test_piece() -> Piece {
        Piece {
            size: Size {
                width: 2,
                height: 2,
            },
            cells: vec![vec!['O', '.'], vec!['.', 'O']],
            trimmed_size: Size {
                width: 2,
                height: 2,
            },
            trimmed_cells: vec![vec!['O', '.'], vec!['.', 'O']],
            symbol_count: 2,
            offset: (0, 0),
        }
    }

    fn create_test_placement(pos: Pos, score: i32) -> Placement {
        Placement {
            pos,
            piece: create_test_piece(),
            score,
        }
    }

    #[test]
    fn test_check_if_touching_enemy_cell() {
        // Field with player 'a' at (0,1) and enemy 's' at (2,2)
        let field = create_test_field();
        let player_symbol = ('a', '@');

        // They are not adjacent, so should return false
        let result = check_if_touching_enemy_cell(&field, player_symbol);
        assert_eq!(result, false);

        // Create a field where they are adjacent
        let field_adjacent = Field {
            size: Size {
                width: 4,
                height: 4,
            },
            cells: vec![
                vec!['.', 'a', 's', '.'], // Player and enemy adjacent
                vec!['.', '.', '.', '.'],
                vec!['.', '.', '.', '.'],
                vec!['.', '.', '.', '.'],
            ],
        };

        let result_adjacent = check_if_touching_enemy_cell(&field_adjacent, player_symbol);
        assert_eq!(result_adjacent, true);
    }

    #[test]
    fn test_evaluate_placement_for_enemy_distance() {
        let field = create_test_field();
        let enemy_pos = Pos { x: 2, y: 2 }; // Enemy at (2,2)
        let current_turn = 1;

        let mut placements = vec![
            create_test_placement(Pos { x: 0, y: 0 }, 0), // Far from enemy
            create_test_placement(Pos { x: 1, y: 1 }, 0), // Close to enemy
        ];

        evaluate_placement_for_enemy_distance(&field, &mut placements, enemy_pos, current_turn);

        // The closer placement should have higher score
        assert!(placements[1].score > placements[0].score);
        // Both should have positive scores
        assert!(placements[0].score > 0);
        assert!(placements[1].score > 0);
    }

    #[test]
    fn test_is_enemy_cell() {
        let player_symbol = ('a', '@');

        // Test enemy cells
        assert_eq!(is_enemy_cell(Some('s'), player_symbol), true);
        assert_eq!(is_enemy_cell(Some('$'), player_symbol), true);

        // Test non-enemy cells
        assert_eq!(is_enemy_cell(Some('a'), player_symbol), false); // Player cell
        assert_eq!(is_enemy_cell(Some('@'), player_symbol), false); // Player cell
        assert_eq!(is_enemy_cell(Some('.'), player_symbol), false); // Empty cell
        assert_eq!(is_enemy_cell(None, player_symbol), false); // No cell
    }

    #[test]
    fn test_is_player_cell() {
        let player_symbol = ('a', '@');

        // Test player cells
        assert_eq!(is_player_cell(Some('a'), player_symbol), true);
        assert_eq!(is_player_cell(Some('@'), player_symbol), true);

        // Test non-player cells
        assert_eq!(is_player_cell(Some('s'), player_symbol), false); // Enemy cell
        assert_eq!(is_player_cell(Some('$'), player_symbol), false); // Enemy cell
        assert_eq!(is_player_cell(Some('.'), player_symbol), false); // Empty cell
        assert_eq!(is_player_cell(None, player_symbol), false); // No cell
    }
}
