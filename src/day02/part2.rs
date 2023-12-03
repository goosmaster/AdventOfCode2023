use std::fs;
use crate::day02::part1::{game_parser, Game, Set, Cube, Color};

pub fn main() {
    let input: String = fs::read_to_string("./inputs/day02/input.txt")
        .expect("Was not able to read, does the file exist?");

    let games: Vec<Game> = game_parser(&input);
    let total_power: u32 = get_total_power_sum(games);

    println!("The total sum of the power of all necessary cubes is: {total_power}")
}

fn get_total_power_sum(games: Vec<Game>) -> u32 {
    let mut total: u32 = 0;

    for game in games {
        let necessary_cube_amounts: Vec<Cube> = fewest_cubes_necessary_for_sets(game.sets);

        let red: u8 = necessary_cube_amounts.iter().find(|cube: &&Cube| cube.color == Color::Red).unwrap().amount;
        let green: u8 = necessary_cube_amounts.iter().find(|cube: &&Cube| cube.color == Color::Green).unwrap().amount;
        let blue: u8 = necessary_cube_amounts.iter().find(|cube: &&Cube| cube.color == Color::Blue).unwrap().amount;

        total += red as u32 * green as u32 * blue as u32;
    }

    return total;
}

fn fewest_cubes_necessary_for_sets(sets: Vec<Set>) -> Vec<Cube> {
    // {8 green}, {6 blue}, {20 red}        20 red  8 green  6 blue
    // {5 blue}, {4 red}, {13 green}        20 red 13 green  6 blue
    // {5 green}, {1 red}                   20 red 13 green  6 blue
    let mut necessary_red_cubes: u8 = 0;
    let mut necessary_green_cubes: u8 = 0;
    let mut necessary_blue_cubes: u8 = 0;

    for set in sets {
        for cube in set.cubes {
            match cube.color {
                Color::Red => { if cube.amount > necessary_red_cubes { necessary_red_cubes = cube.amount } }
                Color::Green => { if cube.amount > necessary_green_cubes { necessary_green_cubes = cube.amount } }
                Color::Blue => { if cube.amount > necessary_blue_cubes { necessary_blue_cubes = cube.amount } }
                _ => {}
            }
        }
    }

    return vec![
        Cube { amount: necessary_red_cubes, color: Color::Red },
        Cube { amount: necessary_green_cubes, color: Color::Green },
        Cube { amount: necessary_blue_cubes, color: Color::Blue },
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_the_fewest_cubes_necessary_for_given_sets() {
        let input: Vec<Set> = vec![
            Set {
                cubes: vec![
                    Cube { amount: 8, color: Color::Green },
                    Cube { amount: 6, color: Color::Blue },
                    Cube { amount: 20, color: Color::Red },
                ]
            },
            Set {
                cubes: vec![
                    Cube { amount: 5, color: Color::Blue },
                    Cube { amount: 4, color: Color::Red },
                    Cube { amount: 13, color: Color::Green },
                ]
            },
            Set {
                cubes: vec![
                    Cube { amount: 5, color: Color::Green },
                    Cube { amount: 1, color: Color::Red },
                ]
            },
        ];
        let result = fewest_cubes_necessary_for_sets(input);

        assert_eq!(vec![
            Cube { amount: 20, color: Color::Red },
            Cube { amount: 13, color: Color::Green },
            Cube { amount: 6, color: Color::Blue },
        ], result)
    }
    
    #[test]
    fn it_calculates_correct_total_power_sum() {
        let input = vec![
            // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game {
                id: 1,
                sets: vec![
                    Set {
                        cubes: vec![
                            Cube { amount: 3, color: Color::Blue },
                            Cube { amount: 4, color: Color::Red },
                        ]
                    },
                    Set {
                        cubes: vec![
                            Cube { amount: 1, color: Color::Red },
                            Cube { amount: 2, color: Color::Green },
                            Cube { amount: 6, color: Color::Blue },
                        ]
                    },
                    Set {
                        cubes: vec![
                            Cube { amount: 2, color: Color::Green },
                        ]
                    },
                ],
            },
            // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game {
                id: 2,
                sets: vec![
                    Set {
                        cubes: vec![
                            Cube { amount: 1, color: Color::Blue },
                            Cube { amount: 2, color: Color::Green },
                        ]
                    },
                    Set {
                        cubes: vec![
                            Cube { amount: 3, color: Color::Green },
                            Cube { amount: 4, color: Color::Blue },
                            Cube { amount: 1, color: Color::Red },
                        ]
                    },
                    Set {
                        cubes: vec![
                            Cube { amount: 1, color: Color::Green },
                            Cube { amount: 1, color: Color::Blue },
                        ]
                    },
                ],
            },
            // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game {
                id: 3,
                sets: vec![
                    Set {
                        cubes: vec![
                            Cube { amount: 8, color: Color::Green },
                            Cube { amount: 6, color: Color::Blue },
                            Cube { amount: 20, color: Color::Red },
                        ]
                    },
                    Set {
                        cubes: vec![
                            Cube { amount: 5, color: Color::Blue },
                            Cube { amount: 4, color: Color::Red },
                            Cube { amount: 13, color: Color::Green },
                        ]
                    },
                    Set {
                        cubes: vec![
                            Cube { amount: 5, color: Color::Green },
                            Cube { amount: 1, color: Color::Red },
                        ]
                    },
                ],
            },
            // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game {
                id: 4,
                sets: vec![
                    Set {
                        cubes: vec![
                            Cube { amount: 1, color: Color::Green },
                            Cube { amount: 3, color: Color::Red },
                            Cube { amount: 6, color: Color::Blue },
                        ]
                    },
                    Set {
                        cubes: vec![
                            Cube { amount: 3, color: Color::Green },
                            Cube { amount: 6, color: Color::Red },
                        ]
                    },
                    Set {
                        cubes: vec![
                            Cube { amount: 3, color: Color::Green },
                            Cube { amount: 15, color: Color::Blue },
                            Cube { amount: 14, color: Color::Red },
                        ]
                    },
                ],
            },
            // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            Game {
                id: 5,
                sets: vec![
                    Set {
                        cubes: vec![
                            Cube { amount: 6, color: Color::Red },
                            Cube { amount: 1, color: Color::Blue },
                            Cube { amount: 3, color: Color::Green },
                        ]
                    },
                    Set {
                        cubes: vec![
                            Cube { amount: 2, color: Color::Blue },
                            Cube { amount: 1, color: Color::Red },
                            Cube { amount: 2, color: Color::Green },
                        ]
                    },
                ],
            },
        ];
        let result = get_total_power_sum(input);

        assert_eq!(2286, result);
    }
}