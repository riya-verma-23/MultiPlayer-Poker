// pub mod deck;
pub mod card;
use crate::card::Card;
pub mod deck;
use crate::deck::Deck;

pub fn scoreHand(hand : Vec<Card>) -> u32 {
    return 100;
}
fn count_num(hand: Vec<Card>, find: i32) ->i32 {
    let mut count: i32 = 0;
    for c in hand.iter() {
        if (c.val == find){
            count += 1;
        }
    }
    return count;
}

fn check_all_same_suit(hand: Vec<Card>) -> bool {
    for i in 0..5 {
        if hand[0].val != hand[i].val {
            return false;
        }
    }
    return true;
}

fn check_royal_flush(hand: Vec<Card>) -> bool {
    let mut num: Vec<i32> = Vec::new();
    let royal: Vec<i32> = [14, 13, 12, 11, 10].to_vec();
    //initialize num with all numbers from hand
    for i in 0..5 {
        num.push(hand[i].val);
    }
    //check if it contains all royal flush values
    for i in 0..5 {
        if !num.contains(&royal[i]) {
            return false;
        }
    }
    return true;
}

// fn check_quads(hand: Vec<Card>){
//     quad: i32 = 0;
//     single: i32 = 0;
//     for (Card c : hand){
//         if (count_num(hand, c.val) == 4){
//             quad = c.val;
//         }
//         else{
//             single = c.val;
//         }
//     }
//     return quad + single/100;
// }

fn main () {
    println!("Welcome to Poker");
    // let mycard : Card = Card::randomCard();
    // mycard.PrintCard();
    let mut mydeck : Deck = Deck::new();
    mydeck.PrintDeck();
    print!("\n");
    mydeck.PrintDeck();
    print!("\n\n\n hand\n");
    let hand: Vec<Card> = mydeck.GenerateHand();
    print!("{}\n", count_num(hand.clone(), 4));
    print!("{}\n", scoreHand(hand.clone()));
    
}
