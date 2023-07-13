use rand::Rng;

use super::card::{Card, Suit, Rank};

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub const SIZE: usize = 52;

    pub fn new() -> Self {
        let mut cards = Vec::new();
        cards.reserve(Self::SIZE);

        for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for rank in [
                Rank::Two, Rank::Three, Rank::Four, Rank::Five, 
                Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, 
                Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, 
                Rank::Ace
            ] {
                cards.push(Card::new(rank, suit));
            }
        }
        
        return Self { cards };
    }

    #[inline]
    pub fn shuffled(mut self) -> Self {
        self.shuffle();
        return self;
    }

    #[inline]
    pub fn cards(&self) -> &Vec<Card> {
        return &self.cards;
    }

    #[inline]
    pub fn draw_card(&mut self) -> Option<Card> {
        return self.cards.pop();
    }

    #[inline]
    pub fn put_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn shuffle(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();

        for i in 0..Self::SIZE - 2 {
            let j = rng.gen_range(i..Self::SIZE);
            self.cards.swap(i, j);
        }

        return self;
    }
}
