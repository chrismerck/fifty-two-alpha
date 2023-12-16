use std::fmt;

pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Clubs => write!(f, "♣︎"),
            Suit::Diamonds => write!(f, "♦︎"),
            Suit::Hearts => write!(f, "♥︎"),
            Suit::Spades => write!(f, "♠︎"),
        }
    }
}

pub enum Number {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Number::Ace => write!(f, "A"),
            Number::Two => write!(f, "2"),
            Number::Three => write!(f, "3"),
            Number::Four => write!(f, "4"),
            Number::Five => write!(f, "5"),
            Number::Six => write!(f, "6"),
            Number::Seven => write!(f, "7"),
            Number::Eight => write!(f, "8"),
            Number::Nine => write!(f, "9"),
            Number::Ten => write!(f, "10"),
            Number::Jack => write!(f, "J"),
            Number::Queen => write!(f, "Q"),
            Number::King => write!(f, "K"),
        }
    }
}
pub struct Card {
    suit: Suit,
    number: Number,
}

impl Card {
  pub fn new(suit: Suit, number: Number) -> Card {
    Card {
      suit,
      number,
    }
  }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.number, self.suit)
    }
}
