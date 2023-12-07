#![allow(clippy::needless_return)]
#![allow(dead_code)]

#[cfg(test)]
use rstest::rstest;

use std::fs;
use item::Item;
use item_type::ItemType;
use map::Map;
use crate::util::get_sub_str;

mod item;
mod item_type;
mod map;
mod util;

pub fn main() {
    let _input: String = fs::read_to_string("../../../inputs/day05/part1.txt")
        .expect("Was not able to read, does the file exist?");

    // let tmp = parse_item(&input);
    //
    // println!("{}", "blabla")
    todo!();
}

fn part1() {
    todo!();
}

fn parse_seeds(seeds_list: &str) -> Vec<Item> {
    let mut results: Vec<Item> = Vec::new();

    let parts: Vec<&str> = seeds_list
        .split(':')
        .map(|string: &str| string.trim())
        .collect::<Vec<&str>>();

    let category: &str = parts
        .first()
        .unwrap();

    if category != "seeds" {
        panic!("Parse seeds was asked to parse different category: {}", category)
    }

    let ids: Vec<u32> = parts
        .iter()
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|string: &str| string.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    for id in ids {
        results.push(Item{
            item_type: ItemType::from_string(category),
            id,
            child: None,
        })
    }

    return results;
}

fn parse_map(item_list: &str) -> Vec<Map> {
    let mut results: Vec<Map> = Vec::new();

    let parts: Vec<&str> = item_list
        .split(':')
        .map(|string: &str| string.trim())
        .collect::<Vec<&str>>();

    let to_category: &str = parts
        .first()
        .unwrap();

    let from_category = get_sub_str("", "-to-", to_category).unwrap();

    return parts
        .iter()
        .next_back()
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|string: &&str| {
            let values: Vec<u32> = string
                .split_whitespace()
                .map(|string: &str| string.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let destination_range_start = values[0];
            let source_range_start = values[1];
            let range_length = values[2];
            let mut destinations;
            let mut sources;

            if range_length == 0 {
                destinations = 0..0;
                sources = 0..0;
            } else if range_length == 1 {
                destinations = destination_range_start..destination_range_start;
                sources = source_range_start..source_range_start;
            } else if range_length == 2 {
                destinations = destination_range_start..destination_range_start + 1;
                sources = source_range_start..source_range_start + 1;
            } else {
                destinations = destination_range_start..destination_range_start + range_length - 1;
                sources = source_range_start..source_range_start + range_length - 1;
            }

            return Map{
                from: ItemType::from_string(&from_category),
                to: ItemType::from_string(to_category),
                destination_range_start,
                source_range_start,
                range_length,
                destinations,
                sources,
            }
        })
        .collect::<Vec<Map>>();
}

fn traverse_maps() -> Vec<Item> {
    // For each item
    //   Get maps that have from item.item_type
    //      For every seed (source) that is contained in map's destinations
    //          add
    todo!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case("seeds: 79", vec![Item{item_type: ItemType::Seed, id: 79, child: None }])]
    #[case("seeds: 1848591090 462385043", vec![Item{item_type: ItemType::Seed, id: 1848591090, child: None}, Item{item_type: ItemType::Seed, id: 462385043, child: None}])]
    fn it_parses_items_from_input_string(#[case] input: &str, #[case] expected: Vec<Item>) {
        let output = parse_seeds(input);
        assert_eq!(expected, output);
    }

    #[test]
    fn it_parses_mapping_from_input_string() {
        let input = "seed-to-soil map:
50 98 2
52 50 48";
        let output = parse_map(input);
        assert_eq!(vec![
            Map{
                from: ItemType::Seed,
                to: ItemType::Soil,
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2,
                destinations: 50..51,
                sources: 98..99,
            },
            Map{
                from: ItemType::Seed,
                to: ItemType::Soil,
                destination_range_start: 52,
                source_range_start: 50,
                range_length: 48,
                destinations: 52..99,
                sources: 50..97,
            }
        ], output);
    }

//     #[test]
//     fn it_() {
//         let input = "seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4
//
// water-to-light map:
// 88 18 7
// 18 25 70
//
// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13
//
// temperature-to-humidity map:
// 0 69 1
// 1 0 69
//
// humidity-to-location map:
// 60 56 37
// 56 93 4";
//
//     }
}