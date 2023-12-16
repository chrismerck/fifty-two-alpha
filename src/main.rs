/*
 * A Go-Fish Game
 */

mod french;
use crate::french::{Card, Suit, Number, Deck, Hand};

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let mut hand = Hand::new();
    for _ in 0..7 {
        hand.add(deck.deal().unwrap());
    }
    println!("Your Hand: {}", hand);
}
