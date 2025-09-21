#[derive(Debug, Clone)]
pub struct Player {
    pub _num: usize,
    pub symbol: (char, char),
    pub path: String,
    pub score: usize,
}

impl Player {
    pub fn new(input: &str) -> Self {
        let path = input
            .splitn(2, ':').nth(1)
            .map(str::trim)     // remove surrounding whitespace, but not brackets
            .unwrap_or("")      // fallback to empty string if split fails
            .to_string();       // convert to owned String

        if input.starts_with("$$$ exec p1") {
            Player { _num: 1, symbol: ('a', '@'), path, score: 0 }
        } else {
            Player { _num: 2, symbol: ('s', '$'), path, score: 0 }
        }
    }

    pub fn is_mine(&self, c: &char) -> bool {
        self.symbol.0 == *c || self.symbol.1 == *c
    }
}