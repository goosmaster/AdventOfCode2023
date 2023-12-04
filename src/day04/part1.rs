#![allow(clippy::needless_return)]

use std::fs;

pub fn main() {
    let input: String = fs::read_to_string("./inputs/day04/input.txt")
        .expect("Was not able to read, does the file exist?");

    println!("{}", part1(&input))
}

fn part1(input: &str) -> String {
    todo!()
}