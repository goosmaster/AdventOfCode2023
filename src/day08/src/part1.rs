#![allow(clippy::needless_return)]

use std::fs;
use crate::day08::src::DecissionPath::DecisionPath;

pub fn main() {
    let input: String = fs::read_to_string("./inputs/day08/input.txt")
        .expect("Was not able to read, does the file exist?");

    println!("{}", part1(&input));
}

fn part1(input: &str) -> i32 {
    todo!();
}

fn parse_direction_to_key(direction: char) -> Option<u8> {
    return match direction {
        'l' | 'L' => Some(0u8),
        'r' | 'R' => Some(1u8),
        _ => None,
    }
}

fn append_to_decision_path(options: (&str, &str), path: &DecisionPath) -> DecisionPath {
    todo!();
}


#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[ignore]
    #[test]
    fn it_calculates_step_count() {
        let input = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
        let expected = 2;

        assert_eq!(expected, part1(input));
    }

    #[rstest]
    #[case('L', Some(0))]
    #[case('R', Some(1))]
    #[case('r', Some(1))]
    #[case('3', None)]
    fn it_translate_rl_into_keys(#[case] input: char, #[case] expected: Option<u8>) {
        assert_eq!(expected, parse_direction_to_key(input));
    }
}