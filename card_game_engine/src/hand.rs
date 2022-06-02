mod hand {
    use deckofcards::{Card, Deck, Hand};

    pub fn create_hand_from_deck(deck: &mut Deck, numcards: usize) -> Vec<Card> {
        let mut hand = deck.deal(numcards);

        return hand;
    }

    pub fn display_cards(hand: Vec<Card>) {
        println!("Your hand is {:?}", hand)
    }
}
