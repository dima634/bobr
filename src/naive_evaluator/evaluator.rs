use super::{hand::Hand, hand_ranking::HandRanking, card::{Rank, Suit, Card}};

pub fn evaluate_five_cards<const HAND_SIZE: usize>(hand: &Hand<HAND_SIZE>) -> HandRanking {
  return has_royal_flush(hand)
    .or(has_straight_flush(hand)) 
    .or(has_four_of(hand))
    .or(has_full_house(hand))
    .or(has_flush(hand))
    .or(has_straight(hand))
    .unwrap_or(HandRanking::Pair(Rank::Ace));
}

fn has_royal_flush<const HAND_SIZE: usize>(hand: &Hand<HAND_SIZE>) -> Option<HandRanking> {
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

fn has_straight_flush<const HAND_SIZE: usize>(hand: &Hand<HAND_SIZE>) -> Option<HandRanking> {
    let mut high_card = hand.cards().last().unwrap();
    let mut previous_card = high_card;
    let mut straight_cards_count = 1;

    for card in hand.cards().iter().rev().skip(1) {
        let ok = 
            card.suit() == high_card.suit() &&
            card.rank() == previous_card.rank().lower();

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

fn has_four_of<const HAND_SIZE: usize>(hand: &Hand<HAND_SIZE>) -> Option<HandRanking> {
    return hand.cards()
        .windows(4)
        .find(|four| {
            let first = four[0];
            return four.iter().skip(1).all(|card| card.rank() == first.rank());
        })
        .map(|four| HandRanking::FourOf(four[0].rank()));
}

fn has_full_house<const HAND_SIZE: usize>(hand: &Hand<HAND_SIZE>) -> Option<HandRanking> {
    let three_of = hand.cards()
        .windows(3)
        .find(|three| three[0].rank() == three[1].rank() && three[0].rank() == three[2].rank());

    return three_of.and_then(|three_of| {
        let has_pair = hand.cards()
            .windows(2)
            .any(|pair| pair[0].rank() == pair[1].rank() && pair[0].rank() != three_of[0].rank());

        if has_pair {
            return Some(HandRanking::FullHouse(three_of[0].rank()));
        }

        return None;
    });
}

fn has_flush<const HAND_SIZE: usize>(hand: &Hand<HAND_SIZE>) -> Option<HandRanking> {
    return has_flush_of(Suit::Diamonds, hand)
        .or(has_flush_of(Suit::Clubs, hand))
        .or(has_flush_of(Suit::Hearts, hand)) 
        .or(has_flush_of(Suit::Spades, hand));
}

fn has_flush_of<const HAND_SIZE: usize>(suit: Suit, hand: &Hand<HAND_SIZE>) -> Option<HandRanking> {
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

fn has_straight<const HAND_SIZE: usize>(hand: &Hand<HAND_SIZE>) -> Option<HandRanking> {
    let mut high_card_rank = hand.cards().last().unwrap().rank();
    let mut previous_rank = high_card_rank;
    let mut count = 1;

    for card in hand.cards().iter().rev().skip(1) {
        if card.rank() != previous_rank.lower() {
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
}
