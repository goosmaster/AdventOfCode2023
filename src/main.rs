#![allow(clippy::needless_return)]

extern crate core;

use std::fs;
use std::io::{Error, stdin};
use std::process::exit;
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
    selection();
}

fn selection() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().to_lowercase() == "exit" {
        exit(0);
    }

    // let id = game_metadata.split(' ').collect::<Vec<&str>>().clone()
    //         .iter().next_back().unwrap().parse::<u32>().unwrap();

    let mut parts_iter = input.trim().split('-').map(|s| s.parse::<u8>().unwrap());
    let day = parts_iter.next().unwrap();
    let part = parts_iter.next().unwrap();

    match day {
        1 => Trebuchet::main::part(Part::from(part)),
        2 => CubeConundrum::main::part(Part::from(part)),
        3 => GearRatios::main::part(Part::from(part)),
        4 => Scratchcards::main::part(Part::from(part)),
        5 => SeedFertilizer::src::main::part(Part::from(part)),
        6 => WaitForIt::src::main::part(Part::from(part)),
        7 => CamelCards::src::main::part(Part::from(part)),
        8 => HauntedWasteland::src::main::part(Part::from(part)),
        9 => todo!(),
        10 => todo!(),
        11 => todo!(),
        12 => todo!(),
        13 => todo!(),
        14 => todo!(),
        15 => todo!(),
        16 => todo!(),
        17 => todo!(),
        18 => todo!(),
        19 => todo!(),
        20 => todo!(),
        21 => todo!(),
        22 => todo!(),
        23 => todo!(),
        24 => todo!(),
        25 => todo!(),
        _ => println!("Invalid day-part combination")
    }

    println!("Type \"exit\" to quit or another [day]-[part]:");
    selection();
}