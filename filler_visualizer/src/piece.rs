use std::io::Error;
use crate::grid::{Size, Grid};

#[derive(Debug, Clone)]
pub struct Piece {
    pub size: Size,
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

        Piece { 
            size: Size{width: col_count, height: row_count}, cells
        }
    }

    pub fn update<I: Iterator<Item = Result<String, Error>>>(&mut self, lines: &mut I) {
        for r in 0..self.height() {
            let line = match lines.next() {
                Some(Ok(l)) => l.trim_end().to_string(),
                _ => panic!("Unexpected end of input while reading row {}", r),
            };

            if line.chars().count() < self.width() {
                panic!( "Invalid row {}: expected at least {} characters, got {}",
                    r, self.width(), line.chars().count());
            }

            let row_data: Vec<char> = line.chars().take(self.width()).collect();
            if row_data.len() != self.width() {
                panic!( "Row {} has incorrect number of columns: expected {}, got {}",
                    r, self.width(), row_data.len());
            }
            self.cells[r] = row_data;
        }
    }
}

impl Grid for Piece {
    fn height(&self) -> usize { self.size.height }
    fn width(&self) -> usize { self.size.width }
    fn cells(&self) -> &Vec<Vec<char>> { &self.cells }
    fn cells_mut(&mut self) -> &mut Vec<Vec<char>> { &mut self.cells }
}
use std::fmt;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_grid())
    }
}