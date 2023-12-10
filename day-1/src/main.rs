use regex::{Captures, Error as RegexError, Regex};
use std::fs::File;
use std::io::BufRead;

fn main() {
    let input_file = File::open(".input-data/input.txt").expect("Could not open input file");
    let reader = std::io::BufReader::new(input_file);
    let matcher = DigitMatcher::new().expect("Could not compile RegEx");
    let parser = CalibrationParser::new(matcher);
    let mut total: u32 = 0;
    for (line_number, line) in reader.lines().map(|line| line.ok()).enumerate() {
        let value = line
            .to_owned()
            .and_then(|line| parser.parse(line.as_str()))
            .unwrap_or(0);
        println!(
            "Line number {} | Value {} | Result {}",
            line_number + 1,
            line.unwrap_or("Unreadable".to_string()),
            value,
        );
        total += value;
    }
    println!("Total {}", total);
    // let total: u32 = reader
    //     .lines()
    //     .flatten()
    //     .flat_map(|line| parser.parse(&line))
    //     .sum();
    // println!("{}", total)
}

#[derive(Clone, Copy)]
enum DigitMatch<'l> {
    WordDigit(&'l str),
    PlainDigit(&'l str),
}

pub struct DigitMatcher {
    pattern: Regex,
}

impl DigitMatcher {
    pub fn new() -> Result<Self, RegexError> {
        Regex::new(r"(\d)|(one|two|three|four|five|six|seven|eight|nine)")
            .map(|pattern| Self { pattern })
    }

    fn get_matches<'l>(&self, haystack: &'l str) -> Vec<DigitMatch<'l>> {
        self.pattern
            .captures_iter(haystack)
            .flat_map(|capture: Captures| Self::get_digit_match(capture).to_owned())
            .collect()
    }

    fn get_digit_match<'l>(capture: Captures<'l>) -> Option<DigitMatch<'l>> {
        capture
            .get(1)
            .map(|x| DigitMatch::PlainDigit(x.as_str()))
            .or(capture.get(2).map(|x| DigitMatch::WordDigit(x.as_str())))
    }
}

pub struct CalibrationParser {
    matcher: DigitMatcher,
}
impl CalibrationParser {
    pub fn new(matcher: DigitMatcher) -> Self {
        Self { matcher }
    }

    pub fn parse(&self, line: &str) -> Option<u32> {
        let matches = self.matcher.get_matches(line);
        let numbers: Vec<u32> = [matches.first(), matches.last()]
            .iter()
            .flatten()
            .filter_map(|m| Self::parse_match(**m))
            .collect();
        match numbers.as_slice() {
            [first, last] => Some(first * 10 + last),
            [first] => Some(first * 11),
            _ => None,
        }
    }

    fn parse_match(digit_match: DigitMatch) -> Option<u32> {
        match digit_match {
            DigitMatch::PlainDigit(d) => d.parse().ok(),
            DigitMatch::WordDigit(w) => Self::parse_word_digit(w),
        }
    }

    fn parse_word_digit(text: &str) -> Option<u32> {
        match text {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test_parse_calibration_string {
    use super::*;

    #[test]
    fn number_at_beginning() {
        test_case("3abcde9", Some(39));
    }

    #[test]
    fn only_one_number() {
        test_case("dsfhjhwejh6dasfhjah", Some(66));
    }

    #[test]
    fn multiple_numbers() {
        test_case("i4n354pu4t5b", Some(45));
    }

    #[test]
    fn only_word_numbers() {
        test_case("twothree", Some(23));
    }

    #[test]
    fn all_words_supported() {
        let words = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];
        for (left_word, left_val) in words {
            for (right_word, right_val) in words {
                test_case(
                    (String::from(left_word) + right_word).as_str(),
                    Some(left_val * 10 + right_val),
                )
            }
        }
    }

    #[test]
    fn no_numbers() {
        test_case("abcdefg", None);
    }

    #[test]
    fn stage_two_examples() {
        let cases = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];
        for (text, val) in cases {
            test_case(text, Some(val));
        }
    }

    fn test_case(input: &str, expected_result: Option<u32>) {
        let matcher = DigitMatcher::new().unwrap();
        let parser: CalibrationParser = CalibrationParser::new(matcher);
        assert_eq!(parser.parse(input), expected_result);
    }
}
