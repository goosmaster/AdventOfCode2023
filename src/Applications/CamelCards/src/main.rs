use advent_of_code_2023::common::Types::part::Part;
use crate::Applications::CamelCards;

pub fn part(part: Part) {
    let number = part.part_number;
    return match number {
        1 => CamelCards::src::part1::main(),
        2 => CamelCards::src::part2::main(),
        _ => panic!("Part {number:?} not found!"),
    };
}