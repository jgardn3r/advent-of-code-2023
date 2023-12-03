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

fn digit_or_word(
    input_bytes: &[u8],
    index_range: impl Iterator<Item = usize>,
    word_list: &[(&str, u32)],
) -> u32 {
    for char_index in index_range {
        let current_byte = input_bytes[char_index];
        if current_byte.is_ascii_digit() {
            return (current_byte - b'0') as u32;
        }
        for &(digit_word, value) in word_list {
            if char_index + digit_word.len() > input_bytes.len() {
                continue;
            }
            if &input_bytes[char_index..char_index + digit_word.len()] == digit_word.as_bytes() {
                return value;
            }
        }
    }
    panic!("there are no digits")
}

fn solve_part(input: &str, part_solver: impl Fn(&str) -> (u32, u32)) -> u32 {
    input
        .lines()
        .map(part_solver)
        .map(|(first, last)| first * 10 + last)
        .sum()
}

#[aoc(day1, part1)]
pub fn part_1(input: &str) -> u32 {
    solve_part(input, |s| {
        (
            digit_or_word(s.as_bytes(), 0..s.len(), &[]),
            digit_or_word(s.as_bytes(), (0..s.len()).rev(), &[]),
        )
    })
}

#[aoc(day1, part2)]
pub fn part_2(input: &str) -> u32 {
    solve_part(input, |s| {
        (
            digit_or_word(s.as_bytes(), 0..s.len(), &DIGIT_WORDS),
            digit_or_word(s.as_bytes(), (0..s.len()).rev(), &DIGIT_WORDS),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input1 = indoc! {
            "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
            "
        };
        assert_eq!(part_1(input1), 142);
        let input2 = indoc! {
            "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            "
        };
        assert_eq!(part_2(input2), 281);
    }

    #[yare::parameterized(
        test1 = { "1abc2", 1, 2 },
        test2 = { "pqr3stu8vwx", 3, 8 },
        test3 = { "a1b2c3d4e5f", 1, 5 },
        test4 = { "treb7uchet", 7, 7 },
    )]
    fn part1_lines_test(input: &str, first_digit: u32, last_digit: u32) {
        dbg!(input);
        assert_eq!(
            (
                digit_or_word(input.as_bytes(), 0..input.len(), &[]),
                digit_or_word(input.as_bytes(), (0..input.len()).rev(), &[]),
            ),
            (first_digit, last_digit)
        );
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
            (
                digit_or_word(input.as_bytes(), 0..input.len(), &DIGIT_WORDS),
                digit_or_word(input.as_bytes(), (0..input.len()).rev(), &DIGIT_WORDS),
            ),
            (first_digit, last_digit)
        );
    }
}
