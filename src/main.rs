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

    /// play a single turn
    fn play(&mut self) {
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
        self.turn = (self.turn + 1) % self.strategies.len();
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
    game.play();
    game.play();
    game.play();
    game.play();
}
