use rand::prelude::*;
pub enum Suit{
    Heart,
    Club,
    Diamond,
    Spade,
}

pub struct Card {
    val : i32,
    suit : Suit,    
}

impl Card {
    pub fn new(s: Suit, v: i32) ->Card {
        let c = Card {
            val : v,
            suit : s,
        };
        return c;
    }

    pub fn randomCard() -> Card {
        let mut rng = rand::thread_rng();
        let v = rng.gen_range(1..13);
        let s : Suit = IntToSuit(rng.gen_range(1..4)).unwrap();
        let c = Card {
            val : v,
            suit : s,
        };
        return c;
    }

    pub fn PrintCard(&self) {
        match self.suit {
            Suit::Heart => print!("Heart "),
            Suit::Club => print!("Club "),
            Suit::Diamond => print!("Diamond "),
            Suit::Spade =>print!("Spade "),
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