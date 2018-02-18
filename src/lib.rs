use std::fmt;

pub type Suit = i8;
pub const NULL_SUIT: Suit = 0;
pub const CLUBS: Suit = 1;
pub const DIAMONDS: Suit = 2;
pub const HEARTS: Suit = 3;
pub const SPADES: Suit = 4;

pub type Rank = i8;
pub const NULL_RANK: Rank = 0;
pub const ACE: Rank = 1;
pub const TWO: Rank = 2;
pub const THREE: Rank = 3;
pub const FOUR: Rank = 4;
pub const FIVE: Rank = 5;
pub const SIX: Rank = 6;
pub const SEVEN: Rank = 7;
pub const EIGHT: Rank = 8;
pub const NINE: Rank = 9;
pub const TEN: Rank = 10;
pub const JACK: Rank = 11;
pub const QUEEN: Rank = 12;
pub const KING: Rank = 13;


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit = match self.suit {
            NULL_SUIT => "null",
            CLUBS => "Clubs",
            DIAMONDS => "Diamonds",
            HEARTS => "Hearts",
            SPADES => "Spades",
            _ => "*unknown*"
        };

        let rank = match self.rank {
            NULL_RANK => "null",
            ACE => "Ace",
            TWO => "2",
            THREE => "3",
            FOUR => "4",
            FIVE =>"5",
            SIX => "6",
            SEVEN => "7",
            EIGHT => "8",
            NINE => "9",
            TEN => "10", 
            JACK => "Jack",
            QUEEN => "Queen",
            KING => "King",
            _ => "*unknown*"            
        };

        write!(f, "{} of {}", rank, suit)
    }
} 

const DECKSIZE: usize = 52;

pub struct Deck {
    pub cards: [Card; DECKSIZE],
}

impl Deck {
    pub fn new() -> Deck {
        let mut i: usize = 0;
        let mut cards = [Card{suit: NULL_SUIT, rank: NULL_RANK}; DECKSIZE];
        for s in CLUBS..SPADES+1 {
            for r in ACE..KING+1 {
                cards[i] = Card{suit: s, rank: r};
                i += 1;
            }
        }
        Deck{cards: cards}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_in_order() {
        // verify that a new deck is in increasing order
        let d = Deck::new();

        let mut prev: Card = Card{suit: NULL_SUIT, rank: NULL_RANK};
        for c in d.cards.iter() {
            assert!(c > &prev);
            prev = *c;
        }

    }

    #[test]
    fn it_works() {
        let c = Card{suit: SPADES, rank: TEN};
        let d = Deck::new();

        println!("card = {:?}", c);
        for c in d.cards.iter() {
            println!("card = {}", c);
        }
        assert_eq!(2 + 2, 4);
    }
}
