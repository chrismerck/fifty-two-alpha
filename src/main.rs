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

/// Strategy for drawing and discarding.
type Strategy = (fn(&Game, &Hand) -> DrawAction, fn(&Game, &Hand) -> usize);

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
        println!("  ---------");
        let (draw, discard) = self.strategies[self.turn];
        let draw_action = draw(self, &self.hands[self.turn]);
        let card = match draw_action {
            DrawAction::DrawFromDeck => self.deck.deal().unwrap(),
            DrawAction::DrawFromDiscard => self.pack.pop().unwrap(),
        };
        match draw_action {
            DrawAction::DrawFromDeck => println!("  You draw a {} from the deck.", card),
            DrawAction::DrawFromDiscard => println!("  You pick up the {} from the discard pile.", card),
        };
        self.hands[self.turn].add(card);
        let discard_index = discard(self, &self.hands[self.turn]);
        let card = self.hands[self.turn].cards.remove(discard_index);
        println!("  You discard the {}.", card);
        println!("");
        self.pack.push(card);
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

fn my_draw_strategy(game: &Game, hand: &Hand) -> DrawAction {
    DrawAction::DrawFromDeck
}

fn my_discard_strategy(game: &Game, hand: &Hand) -> usize {
    0
}

const MY_STRATEGY: Strategy = (my_draw_strategy, my_discard_strategy);

fn main() {
    let mut game = Game::new(vec![MY_STRATEGY, MY_STRATEGY]);
    let hand = &game.hands[0];
    println!("{:?}", game.play());

    println!("====================");

    // let's test a few Melds to make sure they work
    let mut cards : Vec<Card> = 
        vec!["2C", "3C", "4C", "5C"].iter().map(|s| Card::from_string(s)).collect();
    let meld = Meld::new(cards);
    println!("{} is a valid meld", meld.cards.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "));

    // let's list a few sets of cards and check if they are valid melds
    let mut sets : Vec<Vec<Card>> = vec![
        vec!["2C", "4C", "3C"],
        vec!["AS", "3S", "2S"],
        vec!["JD", "QD", "AD", "KD"],
        vec!["6S", "4C", "5C"],
        vec!["6S", "6C", "6H"],
    ].iter().map(|v| v.iter().map(|s| Card::from_string(s)).collect()).collect();

    for mut cards in sets {
        // check if valid
        println!("{} is {}",
            cards.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "),
            if Meld::is_valid(&mut cards) { "valid" } else { "invalid" }
        );
    }

}
