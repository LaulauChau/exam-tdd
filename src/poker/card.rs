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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_ordering() {
        assert!(Rank::Ace > Rank::King);
        assert!(Rank::King > Rank::Queen);
        assert!(Rank::Queen > Rank::Jack);
        assert!(Rank::Jack > Rank::Ten);
        assert!(Rank::Ten > Rank::Nine);
        assert!(Rank::Nine > Rank::Eight);
        assert!(Rank::Eight > Rank::Seven);
        assert!(Rank::Seven > Rank::Six);
        assert!(Rank::Six > Rank::Five);
        assert!(Rank::Five > Rank::Four);
        assert!(Rank::Four > Rank::Three);
        assert!(Rank::Three > Rank::Two);
    }

    #[test]
    fn test_hand_type_ordering() {
        assert!(HandType::RoyalFlush > HandType::StraightFlush);
        assert!(HandType::StraightFlush > HandType::FourOfAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::Flush);
        assert!(HandType::Flush > HandType::Straight);
        assert!(HandType::Straight > HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }
} 