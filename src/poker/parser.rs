use super::card::{Card, Rank, Suit};
use super::hand::Hand;

pub fn parse_hand(hand_str: &str) -> Result<Hand, &'static str> {
    let cards: Vec<&str> = hand_str.split_whitespace().collect();
    
    if cards.len() != 5 {
        return Err("A hand must contain exactly 5 cards");
    }
    
    let mut parsed_cards = Vec::with_capacity(5);
    
    for card_str in cards {
        match parse_card(card_str) {
            Ok(card) => parsed_cards.push(card),
            Err(e) => return Err(e),
        }
    }
    
    Hand::new(parsed_cards)
}

pub fn parse_card(card_str: &str) -> Result<Card, &'static str> {
    if card_str.len() < 2 {
        return Err("Card string too short");
    }
    
    let rank_char = &card_str[0..card_str.len()-1];
    let suit_char = &card_str[card_str.len()-1..];
    
    let rank = match rank_char {
        "2" => Rank::Two,
        "3" => Rank::Three,
        "4" => Rank::Four,
        "5" => Rank::Five,
        "6" => Rank::Six,
        "7" => Rank::Seven,
        "8" => Rank::Eight,
        "9" => Rank::Nine,
        "10" => Rank::Ten,
        "J" => Rank::Jack,
        "Q" => Rank::Queen,
        "K" => Rank::King,
        "A" => Rank::Ace,
        _ => return Err("Invalid rank"),
    };
    
    let suit = match suit_char {
        "H" => Suit::Hearts,
        "D" => Suit::Diamonds,
        "C" => Suit::Clubs,
        "S" => Suit::Spades,
        _ => return Err("Invalid suit"),
    };
    
    Ok(Card { rank, suit })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::card::HandType;
    
    #[test]
    fn test_parse_card() {
        assert_eq!(
            parse_card("AS").unwrap(),
            Card { rank: Rank::Ace, suit: Suit::Spades }
        );
        
        assert_eq!(
            parse_card("KH").unwrap(),
            Card { rank: Rank::King, suit: Suit::Hearts }
        );
        
        assert_eq!(
            parse_card("QD").unwrap(),
            Card { rank: Rank::Queen, suit: Suit::Diamonds }
        );
        
        assert_eq!(
            parse_card("JC").unwrap(),
            Card { rank: Rank::Jack, suit: Suit::Clubs }
        );
        
        assert_eq!(
            parse_card("10S").unwrap(),
            Card { rank: Rank::Ten, suit: Suit::Spades }
        );
        
        assert!(parse_card("1S").is_err());
        assert!(parse_card("AX").is_err());
        assert!(parse_card("A").is_err());
    }
    
    #[test]
    fn test_parse_hand() {
        let hand = parse_hand("AS KS QS JS 10S").unwrap();
        assert_eq!(hand.evaluate(), HandType::RoyalFlush);
        
        let hand = parse_hand("AH AD AC AS KH").unwrap();
        assert_eq!(hand.evaluate(), HandType::FourOfAKind);
        
        assert!(parse_hand("AS KS QS JS").is_err());
        assert!(parse_hand("AS KS QS JS 10S 9S").is_err());
        assert!(parse_card("AX").is_err());
    }
} 