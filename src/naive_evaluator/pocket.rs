use super::card::Card;

pub struct Pocket {
    cards: [Card; 2]
}

impl Pocket {
    #[inline]
    pub const fn new(card1: Card, card2: Card) -> Self {
        return Self {
            cards: [card1, card2]
        };
    }

    #[inline]
    pub const fn card1(&self) -> Card {
        return self.cards[0];
    }

    #[inline]
    pub const fn card2(&self) -> Card {
        return self.cards[1];
    }
}
