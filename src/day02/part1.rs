#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue
}

#[derive(Debug, PartialEq)]
struct Cube {
    amount: u8,
    color: Color,
}

#[derive(Debug, PartialEq)]
struct Set {
    cubes: Vec<Cube>
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<Set>
}

pub fn main() {
    // parser rules:
    //  Line starts with "Game x:" where X equals the game ID
    //  After the ":" the first set is revealed, until it hits the first ";"
    //      Each set has a number and a color, either "red", "green" or "blue"
    //
    //

    let game1 = Game{
        id: 1,
        sets: vec![
            Set{
                cubes:
                vec![
                    Cube{ amount: 3, color: Color::Blue },
                    Cube{ amount: 4, color: Color::Red },
                ],
            },
            Set{
                cubes:
                vec![
                    Cube{ amount: 1, color: Color::Red },
                    Cube{ amount: 2, color: Color::Green },
                    Cube{ amount: 6, color: Color::Blue },
                ]
            },
            Set{
                cubes:
                vec![
                    Cube{ amount: 2, color: Color::Green },
                ]
            }
        ]
    };
}

// fn game_parser(input: &str) -> Vec<Game> {
//     // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
// }

fn encode_game(game : &str) -> Game {
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
    return Game{ id, sets}
}

fn encode_set(set: &str) -> Set {
    let cubes_in_set: Vec<&str> = set.split(", ").collect::<Vec<&str>>();
    let mut cubes: Vec<Cube> = Vec::new();

    for cube in cubes_in_set {
        cubes.push(encode_cube(cube));
    }

    // input: 3 blue, 4 red
    // return: Set {cubes: [Cube{...}, Cube{...}]}
    return Set{ cubes }
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
    return Cube{amount, color};
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

        assert_eq!(Cube{amount:3, color:Color::Blue}, result);
    }

    #[test]
    fn it_encodes_sets_correctly() {
        let input = "3 blue, 4 red";
        let result = encode_set(input);

        assert_eq!(Set{ cubes: vec![
            Cube{amount:3, color:Color::Blue},
            Cube{amount:4, color:Color::Red}
        ] }, result);
    }

    #[test]
    fn it_encodes_games_correctly() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = encode_game(input);

        assert_eq!(Game{
            id: 1,
            sets: vec![
                Set{ cubes: vec![
                    Cube{amount:3, color:Color::Blue},
                    Cube{amount:4, color:Color::Red},
                ]},
                Set{ cubes: vec![
                    Cube{amount:1, color:Color::Red},
                    Cube{amount:2, color:Color::Green},
                    Cube{amount:6, color:Color::Blue},
                ]},
                Set{ cubes: vec![
                    Cube{amount:2, color:Color::Green},
                ]}
            ],
        }, result);
    }

//     #[test]
//     fn it_() {
//         let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
//
//         // encode_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
//     }
}