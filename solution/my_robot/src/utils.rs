use crate::piece::Piece;
use crate::grid::Grid;
use std::iter::Iterator;

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