#[derive(Debug, Clone)]
pub struct Player {
    pub _num: usize,
    pub _symbol: (char, char),
    pub path: String,
    pub _score: usize,
}

impl Player {
    pub fn new(input: &str) -> Self {
        let path = input
            .splitn(2, ':').nth(1)
            .map(str::trim)     // remove surrounding whitespace, but not brackets
            .unwrap_or("")      // fallback to empty string if split fails
            .to_string();       // convert to owned String

        if input.starts_with("$$$ exec p1") {
            Player { _num: 1, _symbol: ('a', '@'), path, _score: 0 }
        } else {
            Player { _num: 2, _symbol: ('s', '$'), path, _score: 0 }
        }
    }

    pub fn _is_mine(&self, c: &char) -> bool {
        self._symbol.0 == *c || self._symbol.1 == *c
    }
}