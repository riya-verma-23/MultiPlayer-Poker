// pub mod deck;
pub mod card;
use crate::card::Card;
pub mod deck;
use crate::deck::Deck;

pub fn scoreHand(hand: Vec<Card>) -> f64 {
    let mut score = 0.0;
    score = max(score, check_highcard(&hand));
    score = max(score, check_pair(&hand));
    score = max(score, check_two_pair(&hand));
    score = max(score, check_triples(&hand));
    score = max(score, check_straight(&hand));
    score = max(score, check_full_house(&hand));
    return score;
}
fn count_num(hand: &Vec<Card>, find: i32) -> i32 {
    let mut count: i32 = 0;
    for c in hand {
        if (c.val == find) {
            count += 1;
        }
    }
    return count;
}

fn check_quads(hand: &Vec<Card>) -> f64 {
    let mut quad: i32 = 0;
    let mut single: i32 = 0;
    for c in hand {
        if count_num(&hand, c.val) == 4 {
            quad = c.val;
        } else {
            single = c.val;
        }
    }
    return 700.0 + quad as f64 + (single as f64) / 100.0;
}
fn check_all_same_suit(hand: &Vec<Card>) -> bool {
    for i in 0..5 {
        if hand[0].val != hand[i].val {
            return false;
        }
    }
    return true;
}

fn check_royal_flush(hand: &Vec<Card>) -> f64 {
    let mut num: Vec<i32> = Vec::new();
    let royal: Vec<i32> = [14, 13, 12, 11, 10].to_vec();
    //initialize num with all numbers from hand
    for i in 0..5 {
        num.push(hand[i].val);
    }
    //check if it contains all royal flush values
    for i in 0..5 {
        if !num.contains(&royal[i]) {
            return 0.0;
        }
    }
    return 1000.0;
}
fn check_full_house(hand: &Vec<Card>) -> f64 {
    let mut trip: i32 = 0;
    let mut double: i32 = 0;
    for c in hand {
        if (count_num(&hand, c.val) == 3) {
            trip = c.val;
        } else if (count_num(&hand, c.val) == 2) {
            double = c.val;
        }
    }
    if (trip == 0 || double == 0) {
        return 0.0;
    }
    return 600.0 + trip as f64 + (double as f64) / 100.0;
}
fn check_triples(hand: &Vec<Card>) -> f64 {
    let mut trip: i32 = 0;
    let mut leftover: i32 = 0;
    for c in hand {
        if (count_num(&hand, c.val) == 3) {
            trip = c.val;
        } else {
            leftover = c.val;
        }
    }
    if (trip == 0 || leftover == 0) {
        return 0.0;
    }
    return 300.0 + trip as f64 + (leftover as f64) / 100.0;
}
fn check_two_pair(hand: &Vec<Card>) -> f64 {
    let mut pair_low: i32 = 0;
    let mut pair_high: i32 = 0;
    let mut leftover: i32 = 0;

    for c in hand {
        if (count_num(&hand, c.val) == 2) {
            if (pair_low == 0) {
                pair_low = c.val;
            } else if (c.val != pair_low) {
                pair_high = c.val;
            }
        } else {
            leftover = c.val;
        }
    }
    if (pair_high < pair_low) {
        let mut temp = pair_high;
        pair_high = pair_low;
        pair_low = temp;
    }
    if (pair_high == 0 || pair_low == 0) {
        return 0.0;
    }
    return 200.0 + pair_high as f64 + (pair_low as f64) / 100.0 + (leftover as f64) / 1000.0;
}
fn check_pair(hand: &Vec<Card>) -> f64 {
    let mut pair: i32 = 0;
    let mut extra: Vec<i32> = Vec::new();
    for c in hand {
        if (count_num(&hand, c.val) == 2) {
            pair = c.val
        } else {
            extra.push(c.val);
        }
    }
    if (pair == 0) {
        return 0.0;
    }
    return 100.0 + pair as f64 + (extra[0] as f64) / 100.0 + (extra[1] as f64) / 1000.0;
}
fn check_straight(hand: &Vec<Card>) -> f64 {
    let mut extra: Vec<i32> = Vec::new();
    for c in hand {
        extra.push(c.val);
    }
    extra.sort();
    for i in 1..5 {
        if (extra[i] != extra[i - 1] + 1) {
            return 0.0;
        }
    }
    return 400.0 + extra[4] as f64;
}
fn check_highcard(hand: &Vec<Card>) -> f64 {
    let mut extra: Vec<i32> = Vec::new();
    for c in hand {
        extra.push(c.val);
    }
    extra.sort();
    return extra[0] as f64 + (extra[1] as f64) / 100.0 + (extra[2] as f64) / 10000.0;
}
fn max(first: f64, second: f64) -> f64 {
    if first > second {
        return first;
    }
    return second;
}
fn main() {
    println!("Welcome to Poker");
    let mut mydeck: Deck = Deck::new();
    mydeck.shuffle();
    let hand: Vec<Card> = mydeck.GenerateHand();
    print!("{}\n", count_num(&hand.clone(), 4));
    print!("{}\n", scoreHand(hand.clone()));
    let score = scoreHand(hand.clone());
    let hand_text = match score {
        x if x < 100.0 => "High Card",
        x if x < 200.0 => "Pair",
        x if x < 300.0 => "Two Pair",
        x if x < 400.0 => "Three of a Kind",
        x if x < 500.0 => "Straight",
        x if x < 600.0 => "Flush",
        x if x < 700.0 => "Full House",
        x if x < 800.0 => "Four of a Kind",
        x if x < 900.0 => "Straight Flush",
        x if x < 1000.0 => "Royal Flush",
        _ => "Error",
    };
    print!("{}\n", hand_text);
}
