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

    println!("{}", part2(&input));
}

fn part2(input: &str) -> String {
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
        'T' => 4,
        '9' => 5,
        '8' => 6,
        '7' => 7,
        '6' => 8,
        '5' => 9,
        '4' => 10,
        '3' => 11,
        '2' => 12,
        'J' => 13,
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

    if counts.contains_key(&'J') {
        let most_cards_of = counts
            .iter()
            .filter(|deck| deck.0 != &'J')
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(key, _value)| key)
            .unwrap_or(&'J')
            .clone();

        if most_cards_of != 'J' {
            *counts.entry(most_cards_of).or_insert(0) += counts[&'J'];
        }
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
            let mut score_a: u8 = 0;
            let mut score_b: u8 = 0;

            for (i, card) in a.cards.chars().enumerate() {
                // highest score is worst cards.. (oops)
                score_a += card_score_table(&(a.cards.as_bytes()[i] as char));
                score_b += card_score_table(&(b.cards.as_bytes()[i] as char));

                if score_a < score_b {
                    order = Ordering::Greater;
                    break;
                } else if score_a > score_b {
                    order = Ordering::Less;
                    break;
                }
            }

            if order == Ordering::Equal {
                dbg!(((&a.cards, &a.hand, score_a), (&b.cards, &b.hand, score_b)));
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

    #[test]
    fn it_calculates_the_correct_total_bid_winnings() {
        let input = input_to_cards("2345A 1\nQ2KJJ 13\nQ2Q2Q 19\nT3T3J 17\nT3Q33 11\n2345J 3\nJ345A 2\n32T3K 5\nT55J5 29\nKK677 7\nKTJJT 34\nQQQJA 31\nJJJJJ 37\nJAAAA 43\nAAAAJ 59\nAAAAA 61\n2AAAA 23\n2JJJJ 53\nJJJJ2 41");
        let output = calculate_score(sort(input));

        assert_eq!(6839, output);
    }

    #[test]
    fn it_sorts_the_card_from_low_to_high() {
        let input = input_to_cards("2345A 1\nQ2KJJ 13\nQ2Q2Q 19\nT3T3J 17\nT3Q33 11\n2345J 3\nJ345A 2\n32T3K 5\nT55J5 29\nKK677 7\nKTJJT 34\nQQQJA 31\nJJJJJ 37\nJAAAA 43\nAAAAJ 59\nAAAAA 61\n2AAAA 23\n2JJJJ 53\nJJJJ2 41");
        let output = sort(input);

        //   left: 6839
        //  right: 6696

        //   left: 6839
        //  right: 6778

        assert_eq!(vec![
            Card { cards: "2345A".to_string(), hand: Hand::OnePair, bid: 1, },
            Card { cards: "J345A".to_string(), hand: Hand::OnePair, bid: 2, },
            Card { cards: "2345J".to_string(), hand: Hand::OnePair, bid: 3, },
            Card { cards: "32T3K".to_string(), hand: Hand::OnePair, bid: 5, },
            Card { cards: "KK677".to_string(), hand: Hand::OnePair, bid: 7, },
            Card { cards: "T3Q33".to_string(), hand: Hand::OnePair, bid: 11, },
            Card { cards: "Q2KJJ".to_string(), hand: Hand::OnePair, bid: 13, },
            Card { cards: "T3T3J".to_string(), hand: Hand::OnePair, bid: 17, },
            Card { cards: "Q2Q2Q".to_string(), hand: Hand::OnePair, bid: 19, },
            Card { cards: "2AAAA".to_string(), hand: Hand::OnePair, bid: 23, },
            Card { cards: "T55J5".to_string(), hand: Hand::OnePair, bid: 29, },
            Card { cards: "QQQJA".to_string(), hand: Hand::OnePair, bid: 31, },
            Card { cards: "KTJJT".to_string(), hand: Hand::OnePair, bid: 34, },
            Card { cards: "JJJJJ".to_string(), hand: Hand::OnePair, bid: 37, },
            Card { cards: "JJJJ2".to_string(), hand: Hand::OnePair, bid: 41, },
            Card { cards: "JAAAA".to_string(), hand: Hand::OnePair, bid: 43, },
            Card { cards: "2JJJJ".to_string(), hand: Hand::OnePair, bid: 53, },
            Card { cards: "AAAAJ".to_string(), hand: Hand::OnePair, bid: 59, },
            Card { cards: "AAAAA".to_string(), hand: Hand::OnePair, bid: 61, },
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
    #[case("T55J5", Hand::FourOfAKind)]
    #[case("KTJJT", Hand::FourOfAKind)]
    #[case("QQQJA", Hand::FourOfAKind)]
    #[case("QQQQJ", Hand::FiveOfAKind)]
    #[case("QQJ99", Hand::FullHouse)]
    #[case("QQJ98", Hand::ThreeOfAKind)]
    #[case("J8JJJ", Hand::FiveOfAKind)]
    #[case("JJJJJ", Hand::FiveOfAKind)]
    #[case("AABBJ", Hand::FullHouse)]
    fn it_correctly_identifies_hands(#[case] input: &str, #[case] expected: Hand) {
        let output = determine_hand(input);

        assert_eq!(expected, output);
    }
}