use super::{
    hand::Hand,
    hand_ranking::{HandRanking, FourOf, FullHouse, Flush, ThreeOf, TwoPair, Pair}, 
    card::{Rank, Suit}
};

pub const CARDS_IN_COMBO: usize = 5;

pub fn evaluate_five_cards(hand: &Hand) -> HandRanking {
    return has_royal_flush(hand)
        .or_else(|| has_straight_flush(hand)) 
        .or_else(|| has_four_of(hand))
        .or_else(|| has_full_house(hand))
        .or_else(|| has_flush(hand))
        .or_else(|| has_straight(hand))
        .or_else(|| has_three_of(hand))
        .or_else(|| has_pairs(hand))
        .unwrap_or(highest_card(hand));
}

fn has_royal_flush(hand: &Hand) -> Option<HandRanking> {
    let first_card = &hand.cards()[0];

    if first_card.rank() != Rank::Ace {
        return None;
    }

    let ace_suit = first_card.suit();

    for card in hand.cards().iter().skip(1).take(4) {
        if ace_suit != card.suit() {
            return None;
        }
    }

    return Some(HandRanking::RoyalFlush);
}

fn has_straight_flush(hand: &Hand) -> Option<HandRanking> {
    let mut high_card = hand.cards()[0];
    let mut previous_card = high_card;
    let mut straight_cards_count = 1;

    for card in hand.cards().iter().skip(1) {
        let expected_rank = 
            if let Some(lower) = previous_card.rank().lower() {
                lower
            } else {
                return None;
            };

        let ok = 
            card.suit() == high_card.suit() &&
            card.rank() == expected_rank;

        if !ok {
            high_card = *card;
            previous_card = *card;
            straight_cards_count = 1;
            continue;
        }

        previous_card = *card;
        straight_cards_count += 1;
    }

    if straight_cards_count >= CARDS_IN_COMBO {
        return Some(HandRanking::StraightFlush(high_card.rank()));
    }

    return None;
}

fn has_four_of(hand: &Hand) -> Option<HandRanking> {
    return hand.cards()
        .windows(4)
        .enumerate()
        .find(|(_, four)| {
            let first = four[0];
            return four.iter().skip(1).all(|card| card.rank() == first.rank());
        })
        .map(|(i, four)| HandRanking::FourOf(
            FourOf::new(
                four[0].rank(), 
                hand.cards()[if i == 0 { 4 } else { 0 }].rank()
            )
        ));
}

fn has_full_house(hand: &Hand) -> Option<HandRanking> {
    let maybe_three_of = hand.cards()
        .windows(3)
        .find(|trey| trey[0].rank() == trey[1].rank() && trey[0].rank() == trey[2].rank());

    return maybe_three_of.and_then(|three_of| {
        let three_of_rank = three_of[0].rank();
        let maybe_pair = hand.cards()
            .windows(2)
            .find(|pair| pair[0].rank() == pair[1].rank() && pair[0].rank() != three_of_rank);

        if let Some(pair) = maybe_pair {
            let full_house = FullHouse::new(pair[0].rank(), three_of_rank);
            return Some(HandRanking::FullHouse(full_house));
        }

        return None;
    });
}

fn has_flush(hand: &Hand) -> Option<HandRanking> {
    return has_flush_of(Suit::Diamonds, hand)
        .or_else(|| has_flush_of(Suit::Clubs, hand))
        .or_else(|| has_flush_of(Suit::Hearts, hand)) 
        .or_else(|| has_flush_of(Suit::Spades, hand));
}

fn has_flush_of(suit: Suit, hand: &Hand) -> Option<HandRanking> {
    let maybe_flush: Vec<_> = hand.cards().iter()
        .filter(|card| card.suit() == suit)
        .map(|card| card.rank())
        .take(CARDS_IN_COMBO)
        .collect();

    if maybe_flush.len() >= CARDS_IN_COMBO {
        return Some(HandRanking::Flush(
            Flush::new(maybe_flush.try_into().unwrap())
        ));
    }
    
    return None;
}

fn has_straight(hand: &Hand) -> Option<HandRanking> {
    let mut cards = hand.cards().map(|c| c.rank()).to_vec();
    cards.dedup();

    if cards.len() < CARDS_IN_COMBO {
        return None;
    }

    return cards
        .windows(CARDS_IN_COMBO)
        .find(|five| five.windows(2).all(|pair| pair[0].lower().is_some_and(|lower| lower == pair[1])))
        .map(|straight| HandRanking::Straight(straight[0]))
        .or_else(|| {
            let c1 = cards[0];
            let c2 = cards[cards.len() - 1];
            let c3 = cards[cards.len() - 2];
            let c4 = cards[cards.len() - 3];
            let c5 = cards[cards.len() - 4];
            
            // Check for Ace low straight
            if c2 == Rank::Two && c1 == Rank::Ace && c5 == Rank::Five && c4 == Rank::Four && c3 == Rank::Three {
                return Some(HandRanking::Straight(Rank::Five));
            }

            return None;
        });
}

