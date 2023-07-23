use std::fmt::Display;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades
}

impl TryFrom<char> for Suit {
    type Error = ();

    #[inline]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        return match value {
            'c' => Ok(Suit::Clubs),
            'd' => Ok(Suit::Diamonds),
            'h' => Ok(Suit::Hearts),
            's' => Ok(Suit::Spades),
            _  => Err(()) 
        };
    }
}

impl From<Suit> for char {
    fn from(value: Suit) -> Self {
        return match value {
            Suit::Clubs     => 'c',
            Suit::Diamonds  => 'd',
            Suit::Hearts    => 'h',
            Suit::Spades    => 's',
        };
    }
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

impl TryFrom<char> for Rank {
    type Error = ();

    #[inline]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        return match value {
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            '9' => Ok(Rank::Nine),
            'T' => Ok(Rank::Ten),
            'J' => Ok(Rank::Jack),
            'Q' => Ok(Rank::Queen),
            'K' => Ok(Rank::King),
            'A' => Ok(Rank::Ace),
            _ => Err(())
        };
    }
}

impl From<Rank> for char {
    fn from(value: Rank) -> Self {
        return match value {
            Rank::Two   => '2',
            Rank::Three => '3',
            Rank::Four  => '4',
            Rank::Five  => '5',
            Rank::Six   => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine  => '9',
            Rank::Ten   => 'T',
            Rank::Jack  => 'J',
            Rank::Queen => 'Q',
            Rank::King  => 'K',
            Rank::Ace   => 'A',
        };
    }
}

impl Display for Rank {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", char::from(*self));
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<&Card> for String {
    #[inline]
    fn from(value: &Card) -> Self {
        return format!("{}{}", char::from(value.rank), char::from(value.suit));
    }
}

impl TryFrom<&str> for Card {
    type Error = ();

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(());
        }

        let rank = Rank::try_from(value.chars().nth(0).unwrap())?;
        let suit = Suit::try_from(value.chars().nth(1).unwrap())?;
        return Ok(Card::new(rank, suit));
    }
}

impl Display for Card {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", String::from(self));
    }
}
