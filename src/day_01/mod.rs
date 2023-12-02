use std::cmp::{max_by_key, min_by_key};

use itertools::Itertools;
use nom::FindSubstring;

type Input = Vec<String>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(str::to_string).collect_vec()
}

fn char_into_digit(input: &char) -> u32 {
    *input as u32 - ('0' as u32)
}

fn first_and_last_digit(input: &str) -> (u32, u32) {
    (
        input
            .chars()
            .filter(|c| c.is_numeric())
            .peekable()
            .peek()
            .map(char_into_digit)
            .unwrap(),
        input
            .chars()
            .filter(|c| c.is_numeric())
            .rev()
            .peekable()
            .peek()
            .map(char_into_digit)
            .unwrap(),
    )
}

const DIGIT_WORDS: [(&str, u32); 10] = [
    ("zero", 0),
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

fn min_by_index(a: Option<(usize, u32)>, b: Option<(usize, u32)>) -> (usize, u32) {
    match (a, b) {
        (Some(a), Some(b)) => min_by_key(a, b, |v| v.0),
        (Some(a), None) => a,
        (None, Some(b)) => b,
        _ => panic!("no digits found"),
    }
}
fn max_by_index(a: Option<(usize, u32)>, b: Option<(usize, u32)>) -> (usize, u32) {
    match (a, b) {
        (Some(a), Some(b)) => max_by_key(a, b, |v| v.0),
        (Some(a), None) => a,
        (None, Some(b)) => b,
        _ => panic!("no digits found"),
    }
}

fn first_digit_or_word(input: &str) -> u32 {
    let first_word_index = DIGIT_WORDS
        .iter()
        .filter_map(|(word, value)| input.find_substring(word).map(|index| (index, *value)))
        .min_by_key(|(index, _)| *index);
    let first_digit_index = input
        .char_indices()
        .find(|(_, char)| char.is_numeric())
        .map(|(index, char)| (index, char_into_digit(&char)));
    min_by_index(first_word_index, first_digit_index).1
}
fn last_digit_or_word(input: &str) -> u32 {
    let last_word_index = DIGIT_WORDS
        .iter()
        .filter_map(|(word, value)| {
            input
                .match_indices(word)
                .last()
                .map(|(index, _)| (index, *value))
        })
        .max_by_key(|(index, _)| *index);
    let last_digit_index = input
        .char_indices()
        .rev()
        .find(|(_, char)| char.is_numeric())
        .map(|(index, char)| (index, char_into_digit(&char)));
    max_by_index(last_word_index, last_digit_index).1
}

#[aoc(day1, part1)]
pub fn part_1(input: &Input) -> u32 {
    input
        .iter()
        .map(|s| first_and_last_digit(s))
        .map(|(first, last)| first * 10 + last)
        .sum()
}

#[aoc(day1, part2)]
pub fn part_2(input: &Input) -> u32 {
    input
        .iter()
        .map(|s| (first_digit_or_word(s), last_digit_or_word(s)))
        .map(|(first, last)| first * 10 + last)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input1 = input_generator(indoc! {
            "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
            "
        });
        assert_eq!(part_1(&input1), 142);
        let input2 = input_generator(indoc! {
            "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            "
        });
        assert_eq!(part_2(&input2), 281);
    }

    #[yare::parameterized(
        test1 = { "1abc2", 1, 2 },
        test2 = { "pqr3stu8vwx", 3, 8 },
        test3 = { "a1b2c3d4e5f", 1, 5 },
        test4 = { "treb7uchet", 7, 7 },
    )]
    fn part1_lines_test(input: &str, first_digit: u32, last_digit: u32) {
        dbg!(input);
        assert_eq!(first_and_last_digit(input), (first_digit, last_digit));
    }

    #[yare::parameterized(
        test1 = { "two1nine", 2, 9 },
        test2 = { "eightwothree", 8, 3 },
        test3 = { "abcone2threexyz", 1, 3 },
        test4 = { "xtwone3four", 2, 4 },
        test5 = { "4nineeightseven2", 4, 2 },
        test6 = { "zoneight234", 1, 4 },
        test7 = { "7pqrstsixteen", 7, 6 },
    )]
    fn part2_lines_test(input: &str, first_digit: u32, last_digit: u32) {
        dbg!(input);
        assert_eq!(
            (first_digit_or_word(input), last_digit_or_word(input)),
            (first_digit, last_digit)
        );
    }
}
