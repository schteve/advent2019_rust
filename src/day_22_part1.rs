/*
    --- Day 22: Slam Shuffle ---
    There isn't much to do while you wait for the droids to repair your ship. At least you're drifting in the right direction. You decide to practice a new card shuffle you've been working on.

    Digging through the ship's storage, you find a deck of space cards! Just like any deck of space cards, there are 10007 cards in the deck numbered 0 through 10006. The deck must be new - they're still in factory order, with 0 on the top, then 1, then 2, and so on, all the way through to 10006 on the bottom.

    You've been practicing three different techniques that you use while shuffling. Suppose you have a deck of only 10 cards (numbered 0 through 9):

    To deal into new stack, create a new stack of cards by dealing the top card of the deck onto the top of the new stack repeatedly until you run out of cards:

    Top          Bottom
    0 1 2 3 4 5 6 7 8 9   Your deck
                        New stack

    1 2 3 4 5 6 7 8 9   Your deck
                    0   New stack

        2 3 4 5 6 7 8 9   Your deck
                    1 0   New stack

        3 4 5 6 7 8 9   Your deck
                2 1 0   New stack

    Several steps later...

                    9   Your deck
    8 7 6 5 4 3 2 1 0   New stack

                        Your deck
    9 8 7 6 5 4 3 2 1 0   New stack
    Finally, pick up the new stack you've just created and use it as the deck for the next technique.

    To cut N cards, take the top N cards off the top of the deck and move them as a single unit to the bottom of the deck, retaining their order. For example, to cut 3:

    Top          Bottom
    0 1 2 3 4 5 6 7 8 9   Your deck

        3 4 5 6 7 8 9   Your deck
    0 1 2                 Cut cards

    3 4 5 6 7 8 9         Your deck
                0 1 2   Cut cards

    3 4 5 6 7 8 9 0 1 2   Your deck
    You've also been getting pretty good at a version of this technique where N is negative! In that case, cut (the absolute value of) N cards from the bottom of the deck onto the top. For example, to cut -4:

    Top          Bottom
    0 1 2 3 4 5 6 7 8 9   Your deck

    0 1 2 3 4 5           Your deck
                6 7 8 9   Cut cards

            0 1 2 3 4 5   Your deck
    6 7 8 9               Cut cards

    6 7 8 9 0 1 2 3 4 5   Your deck
    To deal with increment N, start by clearing enough space on your table to lay out all of the cards individually in a long line. Deal the top card into the leftmost position. Then, move N positions to the right and deal the next card there. If you would move into a position past the end of the space on your table, wrap around and keep counting from the leftmost card again. Continue this process until you run out of cards.

    For example, to deal with increment 3:


    0 1 2 3 4 5 6 7 8 9   Your deck
    . . . . . . . . . .   Space on table
    ^                     Current position

    Deal the top card to the current position:

    1 2 3 4 5 6 7 8 9   Your deck
    0 . . . . . . . . .   Space on table
    ^                     Current position

    Move the current position right 3:

    1 2 3 4 5 6 7 8 9   Your deck
    0 . . . . . . . . .   Space on table
        ^               Current position

    Deal the top card:

        2 3 4 5 6 7 8 9   Your deck
    0 . . 1 . . . . . .   Space on table
        ^               Current position

    Move right 3 and deal:

        3 4 5 6 7 8 9   Your deck
    0 . . 1 . . 2 . . .   Space on table
                ^         Current position

    Move right 3 and deal:

            4 5 6 7 8 9   Your deck
    0 . . 1 . . 2 . . 3   Space on table
                    ^   Current position

    Move right 3, wrapping around, and deal:

            5 6 7 8 9   Your deck
    0 . 4 1 . . 2 . . 3   Space on table
        ^                 Current position

    And so on:

    0 7 4 1 8 5 2 9 6 3   Space on table
    Positions on the table which already contain cards are still counted; they're not skipped. Of course, this technique is carefully designed so it will never put two cards in the same position or leave a position empty.

    Finally, collect the cards on the table so that the leftmost card ends up at the top of your deck, the card to its right ends up just below the top card, and so on, until the rightmost card ends up at the bottom of the deck.

    The complete shuffle process (your puzzle input) consists of applying many of these techniques. Here are some examples that combine techniques; they all start with a factory order deck of 10 cards:

    deal with increment 7
    deal into new stack
    deal into new stack
    Result: 0 3 6 9 2 5 8 1 4 7
    cut 6
    deal with increment 7
    deal into new stack
    Result: 3 0 7 4 1 8 5 2 9 6
    deal with increment 7
    deal with increment 9
    cut -2
    Result: 6 3 0 7 4 1 8 5 2 9
    deal into new stack
    cut -2
    deal with increment 7
    cut 8
    cut -4
    deal with increment 7
    cut 3
    deal with increment 9
    deal with increment 3
    cut -1
    Result: 9 2 5 8 1 4 7 0 3 6
    Positions within the deck count from 0 at the top, then 1 for the card immediately below the top card, and so on to the bottom. (That is, cards start in the position matching their number.)

    After shuffling your factory order deck of 10007 cards, what is the position of card 2019?
*/

