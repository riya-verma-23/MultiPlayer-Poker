use std::collections::HashMap;

struct Deck {
    card_deck: HashMap<Suit,i32>,
}

impl Deck {
    pub fn new() -> Deck {
        let deck_map: HashMap<Suit,i32> = HashMap::new();

        for i in 1..13 {
            for j in 1..4 {
                deck_map.insert(IntToSuit(j), i);
            }
        }

        let deck = Deck {
            card_deck = deck_map;
        }
        return deck;
    }

    
}