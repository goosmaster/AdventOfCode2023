#![allow(clippy::needless_return)]

use std::fs;

pub fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_file_for_calibrations_correctly() {
        let result = part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        assert_eq!(result, "142".to_string());
    }
}