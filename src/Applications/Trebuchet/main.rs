use advent_of_code_2023::common::Types::part::Part;
use crate::Applications::Trebuchet;

pub fn part(part: Part) {
    let number = part.part_number;
    return match number {
        1 => Trebuchet::part1::main(),
        2 => Trebuchet::part2::main(),
        _ => panic!("Part {number:?} not found!"),
    };
}