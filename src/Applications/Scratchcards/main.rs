use advent_of_code_2023::common::Types::part::Part;
use crate::Applications::Scratchcards;

pub fn part(part: Part) {
    let number = part.part_number;
    return match number {
        1 => Scratchcards::part1::main(),
        2 => Scratchcards::part2::main(),
        _ => panic!("Part {number:?} not found!"),
    };
}