#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub num: usize,
    pub symbol: (char, char),
    pub score: usize,
}

impl Player {
    pub fn new(input: &str) -> Self {
        if input.starts_with("$$$ exec p1") {
            Player {
                num: 1, symbol: ('a', '@'), score: 0
            }
        } else {
            Player {
                num: 2, symbol: ('s', '$'), score: 0
            }
        }
    }
}