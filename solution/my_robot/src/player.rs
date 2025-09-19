#[derive(Debug, Clone)]
pub struct Player {
    pub _num: usize,
    pub symbol: (char, char),
    pub score: usize,
}

impl Player {
    pub fn new(input: &str) -> (Self, Self) {
        let p1 = Player {
            _num: 1,
            symbol: ('a', '@'),
            score: 0,
        };
        let p2 = Player {
            _num: 2,
            symbol: ('s', '$'),
            score: 0,
        };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_new_p1_vs_p2() {
        // Test p1 detection
        let (player, enemy) = Player::new("$$$ exec p1 :");
        assert_eq!(player._num, 1);
        assert_eq!(player.symbol, ('a', '@'));
        assert_eq!(enemy._num, 2);
        assert_eq!(enemy.symbol, ('s', '$'));

        // Test p2 detection
        let (player, enemy) = Player::new("$$$ exec p2 :");
        assert_eq!(player._num, 2);
        assert_eq!(player.symbol, ('s', '$'));
        assert_eq!(enemy._num, 1);
        assert_eq!(enemy.symbol, ('a', '@'));
    }

    #[test]
    fn test_is_mine() {
        let p1 = Player {
            _num: 1,
            symbol: ('a', '@'),
            score: 0,
        };

        // Test player's own symbols
        assert_eq!(p1.is_mine(&'a'), true);
        assert_eq!(p1.is_mine(&'@'), true);

        // Test enemy symbols
        assert_eq!(p1.is_mine(&'s'), false);
        assert_eq!(p1.is_mine(&'$'), false);

        // Test empty cell
        assert_eq!(p1.is_mine(&'.'), false);
    }
}
