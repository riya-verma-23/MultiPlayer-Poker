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
    print!("\n\n\n hand\n");
    mydeck.GenerateHand();
}

// pub fn scoreHand(hand : Vec<Card>, ) {
//     let card_values : [i32;5];

//     for i in &card_values {
        
//     }
// }
