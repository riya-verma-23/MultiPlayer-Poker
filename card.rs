use rand::Rng;

enum Suit{
    Heart,
    Club,
    Diamond,
    Spade,
}

struct Card {
    val : i32
    suit : suit    
}

impl Card {
    pub fn new(s: Suit, v: i32) ->Card {
        let c = Card {
            val = v;
            suit = s;
        }
        return c;
    }

    pub fn randomCard() -> Card {

        let mut r = rand::thread_rng();
        let v = rng.gen_range(1..13);
        let s : Suit = IntToSuit(rng.gen_range(1..4));
        let c = Card {
            val = v;
            suit = s;
        }
        return c;
    }

    pub fn IntToSuit(suit : i32) -> Suit {
        match suit {
            1 => Suit::Heart,
            2 => Suit::Club,
            3 => Suit::Diamond,
            4 => Suit::Spade,
        }

    }
}