use bobr::{naive_evaluator::{deck::Deck, hand::Hand, evaluator::evaluate_five_cards}, lookup_table_evaluator::lookup_table_generation::{HandsGenerator, HAND_COMBINATIONS_COUNT}};

fn main() {
    let mut hands_generator = HandsGenerator::new();
    hands_generator.generate();

    for i in 0..hands_generator.hands().len() {
        let hand = &hands_generator.hands()[i];
        let combo =  evaluate_five_cards(hand);
        
        if i % 50000 == 0 {
            println!("{}\t{}\t\t{}", hand, combo, i as f32 / HAND_COMBINATIONS_COUNT as f32);
        }
    }

    println!("{}", hands_generator.hands().len());
}
