#![allow(clippy::needless_return)]

use std::collections::BTreeMap;
use std::fs;

#[derive(Debug, PartialEq)]
struct Card {
    card_id: u8,
    winning_numbers: Vec<u8>,
    card_numbers: Vec<u8>,
    wins: usize,
}

pub fn main() {
    let input: String = fs::read_to_string("./inputs/day04/input.txt")
        .expect("Was not able to read, does the file exist?");

    println!("{}", part2(&input))
}

fn part2(input: &str) -> String {
    let cards = get_cards(input);

    let data = cards
        .iter()
        .map(|card: &Card| card.wins)
        .collect::<Vec<usize>>();

    let mem = (0..cards.len())
        .map(|index| (index, 1))
        .collect::<BTreeMap<usize, usize>>();

    let result = data
        .iter()
        .enumerate()
        .fold(mem, |mut acc, (index, score)| {
            for i in (index + 1)..(index + 1 + *score) {
                let score_to_add = *acc.get(&index).unwrap();
                acc.entry(i).and_modify(|value| {
                    *value += score_to_add;
                });
            }

            return acc;
        })
        .values()
        .sum::<usize>();

    return result.to_string();
}

fn get_cards(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for card in input.lines() {
        let parts = card.split('|').collect::<Vec<&str>>();
        let card_numbers = parse_card_numbers(parts.clone());
        let card_info = parts.first().unwrap().split(':').collect::<Vec<&str>>();
        let winning_numbers = parse_card_numbers(card_info.clone());
        let card_id = card_info.first().unwrap().split("Card").last().unwrap().trim().parse::<u8>().unwrap();
        let mut wins: usize = 0;

        for winning_number in &winning_numbers {
            if card_numbers.contains(winning_number) {
                wins += 1;
            }
        }

        cards.push(Card {
            card_id,
            winning_numbers,
            card_numbers,
            wins,
        })
    }

    return cards;
}

fn parse_card_numbers(parts: Vec<&str>) -> Vec<u8> {
    return parts.last().unwrap().trim().split(' ')
        .filter(|s: &&str| !s.is_empty()).collect::<Vec<&str>>()
        .iter().map(|s: &&str| s.parse::<u8>().unwrap()).collect::<Vec<u8>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_the_points_of_the_given_scratchcard() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = part2(input);

        assert_eq!("30", output);
    }

    #[test]
    fn it_gets_the_correct_card_info_from_string_input() {
        let input = "Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let output = get_cards(input);

        assert_eq!(vec![
            Card{
                card_id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                card_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                wins: 4
            },
        ], output);
    }
}