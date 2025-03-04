#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Result<Self, &'static str> {
        if cards.len() != 5 {
            return Err("A hand must contain exactly 5 cards");
        }
        Ok(Hand { cards })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_creation() {
        let cards = vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::King, suit: Suit::Hearts },
            Card { rank: Rank::Queen, suit: Suit::Hearts },
            Card { rank: Rank::Jack, suit: Suit::Hearts },
            Card { rank: Rank::Ten, suit: Suit::Hearts },
        ];
        
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.cards.len(), 5);
    }

    #[test]
    fn test_hand_creation_error() {
        let cards = vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::King, suit: Suit::Hearts },
            Card { rank: Rank::Queen, suit: Suit::Hearts },
        ];
        
        let result = Hand::new(cards);
        assert!(result.is_err());
    }
} 