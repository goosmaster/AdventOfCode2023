#![allow(clippy::needless_return)]

pub fn main() {
    println!("test")
}

fn part2(input: &str) -> String {
    let mut total: u32 = 0;
    let strings = input
        .split_whitespace()
        .collect::<Vec<&str>>();

    let vec = input
        .split_whitespace()
        .map(|s: &str| s

            .chars()
            .filter(|c| c.
                is_numeric()
            )
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for chars in vec {
        if chars.clone().len() == 0 {
            break;
        }

        let first = chars.clone().into_iter().next().unwrap();
        let last = chars.clone().into_iter().next_back().unwrap();
        let number: String = format!("{}{}", first, last);

        total += number.parse::<u32>().unwrap();
    }
    //
    // for string in strings {
    //     match string.to_lowercase() {
    //         s if s.contains("one") => total += 1,
    //         s if s.contains("two") => total += 2,
    //         s if s.contains("three") => total += 3,
    //         s if s.contains("four") => total += 4,
    //         s if s.contains("five") => total += 5,
    //         s if s.contains("six") => total += 6,
    //         s if s.contains("seven") => total += 7,
    //         s if s.contains("eight") => total += 8,
    //         s if s.contains("nine") => total += 9,
    //         _ => {}
    //     };
    // }

    return total.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_file_for_calibrations_correctly_named_digits() {
        let result = part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen");
        assert_eq!(result, "281".to_string());
    }
}