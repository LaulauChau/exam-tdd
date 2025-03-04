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
        let mut suits = [0; 4];
        
        // Count occurrences of each rank and suit
        for card in &self.cards {
            let rank_index = card.rank as usize - 2; // Rank::Two starts at 2
            ranks[rank_index] += 1;
            
            let suit_index = match card.suit {
                Suit::Hearts => 0,
                Suit::Diamonds => 1,
                Suit::Clubs => 2,
                Suit::Spades => 3,
            };
            suits[suit_index] += 1;
        }
        
        // Check for flush (all cards of the same suit)
        let is_flush = suits.iter().any(|&count| count == 5);
        
        // Check for straight (five cards in sequence)
        let mut is_straight = false;
        let mut consecutive_count = 0;
        
        // Handle special case: A-2-3-4-5 straight
        if ranks[0] == 1 && ranks[1] == 1 && ranks[2] == 1 && ranks[3] == 1 && ranks[12] == 1 {
            is_straight = true;
        } else {
            // Check for normal straights
            for &count in &ranks {
                if count == 1 {
                    consecutive_count += 1;
                    if consecutive_count == 5 {
                        is_straight = true;
                        break;
                    }
                } else {
                    consecutive_count = 0;
                }
            }
        }
        
        // Count pairs, three of a kinds, and four of a kinds
        let mut pairs = 0;
        let mut three_of_a_kind = false;
        let mut four_of_a_kind = false;
        
        for &count in &ranks {
            if count == 2 {
                pairs += 1;
            } else if count == 3 {
                three_of_a_kind = true;
            } else if count == 4 {
                four_of_a_kind = true;
            }
        }
        
        // Determine hand type
        if is_straight && is_flush {
            // Check for royal flush (A-K-Q-J-10 of the same suit)
            if ranks[8] == 1 && ranks[9] == 1 && ranks[10] == 1 && ranks[11] == 1 && ranks[12] == 1 {
                return HandType::RoyalFlush;
            }
            return HandType::StraightFlush;
        }
        
        if four_of_a_kind {
            return HandType::FourOfAKind;
        }
        
        if three_of_a_kind && pairs == 1 {
            return HandType::FullHouse;
        }
        
        if is_flush {
            return HandType::Flush;
        }
        
        if is_straight {
            return HandType::Straight;
        }
        
        if three_of_a_kind {
            return HandType::ThreeOfAKind;
        }
        
        if pairs == 2 {
            return HandType::TwoPair;
        }
        
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
    
    #[test]
    fn test_straight() {
        let cards = vec![
            Card { rank: Rank::Ten, suit: Suit::Hearts },
            Card { rank: Rank::Nine, suit: Suit::Diamonds },
            Card { rank: Rank::Eight, suit: Suit::Clubs },
            Card { rank: Rank::Seven, suit: Suit::Spades },
            Card { rank: Rank::Six, suit: Suit::Hearts },
        ];
        
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.evaluate(), HandType::Straight);
    }
    
    #[test]
    fn test_flush() {
        let cards = vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::Ten, suit: Suit::Hearts },
            Card { rank: Rank::Eight, suit: Suit::Hearts },
            Card { rank: Rank::Six, suit: Suit::Hearts },
            Card { rank: Rank::Four, suit: Suit::Hearts },
        ];
        
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.evaluate(), HandType::Flush);
    }
    
    #[test]
    fn test_full_house() {
        let cards = vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::Ace, suit: Suit::Diamonds },
            Card { rank: Rank::Ace, suit: Suit::Clubs },
            Card { rank: Rank::King, suit: Suit::Spades },
            Card { rank: Rank::King, suit: Suit::Hearts },
        ];
        
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.evaluate(), HandType::FullHouse);
    }
    
    #[test]
    fn test_four_of_a_kind() {
        let cards = vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::Ace, suit: Suit::Diamonds },
            Card { rank: Rank::Ace, suit: Suit::Clubs },
            Card { rank: Rank::Ace, suit: Suit::Spades },
            Card { rank: Rank::King, suit: Suit::Hearts },
        ];
        
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.evaluate(), HandType::FourOfAKind);
    }
    
    #[test]
    fn test_straight_flush() {
        let cards = vec![
            Card { rank: Rank::Nine, suit: Suit::Hearts },
            Card { rank: Rank::Eight, suit: Suit::Hearts },
            Card { rank: Rank::Seven, suit: Suit::Hearts },
            Card { rank: Rank::Six, suit: Suit::Hearts },
            Card { rank: Rank::Five, suit: Suit::Hearts },
        ];
        
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.evaluate(), HandType::StraightFlush);
    }
    
    #[test]
    fn test_royal_flush() {
        let cards = vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::King, suit: Suit::Hearts },
            Card { rank: Rank::Queen, suit: Suit::Hearts },
            Card { rank: Rank::Jack, suit: Suit::Hearts },
            Card { rank: Rank::Ten, suit: Suit::Hearts },
        ];
        
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.evaluate(), HandType::RoyalFlush);
    }
    
    #[test]
    fn test_low_ace_straight() {
        let cards = vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::Two, suit: Suit::Diamonds },
            Card { rank: Rank::Three, suit: Suit::Clubs },
            Card { rank: Rank::Four, suit: Suit::Spades },
            Card { rank: Rank::Five, suit: Suit::Hearts },
        ];
        
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.evaluate(), HandType::Straight);
    }
} 