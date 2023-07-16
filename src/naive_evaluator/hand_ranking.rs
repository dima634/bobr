use std::fmt::Display;

use super::card::Rank;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct FourOf {
    rank: Rank,
    kicker: Rank
}

impl FourOf {
    #[inline]
    pub fn new(rank: Rank, kicker: Rank) -> Self { 
        return Self { rank, kicker };
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct FullHouse {
    three_of_rank: Rank,
    pair_of_rank: Rank
}

impl FullHouse {
    #[inline]
    pub fn new(pair_of_rank: Rank, three_of_rank: Rank) -> Self { 
        return Self { three_of_rank, pair_of_rank };
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Flush {
    ranks: [Rank; 5]
}

impl Flush {
    /// `ranks` - card ranks in descending order
    #[inline]
    pub fn new(ranks: [Rank; 5]) -> Self {
        debug_assert!(ranks.windows(2).all(|pair| pair[0] > pair[1]));
        return Self { ranks };
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ThreeOf {
    three_of: Rank,
    kickers: [Rank; 2]
}

impl ThreeOf {
    /// `kickers` - side card ranks in descending order
    #[inline]
    pub fn new(three_of: Rank, kickers: [Rank; 2]) -> Self {
        debug_assert!(kickers[0] > kickers[1]);
        return Self { three_of, kickers };
    }

    #[inline]
    pub fn rank(&self) -> Rank {
        return self.three_of;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct TwoPair {
    pairs: [Rank; 2],
    kicker: Rank
}

impl TwoPair {
    /// `pairs` - two pairs in descending order
    #[inline]
    pub fn new(pairs: [Rank; 2], kicker: Rank) -> Self {
        debug_assert!(pairs[0] > pairs[1]);
        return Self { pairs, kicker };
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Pair {
    pair_rank: Rank,
    kickers: [Rank; 3]
}

impl Pair {
    /// `kickers` - side cards in descending order
    #[inline]
    pub fn new(pair_rank: Rank, kickers: [Rank; 3]) -> Self {
        debug_assert!(kickers.windows(2).all(|pair| pair[0] > pair[1]));
        return Self { pair_rank, kickers };
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandRanking {
    HighCard([Rank; 5]),
    Pair(Pair),
    TwoPair(TwoPair),
    ThreeOf(ThreeOf),
    Straight(Rank),
    Flush(Flush),
    FullHouse(FullHouse),
    FourOf(FourOf),
    StraightFlush(Rank)
}

impl From<&HandRanking> for String {
    #[inline]
    fn from(value: &HandRanking) -> Self {
        return match value {
            HandRanking::HighCard(high)         => format!("High card {}", high[0]),
            HandRanking::Pair(pair)             => format!("Pair {}", pair.pair_rank),
            HandRanking::TwoPair(two_pair)      => format!("Two pair {}{}", two_pair.pairs[0], two_pair.pairs[1]),
            HandRanking::ThreeOf(trey)          => format!("Three of {}", trey.three_of),
            HandRanking::Straight(high)         => format!("Straight {}", high),
            HandRanking::Flush(flush)           => format!("Flush {}", flush.ranks[0]),
            HandRanking::FullHouse(full_house)  => format!("Full house {}{}", full_house.three_of_rank, full_house.pair_of_rank),
            HandRanking::FourOf(four)           => format!("Four {}", four.rank),
            HandRanking::StraightFlush(high)    => if *high == Rank::Ace { String::from("Royal flush") } else { format!("Straight flush {}", high) },
        };
    }
}

impl Display for HandRanking {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", String::from(self));
    }
}

#[cfg(test)]
mod tests {
    use crate::naive_evaluator::{hand_ranking::{HandRanking, FourOf, FullHouse, Flush, ThreeOf, TwoPair, Pair}, card::Rank};

    #[test]
    fn test_breaking_four_of_tie() {
        assert_eq!(
            HandRanking::FourOf(FourOf::new(Rank::Jack, Rank::Seven)),
            HandRanking::FourOf(FourOf::new(Rank::Jack, Rank::Seven))
        );

        assert!(
            HandRanking::FourOf(FourOf::new(Rank::Jack, Rank::Two)) <
            HandRanking::FourOf(FourOf::new(Rank::Jack, Rank::Three))
        )
    }

    #[test]
    fn test_breaking_straight_flush_tie() {
        assert!(
            HandRanking::StraightFlush(Rank::Jack) >
            HandRanking::StraightFlush(Rank::Ten)
        );
    }
    
    #[test]
    fn test_breaking_full_house_tie() {
        assert!(
            HandRanking::FullHouse(FullHouse::new(Rank::Jack, Rank::Two)) <
            HandRanking::FullHouse(FullHouse::new(Rank::Jack, Rank::Three))
        );

        assert!(
            HandRanking::FullHouse(FullHouse::new(Rank::Jack, Rank::Three)) >
            HandRanking::FullHouse(FullHouse::new(Rank::Ten, Rank::Three))
        );
    }

    #[test]
    fn test_breaking_flush_tie() {
        assert_eq!(
            HandRanking::Flush(Flush::new([Rank::Jack, Rank::Six, Rank::Five, Rank::Four, Rank::Three])),
            HandRanking::Flush(Flush::new([Rank::Jack, Rank::Six, Rank::Five, Rank::Four, Rank::Three]))
        );

        assert!(
            HandRanking::Flush(Flush::new([Rank::Jack, Rank::Nine, Rank::Six, Rank::Five, Rank::Four])) >
            HandRanking::Flush(Flush::new([Rank::Jack, Rank::Eight, Rank::Six, Rank::Five, Rank::Four]))
        );
    }

    #[test]
    fn test_breaking_three_of_tie() {
        assert_eq!(
            HandRanking::ThreeOf(ThreeOf::new(Rank::Ace, [Rank::King, Rank::Queen])),
            HandRanking::ThreeOf(ThreeOf::new(Rank::Ace, [Rank::King, Rank::Queen]))
        );

        assert!(
            HandRanking::ThreeOf(ThreeOf::new(Rank::Ace, [Rank::King, Rank::Jack])) <
            HandRanking::ThreeOf(ThreeOf::new(Rank::Ace, [Rank::King, Rank::Queen]))
        );

        assert!(
            HandRanking::ThreeOf(ThreeOf::new(Rank::Ace, [Rank::King, Rank::Jack])) >
            HandRanking::ThreeOf(ThreeOf::new(Rank::Two, [Rank::King, Rank::Queen]))
        );
    }

    #[test]
    fn test_breaking_tow_pair_tie() {
        assert_eq!(
            HandRanking::TwoPair(TwoPair::new([Rank::King, Rank::Queen], Rank::Jack)),
            HandRanking::TwoPair(TwoPair::new([Rank::King, Rank::Queen], Rank::Jack))
        );
        
        assert!(
            HandRanking::TwoPair(TwoPair::new([Rank::Ten, Rank::Nine], Rank::Jack)) >
            HandRanking::TwoPair(TwoPair::new([Rank::Ten, Rank::Two], Rank::Jack))
        );
        
        assert!(
            HandRanking::TwoPair(TwoPair::new([Rank::Ten, Rank::Two], Rank::Jack)) >
            HandRanking::TwoPair(TwoPair::new([Rank::Ten, Rank::Two], Rank::Eight))
        );
        
        assert!(
            HandRanking::TwoPair(TwoPair::new([Rank::Queen, Rank::Two], Rank::Jack)) >
            HandRanking::TwoPair(TwoPair::new([Rank::Ten, Rank::Two], Rank::Eight))
        );
    }

    #[test]
    fn test_breaking_pair_tie() {
        assert!(
            HandRanking::Pair(Pair::new(Rank::Ace, [Rank::King, Rank::Jack, Rank::Two])) >
            HandRanking::Pair(Pair::new(Rank::Eight, [Rank::King, Rank::Queen, Rank::Two]))
        );

        assert!(
            HandRanking::Pair(Pair::new(Rank::Ace, [Rank::King, Rank::Jack, Rank::Two])) <
            HandRanking::Pair(Pair::new(Rank::Ace, [Rank::King, Rank::Queen, Rank::Two]))
        );
    }

    #[test]
    fn test_high_card() {
        assert!(
            HandRanking::HighCard([Rank::King, Rank::Queen, Rank::Four, Rank::Three, Rank::Two]) >
            HandRanking::HighCard([Rank::King, Rank::Jack, Rank::Four, Rank::Three, Rank::Two])
        );
    }
}
