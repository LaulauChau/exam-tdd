use super::card::{Card, HandType, Rank, Suit};
use std::cmp::Ordering;

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
        
        // Handle special case: A-2-3-4-5 straight
        if ranks[0] == 1 && ranks[1] == 1 && ranks[2] == 1 && ranks[3] == 1 && ranks[12] == 1 {
            is_straight = true;
        } else {
            // Check for normal straights
            let mut consecutive_count = 0;
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
    
    pub fn get_rank_counts(&self) -> [u8; 13] {
        let mut ranks = [0; 13];
        
        for card in &self.cards {
            let rank_index = card.rank as usize - 2; // Rank::Two starts at 2
            ranks[rank_index] += 1;
        }
        
        ranks
    }
    
    pub fn get_sorted_ranks(&self) -> Vec<Rank> {
        let mut ranks: Vec<Rank> = self.cards.iter().map(|card| card.rank).collect();
        ranks.sort_by(|a, b| b.cmp(a)); // Sort in descending order
        ranks
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare hand types
        let self_type = self.evaluate();
        let other_type = other.evaluate();
        
        if self_type != other_type {
            return self_type.cmp(&other_type);
        }
        
        // If hand types are the same, we need to compare based on the specific hand type
        match self_type {
            HandType::HighCard | HandType::Flush => {
                // For high card and flush, we compare the highest cards first
                let mut self_cards = self.cards.clone();
                let mut other_cards = other.cards.clone();
                
                // Sort cards by rank in descending order
                self_cards.sort_by(|a, b| b.rank.cmp(&a.rank));
                other_cards.sort_by(|a, b| b.rank.cmp(&a.rank));
                
                // Compare each card by rank
                for (self_card, other_card) in self_cards.iter().zip(other_cards.iter()) {
                    match self_card.rank.cmp(&other_card.rank) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                
                Ordering::Equal
            },
            HandType::OnePair => {
                // Find the pair rank in each hand
                let self_counts = self.get_rank_counts();
                let other_counts = other.get_rank_counts();
                
                // Find the pair rank
                let self_pair_rank = (0..13)
                    .find(|&i| self_counts[i] == 2)
                    .map(|i| i + 2) // Convert index to rank value
                    .unwrap();
                
                let other_pair_rank = (0..13)
                    .find(|&i| other_counts[i] == 2)
                    .map(|i| i + 2) // Convert index to rank value
                    .unwrap();
                
                // Compare pair ranks
                if self_pair_rank != other_pair_rank {
                    return self_pair_rank.cmp(&other_pair_rank);
                }
                
                // If pairs are the same, compare kickers in descending order
                let mut self_kickers: Vec<usize> = (0..13)
                    .filter(|&i| self_counts[i] == 1)
                    .map(|i| i + 2)
                    .collect();
                
                let mut other_kickers: Vec<usize> = (0..13)
                    .filter(|&i| other_counts[i] == 1)
                    .map(|i| i + 2)
                    .collect();
                
                // Sort kickers in descending order
                self_kickers.sort_by(|a, b| b.cmp(a));
                other_kickers.sort_by(|a, b| b.cmp(a));
                
                // Compare kickers
                for (self_kicker, other_kicker) in self_kickers.iter().zip(other_kickers.iter()) {
                    if self_kicker != other_kicker {
                        return self_kicker.cmp(other_kicker);
                    }
                }
                
                Ordering::Equal
            },
            // For simplicity, we'll implement just a few hand type comparisons
            // In a complete implementation, you would handle all hand types
            _ => {
                // For other hand types, we'll just compare the highest cards
                let mut self_cards = self.cards.clone();
                let mut other_cards = other.cards.clone();
                
                // Sort cards by rank in descending order
                self_cards.sort_by(|a, b| b.rank.cmp(&a.rank));
                other_cards.sort_by(|a, b| b.rank.cmp(&a.rank));
                
                // Compare each card by rank
                for (self_card, other_card) in self_cards.iter().zip(other_cards.iter()) {
                    match self_card.rank.cmp(&other_card.rank) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                
                Ordering::Equal
            },
        }
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
    
    #[test]
    fn test_compare_different_hand_types() {
        let royal_flush = Hand::new(vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::King, suit: Suit::Hearts },
            Card { rank: Rank::Queen, suit: Suit::Hearts },
            Card { rank: Rank::Jack, suit: Suit::Hearts },
            Card { rank: Rank::Ten, suit: Suit::Hearts },
        ]).unwrap();
        
        let four_of_a_kind = Hand::new(vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::Ace, suit: Suit::Diamonds },
            Card { rank: Rank::Ace, suit: Suit::Clubs },
            Card { rank: Rank::Ace, suit: Suit::Spades },
            Card { rank: Rank::King, suit: Suit::Hearts },
        ]).unwrap();
        
        assert!(royal_flush > four_of_a_kind);
    }
    
    #[test]
    fn test_compare_same_hand_type_high_card() {
        let high_card1 = Hand::new(vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::King, suit: Suit::Diamonds },
            Card { rank: Rank::Queen, suit: Suit::Clubs },
            Card { rank: Rank::Jack, suit: Suit::Spades },
            Card { rank: Rank::Eight, suit: Suit::Hearts },
        ]).unwrap();
        
        let high_card2 = Hand::new(vec![
            Card { rank: Rank::King, suit: Suit::Hearts },
            Card { rank: Rank::Queen, suit: Suit::Diamonds },
            Card { rank: Rank::Jack, suit: Suit::Clubs },
            Card { rank: Rank::Nine, suit: Suit::Spades },
            Card { rank: Rank::Seven, suit: Suit::Hearts },
        ]).unwrap();
        
        assert!(high_card1 > high_card2);
    }
    
    #[test]
    fn test_compare_same_hand_type_one_pair() {
        let pair_aces = Hand::new(vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::Ace, suit: Suit::Diamonds },
            Card { rank: Rank::King, suit: Suit::Clubs },
            Card { rank: Rank::Queen, suit: Suit::Spades },
            Card { rank: Rank::Jack, suit: Suit::Hearts },
        ]).unwrap();
        
        let pair_kings = Hand::new(vec![
            Card { rank: Rank::King, suit: Suit::Hearts },
            Card { rank: Rank::King, suit: Suit::Diamonds },
            Card { rank: Rank::Queen, suit: Suit::Clubs },
            Card { rank: Rank::Jack, suit: Suit::Spades },
            Card { rank: Rank::Ten, suit: Suit::Hearts },
        ]).unwrap();
        
        assert!(pair_aces > pair_kings);
    }
    
    #[test]
    fn test_compare_same_pair_different_kickers() {
        let pair_aces_king_high = Hand::new(vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::Ace, suit: Suit::Diamonds },
            Card { rank: Rank::King, suit: Suit::Clubs },
            Card { rank: Rank::Queen, suit: Suit::Spades },
            Card { rank: Rank::Jack, suit: Suit::Hearts },
        ]).unwrap();
        
        let pair_aces_queen_high = Hand::new(vec![
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::Ace, suit: Suit::Diamonds },
            Card { rank: Rank::Queen, suit: Suit::Clubs },
            Card { rank: Rank::Jack, suit: Suit::Spades },
            Card { rank: Rank::Ten, suit: Suit::Hearts },
        ]).unwrap();
        
        assert!(pair_aces_king_high > pair_aces_queen_high);
    }
} 