#[derive(Debug, Clone)]
pub struct Player {
    pub _num: usize,
    pub symbol: (char, char),
    pub score: usize,
}

impl Player {
    pub fn new(input: &str) -> (Self, Self) {
        let p1 = Player { _num: 1, symbol: ('a', '@'), score: 0 };
        let p2 = Player { _num: 2, symbol: ('s', '$'), score: 0 };
        if input.starts_with("$$$ exec p1") {
            (p1, p2)
        } else {
            (p2, p1)
        }
    }

    pub fn is_mine(&self, c: &char) -> bool {
        self.symbol.0 == *c || self.symbol.1 == *c
    }
}