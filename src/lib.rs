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

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
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
    
    pub fn evaluate(&self) -> HandType {
        let mut ranks = [0; 13];
        
        // Count occurrences of each rank
        for card in &self.cards {
            let rank_index = card.rank as usize - 2; // Rank::Two starts at 2
            ranks[rank_index] += 1;
        }
        
        // Count pairs and three of a kinds
        let mut pairs = 0;
        let mut three_of_a_kind = false;
        
        for &count in &ranks {
            if count == 2 {
                pairs += 1;
            } else if count == 3 {
                three_of_a_kind = true;
            }
        }
        
        // Check for three of a kind
        if three_of_a_kind {
            return HandType::ThreeOfAKind;
        }
        
        // Check for two pair
        if pairs == 2 {
            return HandType::TwoPair;
        }
        
        // Check for one pair
        if pairs == 1 {
            return HandType::OnePair;
        }
        
        // If no other combination is found, it's a high card
        HandType::HighCard
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
    
    #[test]
    fn test_high_card() {
        let cards = vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::King, suit: Suit::Diamonds },
            Card { rank: Rank::Queen, suit: Suit::Clubs },
            Card { rank: Rank::Jack, suit: Suit::Spades },
            Card { rank: Rank::Nine, suit: Suit::Hearts },
        ];
        
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.evaluate(), HandType::HighCard);
    }
    
    #[test]
    fn test_one_pair() {
        let cards = vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::Ace, suit: Suit::Diamonds },
            Card { rank: Rank::Queen, suit: Suit::Clubs },
            Card { rank: Rank::Jack, suit: Suit::Spades },
            Card { rank: Rank::Nine, suit: Suit::Hearts },
        ];
        
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.evaluate(), HandType::OnePair);
    }
    
    #[test]
    fn test_two_pair() {
        let cards = vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::Ace, suit: Suit::Diamonds },
            Card { rank: Rank::King, suit: Suit::Clubs },
            Card { rank: Rank::King, suit: Suit::Spades },
            Card { rank: Rank::Nine, suit: Suit::Hearts },
        ];
        
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.evaluate(), HandType::TwoPair);
    }
    
    #[test]
    fn test_three_of_a_kind() {
        let cards = vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::Ace, suit: Suit::Diamonds },
            Card { rank: Rank::Ace, suit: Suit::Clubs },
            Card { rank: Rank::King, suit: Suit::Spades },
            Card { rank: Rank::Nine, suit: Suit::Hearts },
        ];
        
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.evaluate(), HandType::ThreeOfAKind);
    }
} 