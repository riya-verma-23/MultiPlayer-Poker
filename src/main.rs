// pub mod deck;
pub mod card;
use crate::card::Card;
pub mod deck;
use crate::deck::Deck;

pub fn scoreHand(hand : Vec<Card>) -> u32 {
    return 100;
}

fn count_num(hand: Vec<Card>, find: i32){
    count: i32 = 0;
    for (Card c : hand){
        if (c.val == find){
            count += 1;
        }
    }
    return count;
}

fn check_all_same_suit(hand: Vec<Card>) -> u32 {
    for i in 0..5 {
        if hand[0].val != hand[i].val {
            return false;
        }
    }
    return true;
}

fn check_royal_flush(hand: Vec<Card>) -> bool {
    let num: Vec<u32> = Vec::new();
    let royal: Vec<u32> = [14, 13, 12, 11, 10];
    //initialize num with all numbers from hand
    for i in 0..5 {
        num.push(hand.val);
    }
    //check if it contains all royal flush values
    for i in 0..5 {
        if !num.contains(royal[i]) {
            return false;
        }
    }
    return true;
}

fn check_quads(hand: Vec<Card>){
    quad: i32 = 0;
    single: i32 = 0;
    for (Card c : hand){
        if (count_num(hand, c.val) == 4){
            quad = c.val;
        }
        else{
            single = c.val;
        }
    }
    return quad + single/100;
}

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
    let hand: Vec<Card> = mydeck.GenerateHand();
    print!("{}\n", scoreHand(hand));
    
}
