use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // List of suits
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        // List of Values
        let values = [
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King",
        ];

        // Double nested for loop
        let mut cards: Vec<String> = Vec::new();

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        let deck = self.cards.split_off(num_cards);
        let remainder = self.cards.clone();
        self.cards = deck;
        remainder
    }
}



fn deal() {}

fn main() {
    let mut deck: Deck = Deck::new();
    deck.shuffle();

    let cards: Vec<String> = deck.deal(10);
    println!("Heres your cards: {:#?}", cards);
    println!("Heres your deck: {:#?}", deck);
}
