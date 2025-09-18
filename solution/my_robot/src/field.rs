use crate::grid::Grid;
use std::io::Error;

#[derive(Debug, Clone)]
pub struct Field {
    pub row_count: usize,
    pub col_count: usize,
    pub cells: Vec<Vec<char>>,
}

impl Field {
    pub fn new(header: &str) -> Self {
        let parts: Vec<&str> = header.trim_end_matches(':').split_whitespace().collect();
        let col_count: usize = parts[1].parse().expect("Invalid col count");
        let row_count: usize = parts[2].parse().expect("Invalid row count");

        let cells = vec![vec!['.'; col_count]; row_count];
        Field {
            row_count,
            col_count,
            cells,
        }
    }

    pub fn update<I: Iterator<Item = Result<String, Error>>>(&mut self, lines: &mut I) {
        let _ = lines.next(); // skip column headers

        for r in 0..self.row_count() {
            let line = match lines.next() {
                Some(Ok(l)) => l.trim_end().to_string(),
                _ => panic!("Unexpected end of input while reading row {}", r),
            };

            if line.chars().count() < 4 + self.col_count() {
                panic!(
                    "Invalid row {}: expected at least {} characters, got {}",
                    r,
                    4 + self.col_count(),
                    line.chars().count()
                );
            }

            let row_data: Vec<char> = line[4..].chars().take(self.col_count()).collect();
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

impl Grid for Field {
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

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_grid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_header_parsing() {
        // Test basic field header: "Field col_count row_count:"
        let header = "Field 10 15:";
        let field = Field::new(header);

        // Check dimensions were parsed correctly
        assert_eq!(field.col_count(), 10);
        assert_eq!(field.row_count(), 15);

        // Check that cells were initialized correctly
        assert_eq!(field.cells().len(), 15); // 15 rows
        assert_eq!(field.cells()[0].len(), 10); // 10 columns

        // Check all cells are initialized to '.'
        assert_eq!(field.cells()[0][0], '.');
        assert_eq!(field.cells()[14][9], '.'); // last cell
    }

    #[test]
    fn test_different_field_sizes() {
        // Test a small field
        let small_field = Field::new("Field 3 2:");
        assert_eq!(small_field.col_count(), 3);
        assert_eq!(small_field.row_count(), 2);

        // Test a larger field
        let big_field = Field::new("Field 20 25:");
        assert_eq!(big_field.col_count(), 20);
        assert_eq!(big_field.row_count(), 25);

        // Test minimum size
        let tiny_field = Field::new("Field 1 1:");
        assert_eq!(tiny_field.col_count(), 1);
        assert_eq!(tiny_field.row_count(), 1);
        assert_eq!(tiny_field.cells()[0][0], '.');
    }

    #[test]
    #[should_panic(expected = "Invalid col count")]
    fn test_invalid_column_count() {
        // This should panic because "abc" is not a number
        Field::new("Field abc 5:");
    }

    #[test]
    #[should_panic(expected = "Invalid row count")]
    fn test_invalid_row_count() {
        // This should panic because "xyz" is not a number
        Field::new("Field 10 xyz:");
    }
}
