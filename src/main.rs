#[derive(Debug)]
struct Deck {
    cards: Vec<String>, 
}

impl Deck {
    fn new () -> Self {
        let suits: [&'static str;4]   = ["Clubs", "Diamonds", "Hearts", "Spades"];
        let values:[&'static str;4] = ["Ace", "Two", "Three", "Four"];
        let mut cards:Vec<String> = vec![];

        for suit in suits {
            for value in values {
                let card: String = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn printDeck(&self){
        println!("Heres your deck: {:#?}", self);
    }
}

fn main() {
    let deck: Deck = Deck::new();

    deck.printDeck();
}
