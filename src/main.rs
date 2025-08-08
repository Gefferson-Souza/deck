use rand::{prelude::ThreadRng, rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits: [&'static str; 4] = ["Clubs", "Diamonds", "Hearts", "Spades"];
        let values: [&'static str; 4] = ["Ace", "Two", "Three", "Four"];
        let mut cards: Vec<String> = vec![];

        for suit in suits {
            for value in values {
                let card: String = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        return Deck { cards };
    }

    fn shuffle(&mut self) {
        let mut rng: ThreadRng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        let mut i:usize =0;
        let mut return_vector: Vec<String> = vec![];
        while  i < num_cards {
            let card:String = self.cards[i].clone();
            self.cards.remove(i);
            return_vector.push(card);
            i += 1;
        }

        return return_vector;
    }
}

fn main() {
    let mut deck: Deck = Deck::new();

    deck.shuffle();

    let cards: Vec<String> = deck.deal(5);
    print!("Cartas descartadas: {:#?}", cards);

    print!("ACABOU O CÓDIGO DECK: {:#?}", deck)
}
