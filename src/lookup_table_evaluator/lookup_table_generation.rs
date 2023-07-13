use crate::naive_evaluator::{deck::Deck, card::Card, hand::{HAND_SIZE, Hand}};

// const HAND_PERMUTATIONS_COUNT: usize = 674_274_182_400;
pub const HAND_COMBINATIONS_COUNT: usize = 133_784_560;

pub struct HandsGenerator {
    hands: Vec<Hand>,
    hand: Vec<Card>,
    deck: Deck,
    used_cards: [bool; Deck::SIZE]
}

impl HandsGenerator {
    pub fn new() -> Self {
        return Self {
            deck: Deck::new(),
            hands: Vec::with_capacity(HAND_COMBINATIONS_COUNT),
            used_cards: [false; Deck::SIZE],
            hand: Vec::with_capacity(HAND_SIZE)
        };
    }

    #[inline]
    pub fn hands(&self) -> &Vec<Hand> {
        return &self.hands;
    }

    pub fn generate(&mut self) {
        self.generate_i(0);
    }

    pub fn generate_i(&mut self, index: usize) {
        if self.hand.len() == 7 {
            self.hands.push(Hand::new(self.hand.clone().try_into().unwrap()));
            return;
        }

        for i in index..Deck::SIZE {
            if self.used_cards[i] {
                continue;
            }

            self.used_cards[i] = true;
            self.hand.push(self.deck.cards()[i]);

            self.generate_i(i + 1);

            self.used_cards[i] = false;      
            self.hand.pop();
        }  

        if index == 4 {
            println!("{}", (self.hands.len() as f32) / (HAND_COMBINATIONS_COUNT as f32));
        }
    }
}
