use exam::{Card, Hand, Rank, Suit};
use std::env;

fn main() {
    println!("Poker Hand Evaluator");
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 3 {
        // User provided two hands as arguments
        let hand1_str = &args[1];
        let hand2_str = &args[2];
        
        match (parse_hand(hand1_str), parse_hand(hand2_str)) {
            (Ok(hand1), Ok(hand2)) => {
                println!("Hand 1: {:?} - {:?}", hand1_str, hand1.evaluate());
                println!("Hand 2: {:?} - {:?}", hand2_str, hand2.evaluate());
                
                if hand1 > hand2 {
                    println!("Hand 1 wins!");
                } else if hand2 > hand1 {
                    println!("Hand 2 wins!");
                } else {
                    println!("It's a tie!");
                }
            },
            (Err(e), _) => println!("Error parsing hand 1: {}", e),
            (_, Err(e)) => println!("Error parsing hand 2: {}", e),
        }
    } else {
        // No arguments provided, use default hands
        println!("No hands provided as arguments. Using default hands.");
        println!("Usage: cargo run -- <hand1> <hand2>");
        println!("Example: cargo run -- \"AS KS QS JS 10S\" \"AH AD AC AS KH\"");
        println!("Using default hands instead:");
        
        // Create two hands
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
        
        // Compare hands
        println!("Royal Flush: {:?}", royal_flush.evaluate());
        println!("Four of a Kind: {:?}", four_of_a_kind.evaluate());
        
        if royal_flush > four_of_a_kind {
            println!("Royal Flush wins!");
        } else {
            println!("Four of a Kind wins!");
        }
        
        // Create two hands of the same type
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
        
        // Compare hands of the same type
        println!("\nPair of Aces: {:?}", pair_aces.evaluate());
        println!("Pair of Kings: {:?}", pair_kings.evaluate());
        
        if pair_aces > pair_kings {
            println!("Pair of Aces wins!");
        } else {
            println!("Pair of Kings wins!");
        }
    }
}

fn parse_hand(hand_str: &str) -> Result<Hand, &'static str> {
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

fn parse_card(card_str: &str) -> Result<Card, &'static str> {
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
