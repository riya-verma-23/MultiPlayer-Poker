use rand::prelude::*;

#[derive(Clone)]
pub enum Suit{
    Heart,
    Club,
    Diamond,
    Spade,
}

#[derive(Clone)]
#[derive(Default)]
pub struct Card {
    val : i32,
    suit : i32,    
}

impl Card {
    pub fn new(s: i32, v: i32) ->Card {
        let c = Card {
            val : v,
            suit : s,
        };
        return c;
    }

    pub fn randomCard() -> Card {
        let mut rng = rand::thread_rng();
        let v = rng.gen_range(1..13);
        let s : i32 = rng.gen_range(1..4);
        let c = Card {
            val : v,
            suit : s,
        };
        return c;
    }

    pub fn PrintCard(&self) {
        match self.suit {
            1 => print!("Heart "),
            2 => print!("Club "),
            3 => print!("Diamond "),
            4 =>print!("Spade "),
            _ => print!("invalid suit"),
        }
        print!("{}\n", self.val);
    }
}

pub fn IntToSuit(s : i32) -> Result<Suit,()> {
    match s {
        1 => Ok(Suit::Heart),
        2 => Ok(Suit::Club),
        3 => Ok(Suit::Diamond),
        4 => Ok(Suit::Spade),
        _ => Err(())
    }
}

pub fn SuitToInt(s : Suit) -> i32 {
    match s {
        Suit::Heart => 1,
        Suit::Club => 2,
        Suit::Diamond => 3,
        Suit::Spade => 4,
    }
}