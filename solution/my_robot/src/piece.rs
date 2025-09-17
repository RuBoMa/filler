use std::io::Error;
use crate::grid::Grid;

#[derive(Debug, Clone)]
pub struct Piece {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<char>>,
}

impl Piece {
    pub fn new(header: &str) -> Self {

        let parts: Vec<&str> = header
            .trim_end_matches(':')
            .split_whitespace()
            .collect();
        let cols: usize = parts[1].parse().expect("Invalid col count");
        let rows: usize = parts[2].parse().expect("Invalid row count");

        let cells = vec![vec!['.'; cols]; rows];
        Piece { rows, cols, cells }
    }

    pub fn update<I: Iterator<Item = Result<String, Error>>>(&mut self, lines: &mut I) {
        for r in 0..self.rows() {
            let line = match lines.next() {
                Some(Ok(l)) => l.trim_end().to_string(),
                _ => panic!("Unexpected end of input while reading row {}", r),
            };

            if line.chars().count() < self.cols() {
                panic!( "Invalid row {}: expected at least {} characters, got {}",
                    r, self.cols(), line.chars().count());
            }

            let row_data: Vec<char> = line.chars().take(self.cols()).collect();
            if row_data.len() != self.cols() {
                panic!( "Row {} has incorrect number of columns: expected {}, got {}",
                    r, self.cols(), row_data.len());
            }
            self.cells_mut()[r] = row_data;
        }
    }
}

impl Grid for Piece {
    fn rows(&self) -> usize { self.rows }
    fn cols(&self) -> usize { self.cols }
    fn cells(&self) -> &Vec<Vec<char>> { &self.cells }
    fn cells_mut(&mut self) -> &mut Vec<Vec<char>> { &mut self.cells }
}
use std::fmt;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_grid())
    }
}