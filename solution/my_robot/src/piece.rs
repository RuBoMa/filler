use std::io::Error;
use crate::grid::{Size, Grid};

#[derive(Debug, Clone)]
pub struct Piece {
    pub size: Size,
    pub cells: Vec<Vec<char>>,
    pub trimmed_size: Size,
    pub trimmed_cells: Vec<Vec<char>>,
    pub symbol_count: usize,
    pub offset: (usize, usize),
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

        Piece { 
            size: Size{width: cols, height: rows}, cells,
            trimmed_size: Size{width: 0, height: 0}, trimmed_cells: vec![],
            symbol_count: 0, offset: (0,0)
        }
    }

    fn trim_cells(&mut self) {
        let mut top = 0;
        let mut bottom = self.height();
        let mut left = 0;
        let mut right = self.width();

        while top < bottom && self.cells[top].iter().all(|&c| c == '.') { top += 1 }
        while bottom > top && self.cells[bottom - 1].iter().all(|&c| c == '.') { bottom -= 1 }
        while left < right && (top..bottom).all(|i| self.cells[i][left] == '.') { left += 1 }
        while right > left && (top..bottom).all(|i| self.cells[i][right - 1] == '.') { right -= 1 }

        let mut trimmed = vec![];
        (top..bottom).for_each(|i| trimmed.push(self.cells[i][left..right].to_vec()));

        self.trimmed_cells = trimmed;
        self.trimmed_size = Size { width: self.trimmed_cells[0].len(), height: self.trimmed_cells.len() };
        self.offset = (top, left);
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
            self.symbol_count += row_data.iter().filter(|&&ch| ch != '.').count();
            self.cells[r] = row_data;
        }
        self.trim_cells();
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