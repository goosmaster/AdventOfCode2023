#![allow(clippy::needless_return)]

use std::fs;

pub fn main() {
    let input: String = fs::read_to_string("./inputs/day01/input.txt")
        .expect("Was not able to read, does the file exist?");

    println!("{}", part1(&input))
}

fn part1(input: &str) -> String {
    let mut total: u32 = 0;
    let vec = input
        .split_whitespace()
        .map(|s: &str| s
                .chars()
                .filter(|c| c.
                    is_numeric()
                )
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for chars in vec {
        let first = chars.clone().into_iter().next().unwrap();
        let last = chars.clone().into_iter().next_back().unwrap();
        let number: String = format!("{}{}", first, last);

        total += number.parse::<u32>().unwrap();
    }

    return total.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_file_for_calibrations_correctly() {
        let result = part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        assert_eq!(result, "142".to_string());
    }
}