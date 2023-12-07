#![allow(clippy::needless_return)]

use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

pub fn main() {
    let input: String = fs::read_to_string("./inputs/day07/input.txt")
        .expect("Was not able to read, does the file exist?");

    println!("{}", part1(&input));
}

fn part1(input: &str) -> String {
    todo!();
}

fn score_table(card: &char) -> u8 {
    return match card {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'J' => 4,
        'T' => 5,
        '9' => 6,
        '8' => 7,
        '7' => 8,
        '6' => 9,
        '5' => 10,
        '4' => 11,
        '3' => 12,
        '2' => 13,
        _ => 0
    }
}

fn determine_hand(hand: &str) -> Hand {
    let mut deck: Vec<char> = hand.chars().collect::<Vec<char>>();
    let mut counts = HashMap::new();

    deck.sort_by(|a, b| {
        score_table(a)
            .partial_cmp(&score_table(b))
            .unwrap()
    });

    for ch in deck {
        *counts.entry(ch).or_insert(0) += 1;
    }

    let mut has_three = false;
    let mut has_two = false;
    let mut pairs = 0;

    for &count in counts.values() {
        match count {
            5 => return Hand::FiveOfAKind,
            4 => return Hand::FourOfAKind,
            3 => has_three = true,
            2 => {
                has_two = true;
                pairs += 1;
            },
            _ => (),
        }
    }

    if has_three && has_two {
        return Hand::FullHouse;
    } else if has_three {
        return Hand::ThreeOfAKind;
    } else if pairs == 2 {
        return Hand::TwoPair;
    } else if has_two {
        return Hand::OnePair;
    } else {
        return Hand::HighCard;
    }
}

fn sort() {
    // todo: generate vec of all hands
    //      first section of hand 32T3K is hand and needs sorting
    //          this is the only one pair
    //      KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
    //      T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.

    todo!();
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

//     #[test]
//     fn it_sorts_the_card_from_high_to_low() {
//         let input = "32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483";
//
//     }

    #[rstest]
    #[case("AAAAA", Hand::FiveOfAKind)]
    #[case("AA8AA", Hand::FourOfAKind)]
    #[case("23332", Hand::FullHouse)]
    #[case("TTT98", Hand::ThreeOfAKind)]
    #[case("23432", Hand::TwoPair)]
    #[case("A23A4", Hand::OnePair)]
    #[case("23456", Hand::HighCard)]
    fn it_correctly_identifies_hands(#[case] input: &str, #[case] expected: Hand) {
        let output = determine_hand(input);

        assert_eq!(expected, output);
    }
}