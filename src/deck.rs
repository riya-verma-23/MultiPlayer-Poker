pub use crate::card::IntToSuit;
pub use crate::card::Card;

pub struct Deck {
    card_deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck: Vec<Card> = Vec::new();

        for i in 1..14 {
            for j in 1..5 {
                let c : Card = Card::new(IntToSuit(j).unwrap(), i);
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
