#[derive(Debug, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Size { width, height }
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }
}

pub trait Grid {
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn cells(&self) -> &Vec<Vec<char>>;
    fn cells_mut(&mut self) -> &mut Vec<Vec<char>>;

    fn print_grid(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Grid ({} rows x {} cols):\n", self.height(), self.width()));
        for row in self.cells() {
            for ch in row {
                output.push(*ch);
            }
            output.push('\n');
        }
        output
    }
}
