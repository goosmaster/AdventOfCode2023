#![allow(clippy::needless_return)]

extern crate core;

use std::fs;
use std::io::{Error, stdin};

mod common;
mod Applications;
use Applications::Trebuchet;
use Applications::CubeConundrum;
use Applications::GearRatios;
use Applications::Scratchcards;
use Applications::SeedFertilizer;
use Applications::WaitForIt;
use Applications::CamelCards;
use Applications::HauntedWasteland;

fn main() {
    // if running in --interactive
    let pattern = std::env::args().collect::<Vec<String>>();

    if pattern.contains(&"--interactive".to_string()) || pattern.contains(&"-i".to_string()) {
        interactive();
    }
}

fn interactive() {
    match fs::read_to_string("./Assets/tree.txt") {
        Ok(tree) => println!("{}", tree.replace("\\x1b", "\u{1b}")),
        Err(e) => {},
    };

    println!("[day]-[part]:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim() {
        "1-1" => Trebuchet::part1::main(),
        "1-2" => Trebuchet::part2::main(),
        "2-1" => CubeConundrum::part1::main(),
        "2-2" => CubeConundrum::part2::main(),
        "3-1" => GearRatios::part1::main(),
        "3-2" => GearRatios::part2::main(),
        "4-1" => Scratchcards::part1::main(),
        "4-2" => Scratchcards::part2::main(),
        "5-1" => SeedFertilizer::src::part1::main(),
        "5-2" => SeedFertilizer::src::part2::main(),
        "6-1" => WaitForIt::src::part1::main(),
        "6-2" => WaitForIt::src::part2::main(),
        "7-1" => CamelCards::src::part1::main(),
        "7-2" => CamelCards::src::part2::main(),
        "8-1" => HauntedWasteland::src::part1::main(),
        "8-2" => HauntedWasteland::src::part2::main(),
        _ => println!("Invalid day-part combination")
    }
}
