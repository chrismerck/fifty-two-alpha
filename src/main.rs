/*
 * A Go-Fish Game
 */

mod french;
use crate::french::{Card, Suit, Number};

fn main() {
    let card = Card::new(Suit::Clubs, Number::Ace);
    println!("You have a {}", card);
}
