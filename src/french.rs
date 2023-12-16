use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    pub suit: Suit,
    pub number: Number,
}

impl Card {
  pub fn new(suit: Suit, number: Number) -> Card {
    Card {
      suit,
      number,
    }
  }

  pub fn from_string(s: &str) -> Card {
    let mut chars = s.chars();
    // AH ... 10H ... KH AC ... KC AD ... KD AS ... KS
    let number = match chars.next().unwrap() {
      'A' => Number::Ace,
      '2' => Number::Two,
      '3' => Number::Three,
      '4' => Number::Four,
      '5' => Number::Five,
      '6' => Number::Six,
      '7' => Number::Seven,
      '8' => Number::Eight,
      '9' => Number::Nine,
      '1' => Number::Ten,
      'J' => Number::Jack,
      'Q' => Number::Queen,
      'K' => Number::King,
      _ => panic!("Invalid card number: {}", s),
    };
    // remove 0 if present
    if number == Number::Ten {
      chars.next();
    }
    let suit = match chars.next().unwrap() {
      'H' => Suit::Hearts,
      'C' => Suit::Clubs,
      'D' => Suit::Diamonds,
      'S' => Suit::Spades,
      _ => panic!("Invalid card suit: {}", s),
    };
    Card {
      suit,
      number,
    }
  }

  pub fn value(&self, ace_high: bool) -> usize {
    match self.number {
      Number::Ace => if ace_high { 15 } else { 5 },
      Number::Jack => 10,
      Number::Queen => 10,
      Number::King => 10,
      _ => 5,
    }
  }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.number, self.suit)
    }
}

pub struct Deck {
  cards: Vec<Card>,
}

impl Deck {
  pub fn new() -> Deck {
    let mut cards = Vec::new();
    for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades].iter() {
      for number in [Number::Ace, Number::Two, Number::Three, Number::Four, Number::Five, Number::Six, Number::Seven, Number::Eight, Number::Nine, Number::Ten, Number::Jack, Number::Queen, Number::King].iter() {
        cards.push(Card::new(*suit, *number));
      }
    }
    Deck {
      cards,
    }
  }

  pub fn shuffle(&mut self) {
    use rand::seq::SliceRandom;
    self.cards.shuffle(&mut rand::thread_rng());
  }

  pub fn deal(&mut self) -> Option<Card> {
    self.cards.pop()
  }

  pub fn len(&self) -> usize {
    self.cards.len()
  }
}

pub struct Hand {
  pub cards: Vec<Card>,
}

impl Hand {
  pub fn new() -> Hand {
    Hand {
      cards: Vec::new(),
    }
  }

  pub fn add(&mut self, card: Card) {
    self.cards.push(card);
  }

  pub fn sort(&mut self) {
    self.cards.sort_by(|a, b| {
      if a.suit == b.suit {
        a.number.cmp(&b.number)
      } else {
        a.suit.cmp(&b.suit)
      }
    });
  }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in self.cards.iter() {
          write!(f, "{} ", card)?;
        }
        Ok(())
    }
}