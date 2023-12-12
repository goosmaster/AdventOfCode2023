#![allow(clippy::needless_return)]

extern crate core;

use std::fs;
use std::io::{Error, stdin};
use advent_of_code_2023::common::Types::part::Part;

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
use common::Types::part;

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
        "1-1" => Trebuchet::main::part(Part::from(1)),
        "1-2" => Trebuchet::main::part(Part::from(2)),
        "2-1" => CubeConundrum::main::part(Part::from(1)),
        "2-2" => CubeConundrum::main::part(Part::from(2)),
        "3-1" => GearRatios::main::part(Part::from(1)),
        "3-2" => GearRatios::main::part(Part::from(2)),
        "4-1" => Scratchcards::main::part(Part::from(1)),
        "4-2" => Scratchcards::main::part(Part::from(2)),

        "5-1" => SeedFertilizer::src::main::part(Part::from(1)),
        "5-2" => SeedFertilizer::src::main::part(Part::from(2)),
        "6-1" => WaitForIt::src::main::part(Part::from(1)),
        "6-2" => WaitForIt::src::main::part(Part::from(2)),
        "7-1" => CamelCards::src::main::part(Part::from(1)),
        "7-2" => CamelCards::src::main::part(Part::from(2)),
        "8-1" => HauntedWasteland::src::main::part(Part::from(1)),
        "8-2" => HauntedWasteland::src::main::part(Part::from(2)),
        _ => println!("Invalid day-part combination")
    }
}
