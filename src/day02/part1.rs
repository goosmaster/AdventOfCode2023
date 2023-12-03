use std::fs;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct Cube {
    amount: u8,
    color: Color,
}

#[derive(Debug, PartialEq)]
struct Set {
    cubes: Vec<Cube>,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

const MAX_RED_CUBES: u8 = 12;
const MAX_GREEN_CUBES: u8 = 13;
const MAX_BLUE_CUBES: u8 = 14;

pub fn main() {
    let input: String = fs::read_to_string("./inputs/day02/input.txt")
        .expect("Was not able to read, does the file exist?");

    let games = game_parser(&input);
    let mut total: u32 = 0;

    for game in games {
        let mut is_valid = true;

        // if game is possible, add game id to total
        for set in game.sets {
            if is_valid == false {
                break;
            }

            for cube in set.cubes {
                if cube_stays_within_const_limits(cube) == false {
                    is_valid = false;
                }
            }
        }

        if is_valid {
            total += game.id;
        }
    }

    println!("The sum of all valid game IDs: {total}")
}

fn game_parser(games: &str) -> Vec<Game> {
    // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
    let games: Vec<&str> = games.split('\n').collect::<Vec<&str>>();
    let mut game: Vec<Game> = Vec::new();

    for game_input in games {
        game.push(encode_game(game_input));
    }

    return game;
}

fn encode_game(game: &str) -> Game {
    let parts = game.split(':').collect::<Vec<&str>>();

    let game_metadata = parts.iter().next().unwrap();
    let id = game_metadata.split(' ').collect::<Vec<&str>>().clone()
        .iter().next_back().unwrap().parse::<u32>().unwrap();

    let set_results = parts.iter().next_back().unwrap().split(';').collect::<Vec<&str>>();
    let mut sets: Vec<Set> = Vec::new();

    for set in set_results {
        sets.push(encode_set(set.trim()));
    }

    // input: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    // return: Game {id<u32>: 1, sets:[Set{...}, Set{...}, Set{...}]}
    return Game { id, sets };
}

fn encode_set(set: &str) -> Set {
    let cubes_in_set: Vec<&str> = set.split(", ").collect::<Vec<&str>>();
    let mut cubes: Vec<Cube> = Vec::new();

    for cube in cubes_in_set {
        cubes.push(encode_cube(cube));
    }

    // input: 3 blue, 4 red
    // return: Set {cubes: [Cube{...}, Cube{...}]}
    return Set { cubes };
}

fn encode_cube(cube: &str) -> Cube {
    let parts: Vec<&str> = cube.split(' ').collect::<Vec<&str>>();

    if parts.len() != 2 {
        panic!("parts was not equal to two, something went wrong: '{cube}'")
    }

    let amount: u8 = parts.clone().iter().next().unwrap().parse::<u8>().unwrap();
    let color: Color = str_to_color(parts.clone().iter().next_back().unwrap()).unwrap();

    // input: "3 blue"
    // return: Cube {amount<u8>: 3, color<Color>: Blue}
    return Cube { amount, color };
}

fn cube_stays_within_const_limits(cube: Cube) -> bool {
    match cube.color {
        Color::Red => cube.amount <= MAX_RED_CUBES,
        Color::Green => cube.amount <= MAX_GREEN_CUBES,
        Color::Blue => cube.amount <= MAX_BLUE_CUBES,
        _ => false
    }
}

fn str_to_color(string: &str) -> Option<Color> {
    match string.to_lowercase().as_str() {
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "blue" => Some(Color::Blue),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes_cubes_correctly() {
        let input = "3 blue";
        let result = encode_cube(input);

        assert_eq!(Cube { amount: 3, color: Color::Blue }, result);
    }

    #[test]
    fn it_encodes_sets_correctly() {
        let input = "3 blue, 4 red";
        let result = encode_set(input);

        assert_eq!(Set {
            cubes: vec![
                Cube { amount: 3, color: Color::Blue },
                Cube { amount: 4, color: Color::Red },
            ]
        }, result);
    }

    #[test]
    fn it_encodes_games_correctly() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = encode_game(input);

        assert_eq!(Game {
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
        }, result);
    }

    #[test]
    fn it_parses_multiple_games_correctly() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = game_parser(input);

        assert_eq!(vec![
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
        ], result);
    }

    #[test]
    fn it_correctly_validates_cube_limits() {
        let cube_within = Cube{amount: 1, color: Color::Red};
        let cube_outside = Cube{amount:100, color: Color::Red};

        assert_eq!(cube_stays_within_const_limits(cube_within), true);
        assert_eq!(cube_stays_within_const_limits(cube_outside), false);
    }
}