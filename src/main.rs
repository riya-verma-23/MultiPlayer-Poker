// pub mod deck;
pub mod card;
use crate::card::Card;
pub mod deck;
use crate::deck::Deck;


fn main () {
    println!("Welcome to Poker");
    // let mycard : Card = Card::randomCard();
    // mycard.PrintCard();
    let mut mydeck : Deck = Deck::new();
    mydeck.PrintDeck();
    mydeck.shuffle();
    print!("\n");
    mydeck.PrintDeck();
}