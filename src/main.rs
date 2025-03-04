use exam::{Card, Hand, Rank, Suit};

fn main() {
    println!("Poker Hand Evaluator");
    
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
