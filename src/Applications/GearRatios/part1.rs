#![allow(clippy::needless_return)]
#![allow(unused_assignments)]

use std::fs;

const SYMBOLS: &str = "*+&=$@/-%#";

#[derive(Debug, PartialEq)]
pub struct PartNumber {
    number: usize,
    row_index: usize,
    starting_index: usize,
    length: usize,
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    row: usize,
    column: usize,
    character: char,
}

pub fn main() {
    let input: String = fs::read_to_string("./inputs/day03/input.txt")
        .expect("Was not able to read, does the file exist?");

    println!("{}", part1(&input));
}

fn part1(input: &str) -> String {
    let (part_numbers, symbols) = collect_part_numbers_and_symbols(input);

    return sum_valid_part_numbers(part_numbers, symbols).to_string();
}

fn collect_part_numbers_and_symbols(input: &str) -> (Vec<PartNumber>, Vec<Symbol>) {
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (y, string) in input.lines().enumerate() {
        let mut partial_part_numer = String::new();
        let mut start_index: usize = 0;

        for (x, char) in string.chars().enumerate()  {
            if char.is_numeric() {
                if partial_part_numer.is_empty() {
                    start_index = x;
                }

                partial_part_numer = format!("{partial_part_numer}{char}");
            }

            if SYMBOLS.contains(char) {
                symbols.push(Symbol {
                    row: y,
                    column: x,
                    character: char,
                });
            }

            if !partial_part_numer.is_empty() && !char.is_numeric() {
                // part number sequence has ended!
                part_numbers.push(PartNumber {
                    number: partial_part_numer.parse::<usize>().unwrap(),
                    row_index: y,
                    starting_index: start_index,
                    length: partial_part_numer.len(),
                });

                partial_part_numer = String::new();
                start_index = 0;
            }
        }

        if !partial_part_numer.is_empty() {
            // part number sequence has ended!
            part_numbers.push(PartNumber {
                number: partial_part_numer.parse::<usize>().unwrap(),
                row_index: y,
                starting_index: start_index,
                length: partial_part_numer.len(),
            });

            partial_part_numer = String::new();
            start_index = 0;
        }
    }

    return (part_numbers, symbols);
}

fn sum_valid_part_numbers(part_numbers: Vec<PartNumber>, symbols: Vec<Symbol>) -> usize {
    let mut total = 0;

    for part_number in &part_numbers {
        let mut part_number_added = false;

        for symbol in &symbols {
            if part_number_added {
                continue;
            }

            let top_row = (part_number.row_index as isize - 1) as usize == symbol.row;
            let same_row = part_number.row_index == symbol.row;
            let bottom_row = part_number.row_index + 1 == symbol.row;

            if !top_row && !same_row && !bottom_row {
                continue;
            }
            let mut first_column = (part_number.starting_index as isize) - 1;
            let last_column = part_number.starting_index + part_number.length + 1;
            if first_column < 0 {
                first_column = 0;
            }

            let range = (first_column as usize)..last_column;

            if range.contains(&symbol.column) {
                total += part_number.number;
                part_number_added = true;
            }

        }
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_calculates_the_sum_of_part_numbers() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let result = part1(input);

        assert_eq!("4361", result);
    }

    #[test]
    fn it_calculates_the_sum_of_part_numbers_including_top_left() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.+.....58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let result = part1(input);

        assert_eq!("4361", result);
    }

    #[test]
    fn it_calculates_the_sum_of_part_numbers_real_data() {
        let input = "......644............612.......254..638..............802.................................118.....................................317.691....
.....*......321..176....+........&...=...906........*.......=518................994..938.*.....579....35....155...........320...........$...
...939.@225........*......................$........41......................./.....+......102....*.....*...............603....*.413=.........";
        let result = part1(input);

        assert_eq!("9007", result);
    }

    #[test]
    fn it_calculates_the_sum_of_part_numbers_more_data() {
        let input = "....\n.*..\n..21";
        let result = part1(input);

        assert_eq!("21", result);
    }

    #[test]
    fn it_handles_more_edge_cases() {
        let input = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";
        let result = part1(input);

        assert_eq!("925", result);
    }
}