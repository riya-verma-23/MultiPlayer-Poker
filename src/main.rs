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
fn check_full_house(hand: Vec<Card>){
    trip: i32 = 0;
    double: i32 = 0;
    for (Card c : hand){
        if (count_num(hand, c.val) == 3){
            trip = c.val;
        }
        else if (count_num(hand, c.val) == 2){
            double = c.val;
        }
    }
    return trip + double/100;
}
fn check_triples(hand: Vec<Card>){
    trip: i32 = 0;
    leftover: i32 = 0;
    for (Card c : hand){
        if (count_num(hand, c.val) == 3){
            trip = c.val;
        }
        else {
            leftover = c.val;
        }
    }
    return trip + leftover/100;
}
fn check_two_pair(hand: Vec<Card>){
    pair_low: i32 = 0;
    pair_high: i32 = 0;
    for (Card c : hand){
        if (count_num(hand, c.val) == 2){
            if (pair_low == 0){
                pair_low = c.val;
            }
            else{
                pair_high = c.val;
            }
        }
        else {
            leftover = c.val;
        }
    }
    if (pair_high < pair_low){
        temp = pair_high;
        pair_high = pair_low;
        pair_low = temp;
    }
    return pair_high + pair_low/100 + leftover/1000;
}
fn check_pair(hand: Vec<Card>){
    pair: i32 = 0;
    extra: Vec<i32> = Vec::new();
    for (Card c : hand){
        if (count_num(hand, c.val) == 2){
            pair = c.val
        }
        else {
            extra.push(c.val);
        }
    }

    return pair + extra.0/100 + extra.1/1000;
}
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
