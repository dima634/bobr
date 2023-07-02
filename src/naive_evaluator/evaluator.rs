use super::{
    hand::{Hand, HAND_SIZE}, 
    hand_ranking::HandRanking, 
    card::{Rank, Suit}
};

pub fn evaluate_five_cards(hand: &Hand) -> HandRanking {
  return has_royal_flush(hand)
    .or(has_straight_flush(hand)) 
    .or(has_four_of(hand))
    .or(has_full_house(hand))
    .or(has_flush(hand))
    .or(has_straight(hand))
    .or(has_three_of(hand))
    .or(has_pairs(hand))
    .unwrap_or(highest_card(hand));
}

fn has_royal_flush(hand: &Hand) -> Option<HandRanking> {
    let last_card = &hand.cards()[HAND_SIZE - 1];

    if last_card.rank() != Rank::Ace {
        return None;
    }

    let ace_suit = last_card.suit();

    for card in hand.cards().iter().rev().skip(1).take(4) {
        if ace_suit != card.suit() {
            return None;
        }
    }

    return Some(HandRanking::RoyalFlush);
}

fn has_straight_flush(hand: &Hand) -> Option<HandRanking> {
    let mut high_card = hand.cards().last().unwrap();
    let mut previous_card = high_card;
    let mut straight_cards_count = 1;

    for card in hand.cards().iter().rev().skip(1) {
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
            high_card = card;
            previous_card = card;
            straight_cards_count = 1;
            continue;
        }

        previous_card = card;
        straight_cards_count += 1;
    }

    if straight_cards_count >= 5 {
        return Some(HandRanking::StraightFlush(high_card.rank()));
    }

    return None;
}

fn has_four_of(hand: &Hand) -> Option<HandRanking> {
    return hand.cards()
        .windows(4)
        .find(|four| {
            let first = four[0];
            return four.iter().skip(1).all(|card| card.rank() == first.rank());
        })
        .map(|four| HandRanking::FourOf(four[0].rank()));
}

fn has_full_house(hand: &Hand) -> Option<HandRanking> {
    let three_of = has_three_of(hand);

    return three_of.and_then(|three_of| {
        let three_of_rank = match three_of {
            HandRanking::ThreeOf(rank) => rank,
            _ => unreachable!()
        };
        let has_pair = hand.cards()
            .windows(2)
            .any(|pair| pair[0].rank() == pair[1].rank() && pair[0].rank() != three_of_rank);

        if has_pair {
            return Some(HandRanking::FullHouse(three_of_rank));
        }

        return None;
    });
}

fn has_flush(hand: &Hand) -> Option<HandRanking> {
    return has_flush_of(Suit::Diamonds, hand)
        .or(has_flush_of(Suit::Clubs, hand))
        .or(has_flush_of(Suit::Hearts, hand)) 
        .or(has_flush_of(Suit::Spades, hand));
}

fn has_flush_of(suit: Suit, hand: &Hand) -> Option<HandRanking> {
    let mut high_rank = hand.cards().first().unwrap().rank();
    let mut count = 0;

    for card in hand.cards().iter().skip(1) {
        if card.suit() != suit {
            continue;
        }

        if high_rank < card.rank() {
            high_rank = card.rank();
        }

        count += 1;
    }

    if count >= 4 {
        return Some(HandRanking::Flush(high_rank));
    }
    
    return None;
}

fn has_straight(hand: &Hand) -> Option<HandRanking> {
    let mut high_card_rank = hand.cards().last().unwrap().rank();
    let mut previous_rank = high_card_rank;
    let mut count = 1;

    for card in hand.cards().iter().rev().skip(1) {
        let expected_rank = 
            if let Some(lower) = previous_rank.lower() {
                lower
            } else {
                return None;
            };

        if card.rank() != expected_rank {
            high_card_rank = card.rank();
            previous_rank = card.rank();
            count = 1;
            continue;
        }

        count += 1;
        previous_rank = card.rank();
    }

    if count >= 5 {
        return Some(HandRanking::Straight(high_card_rank));
    }

    return None;
}

fn has_three_of(hand: &Hand) -> Option<HandRanking> {
    return hand.cards()
        .windows(3)
        .find(|trey| trey[0].rank() == trey[1].rank() && trey[0].rank() == trey[2].rank())
        .map(|trey| HandRanking::ThreeOf(trey[0].rank()));
}

fn has_pairs(hand: &Hand) -> Option<HandRanking> {
    let pairs = find_all_pairs(hand);

    if pairs.is_empty() {
        return None;
    }

    if pairs.len() == 1 {
        return Some(HandRanking::Pair(pairs[0]));
    }

    return Some(HandRanking::TwoPair(pairs[pairs.len() - 1])); // Last pair has highest rank
}

/// Return all pairs in ascending order by rank
fn find_all_pairs(hand: &Hand) -> Vec<Rank> {
    return hand.cards()
        .windows(2)
        .filter(|pair| pair[0].rank() == pair[1].rank())
        .map(|pair| pair[0].rank())
        .collect();
}

fn highest_card(hand: &Hand) -> HandRanking {
    return HandRanking::HighCard(hand.cards().last().unwrap().rank());
}

#[cfg(test)]
mod tests {
    use crate::naive_evaluator::{
        card::{Card, Rank, Suit}, 
        hand::Hand, 
        hand_ranking::HandRanking
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
        assert_eq!(ranking, HandRanking::FourOf(Rank::Nine));
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
        assert_eq!(ranking, HandRanking::Flush(Rank::King));
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
        assert_eq!(ranking, HandRanking::ThreeOf(Rank::Two));
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
        assert_eq!(ranking, HandRanking::TwoPair(Rank::Eight));
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
        assert_eq!(ranking, HandRanking::Pair(Rank::Three));
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
        assert_eq!(ranking, HandRanking::HighCard(Rank::Queen));
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
        assert_eq!(ranking, HandRanking::FullHouse(Rank::Nine));

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
        assert_eq!(ranking, HandRanking::TwoPair(Rank::Nine));

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
        assert_eq!(ranking, HandRanking::Pair(Rank::Nine));
        
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
        assert_eq!(ranking, HandRanking::Pair(Rank::Nine));
    }
}
