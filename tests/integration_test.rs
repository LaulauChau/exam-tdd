use exam::poker::{parse_hand, HandType};

#[test]
fn test_parse_and_evaluate_royal_flush() {
    let hand = parse_hand("AS KS QS JS 10S").unwrap();
    assert_eq!(hand.evaluate(), HandType::RoyalFlush);
}

#[test]
fn test_parse_and_evaluate_straight_flush() {
    let hand = parse_hand("9S 8S 7S 6S 5S").unwrap();
    assert_eq!(hand.evaluate(), HandType::StraightFlush);
}

#[test]
fn test_parse_and_evaluate_four_of_a_kind() {
    let hand = parse_hand("AH AD AC AS KH").unwrap();
    assert_eq!(hand.evaluate(), HandType::FourOfAKind);
}

#[test]
fn test_parse_and_evaluate_full_house() {
    let hand = parse_hand("AH AD AC KH KD").unwrap();
    assert_eq!(hand.evaluate(), HandType::FullHouse);
}

#[test]
fn test_parse_and_evaluate_flush() {
    let hand = parse_hand("AH 3H 5H 7H 9H").unwrap();
    assert_eq!(hand.evaluate(), HandType::Flush);
}

#[test]
fn test_parse_and_evaluate_straight() {
    let hand = parse_hand("10D JS QH KS AC").unwrap();
    assert_eq!(hand.evaluate(), HandType::Straight);
}

#[test]
fn test_parse_and_evaluate_three_of_a_kind() {
    let hand = parse_hand("AH AD AC 2H 3D").unwrap();
    assert_eq!(hand.evaluate(), HandType::ThreeOfAKind);
}

#[test]
fn test_parse_and_evaluate_two_pair() {
    let hand = parse_hand("AH AD KH KD 2C").unwrap();
    assert_eq!(hand.evaluate(), HandType::TwoPair);
}

#[test]
fn test_parse_and_evaluate_one_pair() {
    let hand = parse_hand("AH AD 2C 3S 4H").unwrap();
    assert_eq!(hand.evaluate(), HandType::OnePair);
}

#[test]
fn test_parse_and_evaluate_high_card() {
    let hand = parse_hand("AH KD 2C 3S 4H").unwrap();
    assert_eq!(hand.evaluate(), HandType::HighCard);
}

#[test]
fn test_compare_different_hand_types() {
    let royal_flush = parse_hand("AS KS QS JS 10S").unwrap();
    let four_of_a_kind = parse_hand("AH AD AC AS KH").unwrap();
    
    assert!(royal_flush > four_of_a_kind);
}

#[test]
fn test_compare_same_hand_type_high_card() {
    let ace_high = parse_hand("AH 5D 4C 3S 2H").unwrap();
    let king_high = parse_hand("KH 5D 4C 3S 2H").unwrap();
    
    assert!(ace_high > king_high);
}

#[test]
fn test_compare_same_hand_type_one_pair() {
    let aces_pair = parse_hand("AH AD 2C 3S 4H").unwrap();
    let kings_pair = parse_hand("KH KD 2S 3C 4D").unwrap();
    
    assert!(aces_pair > kings_pair);
}

#[test]
fn test_compare_same_pair_different_kickers() {
    let aces_pair_king_kicker = parse_hand("AH AD KC 3S 4H").unwrap();
    let aces_pair_queen_kicker = parse_hand("AS AC QS 3C 4D").unwrap();
    
    assert!(aces_pair_king_kicker > aces_pair_queen_kicker);
}

#[test]
fn test_invalid_hand_format() {
    let result = parse_hand("AH AD");
    assert!(result.is_err());
    
    let result = parse_hand("AH AD AC AS KH 2C");
    assert!(result.is_err());
}

#[test]
fn test_invalid_card_format() {
    let result = parse_hand("AH AD AC AS 1H");
    assert!(result.is_err());
    
    let result = parse_hand("AH AD AC AS XH");
    assert!(result.is_err());
}

#[test]
fn test_tie_hands() {
    let royal_flush_hearts = parse_hand("AH KH QH JH 10H").unwrap();
    let royal_flush_diamonds = parse_hand("AD KD QD JD 10D").unwrap();
    
    assert_eq!(royal_flush_hearts.cmp(&royal_flush_diamonds), std::cmp::Ordering::Equal);
} 