use advent_of_code_2023::common::Types::part::Part;
use crate::Applications::WaitForIt;

pub fn part(part: Part) {
    let number = part.part_number;
    return match number {
        1 => WaitForIt::src::part1::main(),
        2 => WaitForIt::src::part2::main(),
        _ => panic!("Part {number:?} not found!"),
    };
}