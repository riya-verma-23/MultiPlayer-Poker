pub use crate::card::Card;
pub use crate::card::IntToSuit;
use rand::thread_rng;
extern crate rand;

use rand::seq::SliceRandom;
pub struct Deck {
    card_deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck: Vec<Card> = Vec::new();

        for i in 1..14 {
            for j in 1..5 {
                let c: Card = Card::new(j, i);
                deck.push(c);
            }
        }
        let d: Deck = Deck { card_deck: deck };
        return d;
    }

    pub fn PrintDeck(&self) {
        for i in 0..self.card_deck.len() {
            self.card_deck[i].PrintCard();
        }
    }

    pub fn shuffle(&mut self) {
        // let slice = self.as_mut_slice();
        // thread_rng().shuffle(slice);
        self.card_deck.shuffle(&mut thread_rng());
    }

    pub fn GenerateHand(&self) -> Vec<Card> {
        let mut hand: Vec<Card> = Vec::new();
        for i in 0..5 {
            self.card_deck[i].PrintCard();
            hand.push(self.card_deck[i].clone());
        }
        return hand;
    }
}
