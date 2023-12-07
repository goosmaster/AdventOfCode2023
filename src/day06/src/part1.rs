#![allow(clippy::needless_return)]

use std::collections::HashMap;
use std::fs;

pub fn main() {
    let input: String = fs::read_to_string("./inputs/day06/input.txt")
        .expect("Was not able to read, does the file exist?");

    println!("{}", part1(&input));
}

fn part1(input: &str) -> String {
    let races = get_races(input);
    let mut totals: Vec<usize> = Vec::new();

    for race in races {
        totals.push(
            get_strategies_available(race.0, race.1).len()
        )
    }

    return totals.iter().copied().product::<usize>().to_string();
}

#[derive(Debug, PartialEq)]
struct Strategy {
    hold_duration: usize,
    predicted_distance: usize,
}

fn get_races(input: &str) -> HashMap<usize, usize> {
    let race_data = input
        .lines()
        .map(|line| {
            line.split(' ')
                .filter(|string| {
                    return string
                        .trim()
                        .parse::<usize>()
                        .is_ok();
                })
                .map(|string| {
                    return string
                        .parse::<usize>()
                        .unwrap();
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let durations: Vec<usize> = race_data[0].clone();
    let distances: Vec<usize> = race_data[1].clone();
    let result: HashMap<usize, usize> = durations.iter().cloned().zip(distances.iter().cloned()).collect();

    return result;
}

fn get_strategies_available(race_duration: usize, record_distance: usize) -> Vec<Strategy> {
    let mut strategies: Vec<Strategy> = Vec::new();

    for i in 1..race_duration - 1 {
        let distance = i * (race_duration - i);

        if distance <= record_distance {
            continue;
        }

        strategies.push(Strategy{
            hold_duration: i,
            predicted_distance: distance
        })
    }

    return strategies;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    fn it_calculates_a_list_of_strategies_available(
        #[case] duration: usize,
        #[case] distance: usize,
        #[case] number_of_available_strategies: usize
    ) {
        let output = get_strategies_available(duration, distance).len();

        assert_eq!(
            number_of_available_strategies,
            output,
            "number of expected strategies: {} vs calculated {}",
            number_of_available_strategies,
            output)
        ;
    }

    #[rstest]
    #[case("Time:        40\nDistance:   219", HashMap::from([(40,219)]))]
    #[case("Time:        81\nDistance:   1012", HashMap::from([(81,1012)]))]
    #[case("Time:        77     72\nDistance:   1365   1089", HashMap::from([(77,1365), (72,1089)]))]
    fn it_parses_input_into_distinct_race_criteria(
        #[case] input: &str,
        #[case] expected: HashMap<usize, usize>
    ) {
        let output = get_races(input);

        assert_eq!(expected, output);
    }

    #[test]
    fn it_parses_input_into_num_strategies_available() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        let output = part1(input);

        assert_eq!("288".to_string(), output)
    }
}