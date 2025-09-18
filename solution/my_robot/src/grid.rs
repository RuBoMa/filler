
pub trait Grid {
    fn row_count(&self) -> usize;
    fn col_count(&self) -> usize;
    fn cells(&self) -> &Vec<Vec<char>>;
    fn cells_mut(&mut self) -> &mut Vec<Vec<char>>;

    fn print_grid(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Grid ({} rows x {} cols):\n", self.row_count(), self.col_count()));
        for row in self.cells() {
            for ch in row {
                output.push(*ch);
            }
            output.push('\n');
        }
        output
    }
}
