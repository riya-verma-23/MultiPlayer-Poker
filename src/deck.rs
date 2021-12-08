pub use crate::card::IntToSuit;
pub use crate::card::Card;
use rand::{thread_rng, Rng};
extern crate rand;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;

pub struct Deck {
    card_deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck: Vec<Card> = Vec::new();

        for i in 1..14 {
            for j in 1..5 {
                let c : Card = Card::new(j, i);
                deck.push(c);
            }
        }
        let d : Deck = Deck {
            card_deck : deck, 
        };
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
        let mut rng = StepRng::new(2, 13);
        let mut irs = Irs::default();
        irs.shuffle(&mut self.card_deck, &mut rng);
    }

    pub fn deal(&mut self) -> Result<Card,()> {
        // if self.card_deck.len() != 0 { 
        let card = self.card_deck.pop().unwrap();
        return Ok(card);
        // else {
        //     Err("The deck is empty");
        // }
}
}