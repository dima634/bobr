use super::card::Card;

pub struct Hand<const SIZE: usize> {
    cards: [Card; SIZE]
}

impl<const SIZE: usize> Hand<SIZE> {
    #[inline]
    pub fn new(mut cards: [Card; SIZE]) -> Self {
        cards.sort_by_key(|card| card.rank());
        return Self { cards };
    }

    #[inline]
    pub const fn cards(&self) -> &[Card; SIZE] {
        return &self.cards;
    }
}
