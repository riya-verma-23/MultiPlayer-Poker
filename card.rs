
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
    pub fn newCard(s: Suit, v: i32) ->Card {
        // if v > 13 || v <= 0 
        let c = Card {
            val = v;
            suit = s;
        }
        return c;
    }

    pub fn randomCard() -> Card {
        
    }
}