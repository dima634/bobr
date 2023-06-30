use super::card::{Suit, Rank, Card};

#[derive(PartialEq, Eq)]
pub enum HandRanking {
    RoyalFlush(Suit),
    StraightFlush(Rank),
    FourOf(Rank),
    FullHouse(Rank),
    Flush(Rank),
    Straight(Rank),
    ThreeOf(Rank),
    TwoPair(Rank),
    Pair(Rank),
    HighCard(Rank)
}
