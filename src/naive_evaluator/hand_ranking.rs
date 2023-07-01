use super::card::Rank;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandRanking {
    HighCard(Rank),
    Pair(Rank),
    TwoPair(Rank),
    ThreeOf(Rank),
    Straight(Rank),
    Flush(Rank),
    FullHouse(Rank),
    FourOf(Rank),
    StraightFlush(Rank),
    RoyalFlush
}
