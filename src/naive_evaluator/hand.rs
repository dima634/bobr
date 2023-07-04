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
