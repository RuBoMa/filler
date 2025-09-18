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
                num: 1,
                symbol: ('a', '@'),
                score: 0,
            }
        } else {
            Player {
                num: 2,
                symbol: ('s', '$'),
                score: 0,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player1_creation() {
        let input = "$$$ exec p1";
        
        let player = Player::new(input);
        
        assert_eq!(player.num, 1);
        assert_eq!(player.symbol, ('a', '@'));
        assert_eq!(player.score, 0);
    }
    #[test]
    fn test_player2_creation() {
        let input = "$$$ exec p2";

        let player = Player::new(input);

        assert_eq!(player.num, 2);
        assert_eq!(player.symbol, ('s', '$'));
        assert_eq!(player.score, 0);
    }
}