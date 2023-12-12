use advent_of_code_2023::common::Types::part::Part;
use crate::Applications::HauntedWasteland;

pub fn part(part: Part) {
    let number = part.part_number;
    return match number {
        1 => HauntedWasteland::src::part1::main(),
        2 => HauntedWasteland::src::part2::main(),
        _ => panic!("Part {number:?} not found!"),
    };
}