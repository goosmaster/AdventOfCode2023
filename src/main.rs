use std::io::stdin;

mod common;
mod day01;

fn main() {
    println!("[day]-[part] :");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim() {
        "1-1" => day01::part1::main(),
        "1-2" => day01::part2::main(),
        _ => println!("Invalid day-part combination")
    }


}
