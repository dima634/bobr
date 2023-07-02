use super::card::Card;

pub const HAND_SIZE: usize = 7;

pub struct Hand {
    cards: [Card; HAND_SIZE]
}


impl Hand {
    #[inline]
    pub fn new(mut cards: [Card; HAND_SIZE]) -> Self {
        cards.sort_by_key(|card| card.rank());
        return Self { cards };
    }

    #[inline]
    pub const fn cards(&self) -> &[Card; HAND_SIZE] {
        return &self.cards;
    }
}
