use std::io::Error;
use crate::grid::Grid;

#[derive(Debug, Clone)]
pub struct Piece {
    pub row_count: usize,
    pub col_count: usize,
    pub cells: Vec<Vec<char>>,
}

impl Piece {
    pub fn new(header: &str) -> Self {

        let parts: Vec<&str> = header
            .trim_end_matches(':')
            .split_whitespace()
            .collect();
        let col_count: usize = parts[1].parse().expect("Invalid col count");
        let row_count: usize = parts[2].parse().expect("Invalid row count");

        let cells = vec![vec!['.'; col_count]; row_count];
        Piece { row_count, col_count, cells }
    }

    pub fn update<I: Iterator<Item = Result<String, Error>>>(&mut self, lines: &mut I) {
        for r in 0..self.row_count() {
            let line = match lines.next() {
                Some(Ok(l)) => l.trim_end().to_string(),
                _ => panic!("Unexpected end of input while reading row {}", r),
            };

            if line.chars().count() < self.col_count() {
                panic!( "Invalid row {}: expected at least {} characters, got {}",
                    r, self.col_count(), line.chars().count());
            }

            let row_data: Vec<char> = line.chars().take(self.col_count()).collect();
            if row_data.len() != self.col_count() {
                panic!( "Row {} has incorrect number of columns: expected {}, got {}",
                    r, self.col_count(), row_data.len());
            }
            self.cells_mut()[r] = row_data;
        }
    }
}

impl Grid for Piece {
    fn row_count(&self) -> usize { self.row_count }
    fn col_count(&self) -> usize { self.col_count }
    fn cells(&self) -> &Vec<Vec<char>> { &self.cells }
    fn cells_mut(&mut self) -> &mut Vec<Vec<char>> { &mut self.cells }
}
use std::fmt;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_grid())
    }
}