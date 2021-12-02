pub mod card;
use crate::card::IntToSuit;
use crate::card::Card;

pub struct Deck {
    card_deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck: Vec<Card> = Vec::new();

        for i in 1..13 {
            for j in 1..4 {
                let c = Card {
                    val : i,
                    suit : IntToSuit(j).unwrap(),
                };
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


}