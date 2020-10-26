/*
    --- Part Two ---
    After a while, you realize your shuffling skill won't improve much more with merely a single deck of cards. You ask every 3D printer on the ship to make you some more cards while you check on the ship repairs. While reviewing the work the droids have finished so far, you think you see Halley's Comet fly past!

    When you get back, you discover that the 3D printers have combined their power to create for you a single, giant, brand new, factory order deck of 119315717514047 space cards.

    Finally, a deck of cards worthy of shuffling!

    You decide to apply your complete shuffle process (your puzzle input) to the deck 101741582076661 times in a row.

    You'll need to be careful, though - one wrong move with this many cards and you might overflow your entire ship!

    After shuffling your new, giant, factory order deck that many times, what number is on the card that ends up in position 2020?
*/

use std::fmt;

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

fn modulo_i128(n: i128, modulus: i128) -> i128 {
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

fn modulo_mult(x: i64, y: i64, modulus: i64) -> i64 {
    let product = (x as i128) * (y as i128);
    let result = modulo_i128(product, modulus as i128);
    result as i64
}

fn modulo_div(x: i64, n: i64, modulus: i64) -> i64 {
    // x * y = n mod m
    // Given x and n, find the inverse of x then multiply by n to get y.
    // E.g. 7 * 5 = 35 = 16 mod 19, given x = 7 and n = 16 find inv_x = -8 then y = -8 * 16 mod 19
    let inv_x = gcd_extended(x, modulus);
    modulo_mult(inv_x, n, modulus)
}

fn gcd_extended(a: i64, b: i64) -> i64 {
    let mut s = 0;
    let mut old_s = 1;
    let mut t = 1;
    let mut old_t = 0;
    let mut r = b;
    let mut old_r = a;

    while r != 0 {
        let quotient = old_r / r;

        let tmp_r = r;
        r = old_r - quotient * r;
        old_r = tmp_r;

        let tmp_s = s;
        s = old_s - quotient * s;
        old_s = tmp_s;

        let tmp_t = t;
        t = old_t - quotient * t;
        old_t = tmp_t;
    }

    /*println!("Bézout coefficients: {}, {}", old_s, old_t);
    println!("greatest common divisor: {}", old_r);
    println!("quotients by the gcd: {}, {}", t, s);*/

    old_s
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

    fn combine_or_swap(x: Self, y: Self, modulus: i64) -> Vec<Self> {
        match x {
            Self::DealNewStack => {
                match y {
                    Self::DealNewStack => {
                        vec![]
                    },
                    Self::DealWithIncrement(y_val) => {
                        let n = modulo(-y_val, modulus);
                        vec![Self::DealWithIncrement(n),
                             Self::Cut(y_val)]
                    },
                    Self::Cut(y_val) => {
                        let n = modulo(-y_val, modulus);
                        vec![Self::Cut(n),
                             Self::DealNewStack]
                    },
                }
            },
            Self::DealWithIncrement(x_val) => {
                match y {
                    Self::DealNewStack => {
                        let n = modulo(-x_val, modulus);
                        vec![Self::DealWithIncrement(n),
                             Self::Cut(1)]
                    },
                    Self::DealWithIncrement(y_val) => {
                        let n = modulo_mult(x_val, y_val, modulus);
                        vec![Self::DealWithIncrement(n)]
                    },
                    Self::Cut(_y_val) => {
                        unimplemented!() // Didn't figure out a good solution for this, but it's not needed since we always pull DealWithIncrement in front of Cut
                    },
                }
            },
            Self::Cut(x_val) => {
                match y {
                    Self::DealNewStack => {
                        let n = modulo(-x_val, modulus);
                        vec![Self::DealNewStack,
                             Self::Cut(n)]
                    },
                    Self::DealWithIncrement(y_val) => {
                        let n = modulo_mult(x_val, y_val, modulus);
                        vec![Self::DealWithIncrement(y_val),
                             Self::Cut(n)]
                    },
                    Self::Cut(y_val) => {
                        let n = modulo(x_val + y_val, modulus);
                        vec![Self::Cut(n)]
                    },
                }
            },
        }
    }
}

impl fmt::Display for Technique {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Technique::DealNewStack => write!(f, "Deal into new stack"),
            Technique::DealWithIncrement(n) => write!(f, "Deal with increment {}", n),
            Technique::Cut(n) => write!(f, "Cut {}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
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

    fn shuffle_many(&mut self, techniques: &[Technique]) {
        techniques.iter().for_each(|&t| self.shuffle(t))
    }

    fn find_card(&self, position: usize) -> Option<usize> {
        self.cards.iter().position(|&card| card == position)
    }
}

fn reduce(techniques: &[Technique], modulus: i64) -> Vec<Technique> {
    // Combine repeated techniques. Pull DealWithIncrement to the front. Push Cut to the back.
    // Thanks to /u/MegaGreenLightning for the idea
    let mut current_vec = techniques.to_owned();
    let mut fresh_data = true;
    while fresh_data == true {
        fresh_data = false;
        let mut next_vec = Vec::new();
        let mut i = 0;
        while (i + 1) < current_vec.len() {
            // First 3 match lines are for combining repeated techniques.
            // Next line is for moving DealWithIncrement to the front.
            // Last line is for moving Cut to the back.
            match (current_vec[i], current_vec[i + 1]) {
                (Technique::DealNewStack, Technique::DealNewStack) |
                (Technique::DealWithIncrement(_), Technique::DealWithIncrement(_)) |
                (Technique::Cut(_), Technique::Cut(_)) |
                (_, Technique::DealWithIncrement(_)) |
                (Technique::Cut(_), _) => {
                    let result = Technique::combine_or_swap(current_vec[i], current_vec[i + 1], modulus);
                    next_vec.extend(result);
                    i += 2; // Consume both items
                    fresh_data = true; // Set flag so that this pass is executed again (some data in the vec changed)
                },

                _ => {
                    // Nothing to do
                    next_vec.push(current_vec[i]);
                    i += 1; // Consume this item
                }
            }
        }

        // Last item in list may not have been processed. Do it now.
        if i < current_vec.len() {
            next_vec.push(current_vec[i]);
        }

        current_vec = next_vec;
    }

    current_vec
}

fn expand(techniques: &[Technique], modulus: i64, target: i64) -> Vec<Technique> {
    // For each of the 64 bits, create a techniques vector that can be performed to accomplish that number of shuffles. Not all bits will be needed.
    let mut bit_vec: Vec<Vec<Technique>> = Vec::new();
    let mut techniques_multiple = techniques.to_owned();
    for _ in 0..64 {
        bit_vec.push(techniques_multiple.clone());
        techniques_multiple.extend(techniques_multiple.clone()); // Double the vector
        techniques_multiple = reduce(&techniques_multiple, modulus);
    }

    let mut target_vec: Vec<Technique> = Vec::new();
    for (i, bit) in bit_vec.iter().enumerate().take(64) {
        if (target & (1 << i)) != 0 {
            target_vec.extend(bit);
        }
    }

    target_vec = reduce(&target_vec, modulus);
    target_vec
}

fn get_card_at_position(techniques: &[Technique], modulus: i64, position: usize) -> i64 {
    match techniques[..] {
        [Technique::DealWithIncrement(x), Technique::Cut(c)] => {
            // DealWithIncrement is the same as modular multiplication. We are trying to undo it and so
            // use modular division. Note that we are trying to find the card that ends up in a position, but
            // since there is a Cut after the Deal we need to adjust the position we are actually looking for.
            modulo_div(x, position as i64 + c, modulus)
        },
        _ => panic!("Reduced vec not in expected form"), // Expect to be in the form: DealWithIncrement, Cut
    }
}

#[aoc(day22, part2)]
pub fn solve(input: &str) -> i64 {
    let techniques = input.lines()
                          .map(|line| Technique::from_string(line.trim()))
                          .collect::<Vec<Technique>>();

    let deck_size = 119315717514047;
    let shuffle_count = 101741582076661;
    let reduce_result = reduce(&techniques, deck_size);
    let expand_result = expand(&reduce_result, deck_size, shuffle_count);
    let card_num = get_card_at_position(&expand_result, deck_size, 2020);
    println!("Card {} ended up in position 2020", card_num);
    card_num
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_modulo_mult() {
        let result = modulo_mult(5, 100, 17);
        assert_eq!(result, 7);

        let result = modulo_mult(0xFFFFFFFF, 5, 1_190_494_771);
        assert_eq!(result, 45930597);

        let result = modulo_mult(0x0FFFFFFF_FFFFFFFF, 5, 10_113_958_159);
        assert_eq!(result, 541975605);

        let result = modulo_mult(0x0FFFFFFF_FFFFFFFF, 0x0FFFFFFF_FFFFFFFF, 119_315_717_514_047);
        assert_eq!(result, 32154407593923);
    }

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

    fn both_ways_combine(t1: Technique, t2: Technique, deck_size: i64) -> (Deck, Deck) {
        let mut deck1 = Deck::new(deck_size as usize);
        deck1.shuffle(t1);
        deck1.shuffle(t2);

        let techniques_combined = Technique::combine_or_swap(t1, t2, deck_size);
        let mut deck2 = Deck::new(deck_size as usize);
        deck2.shuffle_many(&techniques_combined);

        (deck1, deck2)
    }

    #[test]
    fn test_combine_or_swap() {
        for &deck_size in [3, 5, 7, 11, 13, 17, 19, 23, 29, 31].iter() { // Prime deck sizes mean every x and y value less than deck_size is valid
            // DealNewStack + DealNewStack
            let (deck1, deck2) = both_ways_combine(Technique::DealNewStack, Technique::DealNewStack, deck_size);
            assert_eq!(deck1, deck2);

            // DealNewStack + DealWithIncrement
            for x in 1..deck_size {
                let (deck1, deck2) = both_ways_combine(Technique::DealNewStack, Technique::DealWithIncrement(x), deck_size);
                assert_eq!(deck1, deck2);
            }

            // DealNewStack + Cut
            for x in 1..deck_size {
                let (deck1, deck2) = both_ways_combine(Technique::DealNewStack, Technique::Cut(x), deck_size);
                assert_eq!(deck1, deck2);
            }

            // DealWithIncrement + DealNewStack
            for x in 1..deck_size {
                let (deck1, deck2) = both_ways_combine(Technique::DealWithIncrement(x), Technique::DealNewStack, deck_size);
                assert_eq!(deck1, deck2);
            }

            // DealWithIncrement + DealWithIncrement
            for x in 1..deck_size {
                for y in 1..deck_size {
                    let (deck1, deck2) = both_ways_combine(Technique::DealWithIncrement(x), Technique::DealWithIncrement(y), deck_size);
                    assert_eq!(deck1, deck2);
                }
            }

            // DealWithIncrement + Cut
            // Not implemented
            /*for x in 1..deck_size {
                for y in 1..deck_size {
                    let (deck1, deck2) = both_ways_combine(Technique::DealWithIncrement(x), Technique::Cut(y), deck_size);
                    assert_eq!(deck1, deck2);
                }
            }*/

            // Cut + DealNewStack
            for x in 1..deck_size {
                let (deck1, deck2) = both_ways_combine(Technique::Cut(x), Technique::DealNewStack, deck_size);
                assert_eq!(deck1, deck2);
            }

            // Cut + DealWithIncrement
            for x in 1..deck_size {
                for y in 1..deck_size {
                    let (deck1, deck2) = both_ways_combine(Technique::Cut(x), Technique::DealWithIncrement(y), deck_size);
                    assert_eq!(deck1, deck2);
                }
            }

            // Cut + Cut
            for x in 1..deck_size {
                for y in 1..deck_size {
                    let (deck1, deck2) = both_ways_combine(Technique::Cut(x), Technique::Cut(y), deck_size);
                    assert_eq!(deck1, deck2);
                }
            }
        }
    }

    fn both_ways_reduce(techniques: &Vec<Technique>, deck_size: i64) -> (Deck, Deck) {
        let mut deck1 = Deck::new(deck_size as usize);
        deck1.shuffle_many(&techniques);

        let mut deck2 = Deck::new(deck_size as usize);
        let result = reduce(&techniques, deck_size);
        deck2.shuffle_many(&result);

        (deck1, deck2)
    }

    #[test]
    fn test_reduce() {
        // Check DealNewStack
        let techniques = vec![Technique::DealNewStack,
                              Technique::DealNewStack,
                              Technique::DealNewStack];
        let (deck1, deck2) = both_ways_reduce(&techniques, 10007);
        assert_eq!(deck1, deck2);

        // Check DealWithIncrement
        let techniques = vec![Technique::DealWithIncrement(1),
                              Technique::DealWithIncrement(2),
                              Technique::DealWithIncrement(3),
                              Technique::DealWithIncrement(4)];
        let (deck1, deck2) = both_ways_reduce(&techniques, 10007);
        assert_eq!(deck1, deck2);

        // Check Cut
        let techniques = vec![Technique::Cut(1000),
                              Technique::Cut(2000),
                              Technique::Cut(3000),
                              Technique::Cut(4000)];
        let (deck1, deck2) = both_ways_reduce(&techniques, 10007);
        assert_eq!(deck1, deck2);

        // Check multiple
        let techniques = vec![Technique::Cut(1000),
                              Technique::DealNewStack,
                              Technique::DealNewStack,
                              Technique::Cut(2000),
                              Technique::Cut(3000),
                              Technique::Cut(4000),
                              Technique::DealWithIncrement(1),
                              Technique::DealWithIncrement(2),
                              Technique::DealWithIncrement(3),
                              Technique::DealWithIncrement(4),
                              Technique::DealNewStack];
        let (deck1, deck2) = both_ways_reduce(&techniques, 10007);
        assert_eq!(deck1, deck2);

        // Check that puzzle input reduces correctly
        let input = fs::read_to_string("input/2019/day22.txt")
                    .expect("Something went wrong reading the file");

        let techniques = input.lines()
                            .map(|line| Technique::from_string(line.trim()))
                            .collect::<Vec<Technique>>();

        let mut deck1 = Deck::new(10007);
        deck1.shuffle_many(&techniques);

        let mut deck2 = Deck::new(10007);
        let result = reduce(&techniques, 10007);
        deck2.shuffle_many(&result);

        assert_eq!(deck1, deck2);
    }

    #[test]
    fn test_expand() {
        // Check that a single vector expands correctly (vec_a expanded by x should match vec_a repeated x times and then reduced)
        for &modulus in [10007, 999983, 1_190_494_771, 10_113_958_159, 119315717514047].iter() {
            let vec_a = vec![Technique::Cut(1000),
                                Technique::DealNewStack,
                                Technique::DealWithIncrement(5)];
            let result_a = expand(&vec_a, modulus, 6);
            let vec_b = vec![Technique::Cut(1000), Technique::DealNewStack, Technique::DealWithIncrement(5),
                             Technique::Cut(1000), Technique::DealNewStack, Technique::DealWithIncrement(5),
                             Technique::Cut(1000), Technique::DealNewStack, Technique::DealWithIncrement(5),
                             Technique::Cut(1000), Technique::DealNewStack, Technique::DealWithIncrement(5),
                             Technique::Cut(1000), Technique::DealNewStack, Technique::DealWithIncrement(5),
                             Technique::Cut(1000), Technique::DealNewStack, Technique::DealWithIncrement(5)];
            let result_b = reduce(&vec_b, modulus);
            assert_eq!(result_a, result_b);
        }
    }
}
