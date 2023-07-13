use std::fmt::Display;

use super::card::Card;

pub const HAND_SIZE: usize = 7;

#[derive(Debug)]
pub struct Hand {
    // Cards in descending rank order
    cards: [Card; HAND_SIZE]
}

impl Hand {
    #[inline]
    pub fn new(mut cards: [Card; HAND_SIZE]) -> Self {
        cards.sort_unstable_by(|c1, c2| c1.rank().cmp(&c2.rank()).reverse());
        return Self { cards };
    }

    #[inline]
    pub const fn cards(&self) -> &[Card; HAND_SIZE] {
        return &self.cards;
    }
}

impl From<&Hand> for String {
    #[inline]
    fn from(value: &Hand) -> Self {
        return format!("{}{}{}{}{}{}{}", value.cards[0], value.cards[1], value.cards[2], value.cards[3], value.cards[4], value.cards[5], value.cards[6]);
    }
}

impl TryFrom<&str> for Hand {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 14 {
            return Err(());
        }
        
        let c1 = Card::try_from(&value[0..2])?;
        let c2 = Card::try_from(&value[2..4])?;
        let c3 = Card::try_from(&value[4..6])?;
        let c4 = Card::try_from(&value[6..8])?;
        let c5 = Card::try_from(&value[8..10])?;
        let c6 = Card::try_from(&value[10..12])?;
        let c7 = Card::try_from(&value[12..14])?;
        return Ok(Hand::new([c1, c2, c3, c4, c5, c6, c7]));
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", String::from(self));
    }
}