fn modulo(n: i64, modulus: i64) -> i64 {
    let mut m = n;

    let div = n / modulus; // Don't use the % operator because it's messy and this is almost as quick
    m -= div * modulus;

    while m < 0 {
        m += modulus;
    }
    while m >= modulus {
        m -= modulus;
    }
    m
}

#[derive(Clone, Copy)]
enum Technique {
    DealNewStack,
    DealWithIncrement(i64),
    Cut(i64),
}

impl Technique {
    fn from_string(s: &str) -> Self {
        if s == "deal into new stack" {
            return Technique::DealNewStack;
        } else if s.starts_with("deal with increment ") == true {
            for sub in s.split("deal with increment ") {
                if sub != "" {
                    let value = sub.parse::<i64>().unwrap();
                    return Technique::DealWithIncrement(value);
                }
            }
        } else if s.starts_with("cut ") == true {
            for sub in s.split("cut ") {
                if sub != "" {
                    let value = sub.parse::<i64>().unwrap();
                    return Technique::Cut(value);
                }
            }
        }

        panic!("Unknown technique: {}", s);
    }

    fn to_string(&self) -> String {
        match self {
            Technique::DealNewStack => {
                "Deal into new stack".to_owned()
            },
            Technique::DealWithIncrement(n) => {
                "Deal with increment ".to_owned() + &n.to_string()
            },
            Technique::Cut(n) => {
                "Cut ".to_owned() + &n.to_string()
            },
        }
    }
}

struct Deck {
    cards: Vec<usize>,
}

impl Deck {
    fn new(size: usize) -> Self {
        Self {
            cards: (0..size).collect(),
        }
    }

    fn shuffle(&mut self, technique: Technique) {
        match technique {
            Technique::DealNewStack => {
                self.cards.reverse();
            },
            Technique::DealWithIncrement(n) => {
                let mut idx: i64 = 0;
                let mut new_cards = vec![0; self.cards.len()];
                for &card in &self.cards {
                    new_cards[idx as usize] = card;

                    // Get the next index. Fix it up (modulo).
                    idx = modulo(idx + n, self.cards.len() as i64);
                }

                self.cards = new_cards;
            },
            Technique::Cut(n) => {
                let n = modulo(n, self.cards.len() as i64);
                if n >= 0 {
                    self.cards.rotate_left(n as usize);
                } else {
                    self.cards.rotate_right(-n as usize);
                }
            },
        }
    }

    fn shuffle_many(&mut self, techniques: &Vec<Technique>) {
        techniques.iter().for_each(|&t| self.shuffle(t))
    }

    fn find_card(&self, position: usize) -> Option<usize> {
        self.cards.iter().position(|&card| card == position)
    }
}

#[aoc(day22, part1)]
pub fn solve(input: &str) -> usize {
    let techniques = input.lines()
                          .map(|line| Technique::from_string(line.trim()))
                          .collect::<Vec<Technique>>();
    let mut deck = Deck::new(10007);
    deck.shuffle_many(&techniques);

    let position = deck.find_card(2019).expect("Could not find card 2019!");
    println!("Card 2019 position: {}", position);
    position
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deal_new_stack() {
        let mut deck = Deck::new(10);
        deck.shuffle(Technique::DealNewStack);
        assert_eq!(deck.cards, [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_deal_with_increment() {
        let mut deck = Deck::new(10);
        deck.shuffle(Technique::DealWithIncrement(3));
        assert_eq!(deck.cards, [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);

        // No example for negative increment, unclear if that's needed
    }

    #[test]
    fn test_deal_cut() {
        let mut deck = Deck::new(10);
        deck.shuffle(Technique::Cut(3));
        assert_eq!(deck.cards, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

        let mut deck = Deck::new(10);
        deck.shuffle(Technique::Cut(-4));
        assert_eq!(deck.cards, [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_shuffle_many() {
        let mut deck = Deck::new(10);
        let techniques = vec![Technique::DealWithIncrement(7),
                              Technique::DealNewStack,
                              Technique::DealNewStack];
        deck.shuffle_many(&techniques);
        assert_eq!(deck.cards, [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);

        let mut deck = Deck::new(10);
        let techniques = vec![Technique::Cut(6),
                              Technique::DealWithIncrement(7),
                              Technique::DealNewStack];
        deck.shuffle_many(&techniques);
        assert_eq!(deck.cards, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);

        let mut deck = Deck::new(10);
        let techniques = vec![Technique::DealWithIncrement(7),
                              Technique::DealWithIncrement(9),
                              Technique::Cut(-2)];
        deck.shuffle_many(&techniques);
        assert_eq!(deck.cards, [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);

        let mut deck = Deck::new(10);
        let techniques = vec![Technique::DealNewStack,
                              Technique::Cut(-2),
                              Technique::DealWithIncrement(7),
                              Technique::Cut(8),
                              Technique::Cut(-4),
                              Technique::DealWithIncrement(7),
                              Technique::Cut(3),
                              Technique::DealWithIncrement(9),
                              Technique::DealWithIncrement(3),
                              Technique::Cut(-1)];
        deck.shuffle_many(&techniques);
        assert_eq!(deck.cards, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