fn has_three_of(hand: &Hand) -> Option<HandRanking> {
    return hand.cards()
        .windows(3)
        .find(|trey| trey[0].rank() == trey[1].rank() && trey[0].rank() == trey[2].rank())
        .map(|trey| {
            let three_of_rank = trey[0].rank();
            let kickers = hand.cards().iter()
                .filter(|card| card.rank() != three_of_rank)
                .take(2)
                .map(|card| card.rank())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            return HandRanking::ThreeOf(ThreeOf::new(trey[0].rank(), kickers));
        });
}

fn has_pairs(hand: &Hand) -> Option<HandRanking> {
    let pairs = find_all_pairs(hand);

    if pairs.is_empty() {
        return None;
    }

    if pairs.len() == 1 {
        let kickers = hand.cards().iter()
            .filter(|card| card.rank() != pairs[0])
            .take(3)
            .map(|card| card.rank())
            .collect::<Vec<_>>();
    
        return Some(HandRanking::Pair(
            Pair::new(pairs[0], kickers.try_into().unwrap())
        ));
    }

    let kicker = hand.cards().iter()
        .find(|card| card.rank() != pairs[0] && card.rank() != pairs[1])
        .map(|card| card.rank())
        .unwrap();

    return Some(HandRanking::TwoPair(
        TwoPair::new(pairs[0..2].try_into().unwrap(), kicker)
    ));
}

/// Return all pairs in descending order by rank
fn find_all_pairs(hand: &Hand) -> Vec<Rank> {
    return hand.cards()
        .windows(2)
        .filter(|pair| pair[0].rank() == pair[1].rank())
        .map(|pair| pair[0].rank())
        .collect();
}

fn highest_card(hand: &Hand) -> HandRanking {
    return HandRanking::HighCard(
        hand.cards().map(|card| card.rank())[0..5].try_into().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use crate::naive_evaluator::{
        card::{Card, Rank, Suit}, 
        hand::Hand,
        hand_ranking::{HandRanking, FourOf, Flush, ThreeOf, TwoPair, Pair, FullHouse}
    };

    use super::evaluate_five_cards;

    #[test]
    fn test_flush_royal() {
        let hand = Hand::new([
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::RoyalFlush);
    }

    #[test]
    fn test_straight_flush() {
        let hand = Hand::new([
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::StraightFlush(Rank::Ten));
    }

    #[test]
    fn test_four_of() {
        let hand = Hand::new([
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::King, Suit::Spades)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::FourOf(FourOf::new(Rank::Nine, Rank::Ace)));
    }

    #[test]
    fn test_flush() {
        let hand = Hand::new([
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::Flush(
            Flush::new([Rank::King, Rank::Ten, Rank::Nine, Rank::Eight, Rank::Two])
        ));
    }

    #[test]
    fn test_straight() {
        let hand = Hand::new([
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::Straight(Rank::Six));
        
        let hand = Hand::new([
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::Straight(Rank::Five));
        
        let hand = Hand::new([
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::Straight(Rank::Ace));
    }

    #[test]
    fn test_three_of() {
        let hand = Hand::new([
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::ThreeOf(
            ThreeOf::new(Rank::Two, [Rank::King, Rank::Eight])
        ));
    }

    #[test]
    fn test_two_pair() {
        let hand = Hand::new([
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::TwoPair(
            TwoPair::new([Rank::Eight, Rank::Three], Rank::King)
        ));
    }

    #[test]
    fn test_pair() {
        let hand = Hand::new([
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::Pair(
            Pair::new(Rank::Three, [Rank::Ace, Rank::King, Rank::Eight])
        ));
    }

    #[test]
    fn test_high_card() {
        let hand = Hand::new([
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Five, Suit::Spades)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::HighCard(
            [Rank::Queen, Rank::Ten, Rank::Eight, Rank::Seven, Rank::Five]
        ));
    }

    #[test]
    fn test_real_world_cases() {
        let hand = Hand::new([
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Diamonds)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::FullHouse(
            FullHouse::new(Rank::Six, Rank::Nine)
        ));

        let hand = Hand::new([
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Diamonds)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::TwoPair(
            TwoPair::new([Rank::Nine, Rank::Six], Rank::Ace)
        ));

        let hand = Hand::new([
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::Pair(
            Pair::new(Rank::Nine, [Rank::King, Rank::Queen, Rank::Six])
        ));
        
        let hand = Hand::new([
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs)
        ]);
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::Pair(
            Pair::new(Rank::Nine, [Rank::Queen, Rank::Jack, Rank::Six])
        ));
    }

    #[test]
    fn test_straight_with_duplicated_card() {
        let hand = Hand::try_from("7h6d5h4c4d3h2c").unwrap();
        let ranking = evaluate_five_cards(&hand);
        assert_eq!(ranking, HandRanking::Straight(Rank::Seven));
    }
}
