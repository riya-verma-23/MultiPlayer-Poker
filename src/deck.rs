pub mod card;
pub struct Deck {
    card_deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let deck: Vec<Card> = Vec::new();

        for i in 1..13 {
            for j in 1..4 {
                let c = Card {
                    val : i,
                    suit : card::IntToSuit(j),
                };
                deck.pushback(c);
            }
        }
        return deck;
    }


}