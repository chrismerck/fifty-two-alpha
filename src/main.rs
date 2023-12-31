/*
 * Experimenting with Rummy Strategies.
 */

mod french;
use crate::french::{Card, Suit, Number, Deck, Hand};
use rand::Rng;

enum DrawAction {
    DrawFromDeck,
    DrawFromDiscard,
    // TODO: pick up the pack
}

struct Meld {
    cards: Vec<Card>,
}

impl Meld {
    fn new(mut cards: Vec<Card>) -> Meld {
        if !Meld::is_valid(&mut cards) {
            panic!("Invalid meld");
        }
        Meld {
            cards,
        }
    }

    pub fn is_valid(cards: &mut Vec<Card>) -> bool {
        cards.sort_by(|a, b| a.number.cmp(&b.number));
        if cards.len() < 3 {
            return false;
        }
        let is_book = cards.iter().all(|c| c.number == cards[0].number);
        let is_flush = cards.iter().all(|c| c.suit == cards[0].suit);
        let is_straight_aces_low = cards.iter().skip(1).enumerate().all(|(i, c)| {
            c.number as usize == cards[i].number as usize + 1
        });
        let is_straight_aces_high = cards.iter().skip(1).enumerate().all(|(i, c)| {
            if cards[i].number == Number::King {
                c.number == Number::Ace
            } else {
                c.number as usize == cards[i].number as usize + 1
            }
        });
        is_book || is_flush && (is_straight_aces_low || is_straight_aces_high)
    }
}

impl std::fmt::Display for Meld {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.cards.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "))
    }
}

/// Strategy for drawing, making melds, and discarding.
type Strategy = (
    fn(&Hand) -> DrawAction, 
    fn(&mut Hand) -> (
        Vec<Meld>,
        Card
    ));

struct Game {
    deck: Deck,
    pack: Vec<Card>,
    hands: Vec<Hand>,
    melds: Vec<Vec<Meld>>,
    strategies: Vec<Strategy>,
    /// Player index whose turn it is.
    turn: usize,
}

impl Game {
    fn new(strategies: Vec<Strategy>) -> Game {
        let mut deck = Deck::new();
        deck.shuffle();
        // deal 7 cards to each player
        let mut hands = Vec::new();
        for _ in 0..strategies.len() {
            let mut hand = Hand::new();
            for _ in 0..7 {
                hand.add(deck.deal().unwrap());
            }
            hands.push(hand);
        }
        // deal 1 card to the discard pile
        let mut pack = Vec::new();
        pack.push(deck.deal().unwrap());
        // empty meld vector for each player
        let mut melds = Vec::new();
        for _ in 0..strategies.len() {
            melds.push(Vec::new());
        }
        Game {
            deck,
            pack,
            hands,
            melds,
            strategies,
            turn: 0,
        }
    }

    fn round(&mut self) -> bool {
        self.hands[self.turn].sort();
        println!("Player {}'s turn", self.turn + 1);
        println!("  Deck Size: {}", self.deck.len());
        println!("  Pack: {}", self.pack.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "));
        println!("  Your Hand: {}", self.hands[self.turn]);
        if self.melds[self.turn].len() > 0 {
            println!("  Your Melds: {}", self.melds[self.turn].iter().map(|m| m.to_string()).collect::<Vec<String>>().join(" "));
        } else {
            println!("  You have no melds.");
        }
        println!("  ---------");
        let (draw, play) = self.strategies[self.turn];
        let draw_action = draw(&self.hands[self.turn]);
        let card = match draw_action {
            DrawAction::DrawFromDeck => self.deck.deal().unwrap(),
            DrawAction::DrawFromDiscard => self.pack.pop().unwrap(),
        };
        match draw_action {
            DrawAction::DrawFromDeck => println!("  You draw a {} from the deck.", card),
            DrawAction::DrawFromDiscard => println!("  You pick up the {} from the discard pile.", card),
        };
        self.hands[self.turn].add(card);
        let (melds, discard) = play(&mut self.hands[self.turn]);
        for meld in melds {
            println!("  You play a meld: {}", meld);
            self.melds[self.turn].push(meld);
        }
        println!("  You discard the {}.", discard);
        println!("");
        self.pack.push(discard);
        if self.deck.len() == 0 {
            return false;
        }
        self.turn = (self.turn + 1) % self.strategies.len();
        true
    }

    fn play(&mut self) -> Vec<usize> {
        while self.round() {}
        self.score()
    }

    fn score(&self) -> Vec<usize> {
        let mut scores = Vec::new();
        for hand in &self.hands {
            let mut score = 0;
            for card in &hand.cards {
                score += card.value(false);
            }
            scores.push(score);
        }
        scores
    }
}

fn find_meld(hand: &mut Hand) -> Option<Meld> {
    // assume hand is sorted
    // search for straights
    let mut straight_start = 0;
    let mut straight_end = 0;
    for i in 0..hand.cards.len() {
        if i == 0 { continue; }
        if hand.cards[i].suit != hand.cards[i - 1].suit {
            if straight_end - straight_start >= 2 {
                let mut cards = Vec::new();
                for j in straight_start..=straight_end {
                    cards.push(hand.cards.remove(straight_start));
                }
                return Some(Meld::new(cards));
            }
            straight_start = i;
            straight_end = i;
            continue;
        }
        if hand.cards[i].number as usize == hand.cards[i - 1].number as usize + 1 {
            straight_end = i;
        } else {
            if straight_end - straight_start >= 2 {
                let mut cards = Vec::new();
                for j in straight_start..=straight_end {
                    cards.push(hand.cards.remove(straight_start));
                }
                return Some(Meld::new(cards));
            }
            straight_start = i;
            straight_end = i;
            continue;
        }
    }
    None
}

fn my_draw_strategy(hand: &Hand) -> DrawAction {
    DrawAction::DrawFromDeck
}

fn my_play_strategy(hand: &mut Hand) -> (Vec<Meld>, Card) {
    (Vec::new(), hand.cards.remove(0))
}

const MY_STRATEGY: Strategy = (my_draw_strategy, my_play_strategy);

fn main() {
    let mut game = Game::new(vec![MY_STRATEGY, MY_STRATEGY]);
    let hand = &game.hands[0];
    println!("{:?}", game.play());
}
