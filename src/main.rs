use exam::poker::parse_hand;
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
        // No arguments provided, just show usage information
        println!("No hands provided as arguments.");
        println!("Usage: cargo run -- <hand1> <hand2>");
        println!("Example: cargo run -- \"AS KS QS JS 10S\" \"AH AD AC AS KH\"");
    }
}