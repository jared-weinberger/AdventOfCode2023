fn main() {
    println!("Hello, world!");
}
pub fn parse_calibration_string(line: &str) -> Option<u8> {
    let numbers = line.matches(char::is_numeric);
    let pair = numbers.fold(None, |current_pair: Option<(&str, &str)>, elem: &str| {
        current_pair.map_or_else(|| Some((elem, elem)), |(first, _)| Some((first, elem)))
    });
    let string_number = pair.map(|(first, last)| String::from(first) + last);
    string_number.and_then(|num_string| num_string.parse().ok())
}

mod test_parse_calibration_string {
    use crate::parse_calibration_string;

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
    fn no_numbers() {
        test_case("abcdefg", None);
    }

    fn test_case(input: &str, expected_result: Option<u8>) {
        assert_eq!(parse_calibration_string(input), expected_result);
    }
}
