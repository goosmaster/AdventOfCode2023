#![allow(clippy::needless_return)]

extern crate core;

use std::io::stdin;
mod common;
mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    println!("[day]-[part] :");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim() {
        "1-1" => day01::part1::main(),
        "1-2" => day01::part2::main(),
        "2-1" => day02::part1::main(),
        "2-2" => day02::part2::main(),
        "3-1" => day03::part1::main(),
        "3-2" => day03::part2::main(),
        "4-1" => day04::part1::main(),
        "4-2" => day04::part2::main(),
        _ => println!("Invalid day-part combination")
    }
}
