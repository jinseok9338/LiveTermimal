mod deck {
    use deckofcards::*;
    pub fn create_deck() -> Deck {
        let mut deck = Deck::new();
        deck.shuffle();
        return deck;
    }

    pub fn shuffle_deck(deck: &mut Deck) {
        deck.shuffle();
        println!("deck shuffled")
    }

    pub fn reset_deck(deck: &mut Deck) {
        deck.reset_shuffle()
    }
}
