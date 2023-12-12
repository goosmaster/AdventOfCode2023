#![allow(clippy::needless_return)]
#![allow(unused_assignments)]

use std::fs;

const GEAR: char = '*';

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

    println!("{}", part2(&input));
}

fn part2(input: &str) -> String {
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

            if char == GEAR {
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


    // todo: for each symbol (only gears)
    // todo:    check row above and below for any part_number

    for symbol in &symbols {
        let mut part_numer_mem: Vec<PartNumber> = Vec::new();

        for part_number in &part_numbers {
            let top_row = (symbol.row as isize - 1) as usize == part_number.row_index;
            let same_row = symbol.row == part_number.row_index;
            let bottom_row = symbol.row + 1 == part_number.row_index;

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
                part_numer_mem.push(PartNumber {
                    number: part_number.number,
                    row_index: part_number.row_index,
                    starting_index: part_number.starting_index,
                    length: part_number.length,
                });
            }

        }

        if part_numer_mem.len() == 2 {
            total += part_numer_mem.first().unwrap().number * part_numer_mem.last().unwrap().number;
        }
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_correctly_calculates_the_gear_ratios() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let output = part2(input);

        assert_eq!("467835", output);
    }

    #[test]
    fn it_correctly_calculates_the_gear_ratios_with_more_edge_cases() {
        let input = "12.......*..\n+.........34\n.......-12..\n..78........\n..*....60...\n78.........9\n.5.....23..$\n8...90*12...\n............\n2.2......12.\n.*.........*\n1.1..503+.56";
        let output = part2(input);

        assert_eq!("6756", output);
    }
}