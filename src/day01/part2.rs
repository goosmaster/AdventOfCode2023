#![allow(clippy::needless_return)]

// Todo: Solve bug...
// The current implementation, correctly executes a string replace from left to right
// The issue comes when (for example) "1twone" as input is given
// Due to the l2r replacement, "two" gets priority above "one"
// But! The solution requires the first and last valid number to count, so it should be "one" instead of "two"
// Couple of instant work around come to mind:
//      1. For each character, write a different parse order sequence, first -> l2r, last -> r2l
//      2. Instead of replacing (and there by mutilating the input), instead return a vec of "found" numbers in order.
//          then "just" simply take the first and last one.


use std::fs;
use std::ops::Add;

pub fn main() {
    let input: String = fs::read_to_string("./inputs/day01/input.txt")
        .expect("Was not able to read, does the file exist?");

    println!("{}", part2(&input))
}

fn part2(input: &str) -> String {
    let mut total: u32 = 0;

    let vec = input
        .split_whitespace()
        .map(|s: &str| l2r_digits_encoder(s)
            .chars()
            .filter(|c| c.
                is_numeric()
            )
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for chars in vec {
        if chars.is_empty() {
            break;
        }

        let first = chars.clone().into_iter().next().unwrap();
        let last = chars.clone().into_iter().next_back().unwrap();
        let number: String = format!("{}{}", first, last);

        total += number.parse::<u32>().unwrap();
    }

    return total.to_string();
}

fn l2r_digits_encoder(string: &str) -> String {
    let mut partial_string = String::new();

    for char in string.chars().collect::<Vec<char>>() {
        partial_string = partial_string.add(&char.to_string());

        partial_string = partial_string.replace("one", "1e")
            .replace("two", "2o")
            .replace("three", "3e")
            .replace("four", "4r")
            .replace("five", "5e")
            .replace("six", "6x")
            .replace("seven", "7n")
            .replace("eight", "8t")
            .replace("nine", "9e");
    }

    return partial_string;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_file_for_calibrations_correctly_named_digits() {
        let result = part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen");
        assert_eq!(result, "281".to_string());
    }

    #[test]
    fn it_converts_digits_to_numbers_left_to_right() {
        let input = "eightwothree";
        let result = l2r_digits_encoder(input);

        assert_eq!("82o3e", result)
    }

    #[test]
    fn it_parses_the_file_for_calibrations_correctly_named_digits_first_manual_test() {
        let result = part2("8jdzlpqvc89two\n2foursix69k\nxbqrmktzfive4\nfive9rjrvcpfbseightfkmlgbvqkbqj\nqnmkvkmckfxqmdtwosevendj6sevensixfive\nmkdvknghvsgzrbbjqngbsqeight6mjxfivenineq\n6two97mxm\ntwo26jjqjs\n1scslcns");
        // 82 + 29 + 54 + 58 + 25 + 89 + 67 + 26 + 11
        assert_eq!(result, "441".to_string());
    }

    #[test]
    fn it_parses_the_file_for_calibrations_correctly_named_digits_second_manual_test() {
        let result = part2("cbfxpgftninekthreexvmhxmkx1fourf\n76twoone\n7six32two1fivefspjtdsix\nnrcntbgdtjsevenztsmsgfmfour9thslsmhgnk2three\n21ltslbrnineseven7");
        // 94 + 71 + 76 + 73 + 27
        assert_eq!(result, "341".to_string());
    }

    #[test]
    fn it_parses_the_file_for_calibrations_correctly_named_digits_third_manual_test() {
        let result = part2("fdbtmkhdfzrck9kxckbnft");
        assert_eq!(result, "99".to_string());
    }

    #[test]
    fn it_parses_the_file_for_calibrations_correctly_named_digits_forth_manual_test() {
        let result = part2("gtxlzxdninexxhseven5\nxcdkvdg7nineeightsdjvkhzgmone\nzrnrrpxdfcnine2qgjxzfxcqgghbdk9\ntwozrckqbppsixhh7pmrmnktnrb7five\ntdxpjz2kccgqzcslm8sixsixtwo\n8tgcb32\neight391gxm99five8\none3fivegrpjr4trqxj5eighteight");
        // 95 + 71 + 99 + 25 + 22 + 82 + 88 + 18
        assert_eq!(result, "500".to_string());
    }

    #[test]
    fn it_parses_the_file_for_calibrations_correctly_named_digits_example_case_from_reddit() {
        // """The right calibration values for string "eighthree" is 83 and for "sevenine" is 79."""
        // https://www.reddit.com/r/adventofcode/comments/1884fpl/2023_day_1for_those_who_stuck_on_part_2/
        let result = part2("eighthree\nsevenine");
        // 83 + 79
        assert_eq!(result, "162".to_string());
    }
}