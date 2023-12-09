#![allow(clippy::needless_return)]

use std::cmp::Ordering;
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

#[derive(Debug, PartialEq)]
struct Card {
    cards: String,
    hand: Hand,
    bid: i32,
}

pub fn main() {
    let input: String = fs::read_to_string("./inputs/day07/input.txt")
        .expect("Was not able to read, does the file exist?");

    println!("{}", part1(&input));
}

fn part1(input: &str) -> String {
    let cards = sort(input_to_cards(input));
    let score = calculate_score(cards);

    return score.to_string();
}

fn calculate_score(cards: Vec<Card>) -> usize {
    let mut total_winnings: usize = 0;
    for (index, card) in cards.iter().enumerate() {
        total_winnings += card.bid as usize * (index + 1);
    }

    return total_winnings;
}

fn input_to_cards(input: &str) -> Vec<Card> {
    return input
        .lines()
        .map(|line| to_card(line))
        .collect::<Vec<Card>>();
}

fn hand_score_table(hand: &Hand) -> u8 {
    return match hand {
        Hand::FiveOfAKind => 1,
        Hand::FourOfAKind => 2,
        Hand::FullHouse => 3,
        Hand::ThreeOfAKind => 4,
        Hand::TwoPair => 5,
        Hand::OnePair => 6,
        Hand::HighCard => 7,
    }
}

fn card_score_table(card: &char) -> u8 {
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
        card_score_table(a)
            .partial_cmp(&card_score_table(b))
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

fn to_card(input: &str) -> Card {
    let parts = input.split(' ').collect::<Vec<&str>>();
    let cards = parts.first().unwrap().to_owned().to_string();
    let bid = parts.last().unwrap().parse::<i32>().unwrap();
    let hand = determine_hand(&cards);

    return Card {
        cards,
        hand,
        bid,
    }
}

fn sort(mut cards: Vec<Card>) -> Vec<Card> {
    cards.sort_by(|a, b| {
        if a.hand == b.hand {
            let mut order: Ordering = Ordering::Equal;

            for (i, card) in a.cards.chars().enumerate() {
                let score_a = card_score_table(&(a.cards.as_bytes()[i] as char));
                let score_b = card_score_table(&(b.cards.as_bytes()[i] as char));

                if score_a < score_b {
                    order = Ordering::Greater;
                    break;
                } else if score_a > score_b {
                    order = Ordering::Less;
                    break;
                }
            }

            return order;
        } else {
            hand_score_table(&b.hand)
                .partial_cmp(&hand_score_table(&a.hand))
                .unwrap()
        }
    });

    return cards;
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    fn it_calculates_the_correct_total_bid_winnings() {
        let input = input_to_cards("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483");
        let output = calculate_score(sort(input));

        assert_eq!(6440, output);
    }

    #[test]
    fn it_sorts_the_card_from_low_to_high() {
        let input = input_to_cards("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483");
        let output = sort(input);

        assert_eq!(vec![
            // Rank 1 (so will be multiplied by 1)
            Card { cards: "32T3K".to_string(), hand: Hand::OnePair, bid: 765, },
            // Rank 2 (so will be multiplied by 2)
            Card { cards: "KTJJT".to_string(), hand: Hand::TwoPair, bid: 220, },
            // Rank 3 (so will be multiplied by 3)
            Card { cards: "KK677".to_string(), hand: Hand::TwoPair, bid: 028, },
            // Rank 4 (so will be multiplied by 4)
            Card { cards: "T55J5".to_string(), hand: Hand::ThreeOfAKind, bid: 684, },
            // Rank 5 (so will be multiplied by 5)
            Card { cards: "QQQJA".to_string(), hand: Hand::ThreeOfAKind, bid: 483, },
        ], output);
    }

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