
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two,
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
    Ace
}

impl Rank {
    pub fn lower(&self) -> Option<Self> {
        if self != &Rank::Two {
            unsafe { return Some(std::mem::transmute(*self as u8 - 1)); }
        } else {
            return None;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    rank: Rank,
    suit: Suit
}

impl Card {
    #[inline]
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        return Self {
            rank,
            suit
        };
    }

    #[inline]
    pub const fn rank(&self) -> Rank {
        return self.rank;
    }

    #[inline]
    pub const fn suit(&self) -> Suit {
        return self.suit;
    }
}
