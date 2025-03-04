use exam::{Card, Hand, Rank, Suit};

fn main() {
    println!("Poker Hand Evaluator");
    
    // Exemple de création d'une main
    let royal_flush = vec![
        Card { rank: Rank::Ace, suit: Suit::Hearts },
        Card { rank: Rank::King, suit: Suit::Hearts },
        Card { rank: Rank::Queen, suit: Suit::Hearts },
        Card { rank: Rank::Jack, suit: Suit::Hearts },
        Card { rank: Rank::Ten, suit: Suit::Hearts },
    ];
    
    match Hand::new(royal_flush) {
        Ok(hand) => println!("Main créée avec succès: {:?}", hand),
        Err(e) => println!("Erreur lors de la création de la main: {}", e),
    }
}
