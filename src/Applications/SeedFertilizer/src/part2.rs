#![allow(clippy::needless_return)]

use std::fs;

pub fn main() {
    let input: String = fs::read_to_string("../../../inputs/SeedFertilizer/part1.txt")
        .expect("Was not able to read, does the file exist?");

    println!("{}", part2(&input))
}

fn part2(input: &str) -> String {
    todo!()
}