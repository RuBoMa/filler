use crate::grid::Grid;
use std::io::Error;

#[derive(Debug, Clone)]
pub struct Piece {
    pub row_count: usize,
    pub col_count: usize,
    pub cells: Vec<Vec<char>>,
}

impl Piece {
    pub fn new(header: &str) -> Self {
        let parts: Vec<&str> = header.trim_end_matches(':').split_whitespace().collect();
        let col_count: usize = parts[1].parse().expect("Invalid col count");
        let row_count: usize = parts[2].parse().expect("Invalid row count");

        let cells = vec![vec!['.'; col_count]; row_count];
        Piece {
            row_count,
            col_count,
            cells,
        }
    }

    pub fn update<I: Iterator<Item = Result<String, Error>>>(&mut self, lines: &mut I) {
        for r in 0..self.row_count() {
            let line = match lines.next() {
                Some(Ok(l)) => l.trim_end().to_string(),
                _ => panic!("Unexpected end of input while reading row {}", r),
            };

            if line.chars().count() < self.col_count() {
                panic!(
                    "Invalid row {}: expected at least {} characters, got {}",
                    r,
                    self.col_count(),
                    line.chars().count()
                );
            }

            let row_data: Vec<char> = line.chars().take(self.col_count()).collect();
            if row_data.len() != self.col_count() {
                panic!(
                    "Row {} has incorrect number of columns: expected {}, got {}",
                    r,
                    self.col_count(),
                    row_data.len()
                );
            }
            self.cells_mut()[r] = row_data;
        }
    }
}

impl Grid for Piece {
    fn row_count(&self) -> usize {
        self.row_count
    }
    fn col_count(&self) -> usize {
        self.col_count
    }
    fn cells(&self) -> &Vec<Vec<char>> {
        &self.cells
    }
    fn cells_mut(&mut self) -> &mut Vec<Vec<char>> {
        &mut self.cells
    }
}
use std::fmt;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_grid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_header_parsing() {
        let header = "Piece 4 3:";
        let piece = Piece::new(header);

        // Check dimensions were parsed correctly
        assert_eq!(piece.col_count(), 4);
        assert_eq!(piece.row_count(), 3);

        // Check that cells were initialized correctly
        assert_eq!(piece.cells().len(), 3); // 3 rows
        assert_eq!(piece.cells()[0].len(), 4); // 4 columns

        // Check all cells are initialized to '.'
        assert_eq!(piece.cells()[0][0], '.');
        assert_eq!(piece.cells()[2][3], '.'); // last cell
    }

    // Helper function to create mock input lines for testing piece.update()
    fn create_mock_lines(input: &str) -> std::vec::IntoIter<Result<String, Error>> {
        input
            .lines()
            .map(|line| Ok(line.to_string()))
            .collect::<Vec<_>>()
            .into_iter()
    }

    #[test]
    fn test_piece_shape_parsing_l_shape() {
        // Test parsing an L-shaped piece
        let mut piece = Piece::new("Piece 3 3:");

        // L-shaped piece:
        // O..
        // O..
        // OOO
        let input = "O..\nO..\nOOO\n";
        let mut lines = create_mock_lines(input);

        piece.update(&mut lines);

        // Verify the L-shape was parsed correctly
        assert_eq!(piece.cells()[0], vec!['O', '.', '.']); // Row 0: O..
        assert_eq!(piece.cells()[1], vec!['O', '.', '.']); // Row 1: O..
        assert_eq!(piece.cells()[2], vec!['O', 'O', 'O']); // Row 2: OOO
    }

    #[test]
    fn test_piece_single_dot() {
        // Test the simplest piece - just one dot
        let mut piece = Piece::new("Piece 1 1:");

        let input = "O\n";
        let mut lines = create_mock_lines(input);

        piece.update(&mut lines);

        assert_eq!(piece.cells()[0], vec!['O']);
    }

    #[test]
    fn test_piece_line_shape() {
        // Test a vertical line piece
        let mut piece = Piece::new("Piece 1 4:");

        // Vertical line:
        // O
        // O
        // O
        // O
        let input = "O\nO\nO\nO\n";
        let mut lines = create_mock_lines(input);

        piece.update(&mut lines);

        // Verify each row has one 'O'
        for i in 0..4 {
            assert_eq!(piece.cells()[i], vec!['O']);
        }
    }

    #[test]
    fn test_piece_t_shape() {
        // Test a T-shaped piece
        let mut piece = Piece::new("Piece 3 3:");

        // T-shaped piece:
        // OOO
        // .O.
        // .O.
        let input = "OOO\n.O.\n.O.\n";
        let mut lines = create_mock_lines(input);

        piece.update(&mut lines);

        assert_eq!(piece.cells()[0], vec!['O', 'O', 'O']); // Top: OOO
        assert_eq!(piece.cells()[1], vec!['.', 'O', '.']); // Mid: .O.
        assert_eq!(piece.cells()[2], vec!['.', 'O', '.']); // Bot: .O.
    }

    #[test]
    #[should_panic(expected = "Invalid col count")]
    fn test_piece_invalid_header() {
        // Test invalid piece header
        Piece::new("Piece abc 3:");
    }

    #[test]
    #[should_panic(expected = "Unexpected end of input")]
    fn test_piece_missing_rows() {
        let mut piece = Piece::new("Piece 2 3:"); // Expects 3 rows

        // Only provide 2 rows
        let input = "OO\n..\n"; // Missing 3rd row
        let mut lines = create_mock_lines(input);

        piece.update(&mut lines); // Should panic!
    }

    #[test]
    #[should_panic(expected = "expected at least 3 characters")]
    fn test_piece_row_too_short() {
        let mut piece = Piece::new("Piece 3 2:"); // Expects 3 chars per row

        // Row is too short
        let input = "OO\n...\n"; // First row only has 2 chars instead of 3
        let mut lines = create_mock_lines(input);

        piece.update(&mut lines); // Should panic!
    }
}
