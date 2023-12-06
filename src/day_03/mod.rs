use itertools::Itertools;

fn is_symbol(c: u8) -> bool {
    !matches!(c, b'0'..=b'9' | b'.')
}

fn check_for_numbers<Data: Clone>(
    line_bytes: &[u8],
    starting_index: usize,
    data: &mut Data,
    number_operation: &mut impl FnMut(u32, &mut Data),
) {
    match line_bytes.get(starting_index).map(|c| c.is_ascii_digit()) {
        Some(true) => {
            let number = scan_in_number(line_bytes, starting_index);
            number_operation(number, data);
        }
        Some(false) => {
            let number = scan_in_number(line_bytes, starting_index + 1);
            number_operation(number, data);
            if let Some(index) = starting_index.checked_sub(1) {
                let number = scan_in_number(line_bytes, index);
                number_operation(number, data);
            }
        }
        _ => {}
    }
}

fn part_x<Data: Clone>(
    input: &str,
    is_relevant: impl Fn(u8) -> bool,
    starting_data: Data,
    mut number_operation: impl FnMut(u32, &mut Data),
    total_operation: impl Fn(Data) -> u32,
) -> u32 {
    let lines = input.lines().collect_vec();
    let mut total = 0;
    for (line_index, line) in lines.iter().enumerate() {
        let line_bytes = line.as_bytes();
        for (char_index, char) in line.bytes().enumerate() {
            if !is_relevant(char) {
                continue;
            }

            let mut data = starting_data.clone();

            if char_index
                .checked_sub(1)
                .and_then(|i| line_bytes.get(i))
                .map(|b| b.is_ascii_digit())
                .unwrap_or(false)
            {
                let number = scan_in_number(line_bytes, char_index - 1);
                number_operation(number, &mut data);
            }

            if char_index
                .checked_add(1)
                .and_then(|i| line_bytes.get(i))
                .map(|b| b.is_ascii_digit())
                .unwrap_or(false)
            {
                let number = scan_in_number(line_bytes, char_index + 1);
                number_operation(number, &mut data);
            }

            if let Some(line_bytes) = line_index
                .checked_sub(1)
                .and_then(|i| lines.get(i))
                .map(|line| line.as_bytes())
            {
                check_for_numbers(line_bytes, char_index, &mut data, &mut number_operation);
            }
            if let Some(line_bytes) = line_index
                .checked_add(1)
                .and_then(|i| lines.get(i))
                .map(|line| line.as_bytes())
            {
                check_for_numbers(line_bytes, char_index, &mut data, &mut number_operation);
            }
            total += total_operation(data);
        }
    }
    total
}

#[aoc(day3, part1)]
pub fn part_1(input: &str) -> u32 {
    part_x(input, is_symbol, 0, |num, t| *t += num, |t| t)
}

fn scan_in_number(line_bytes: &[u8], starting_index: usize) -> u32 {
    let mut c = starting_index;
    let mut number_value = 0;
    if let Some(false) = line_bytes.get(c).map(|c| c.is_ascii_digit()) {
        return 0;
    }
    loop {
        match line_bytes.get(c).map(|c| c.is_ascii_digit()) {
            Some(false) => {
                c += 1;
                break;
            }
            Some(true) => {
                c = match c.checked_sub(1) {
                    Some(c) => c,
                    _ => break,
                }
            }
            None => break,
        }
    }
    while let Some((char, true)) = line_bytes.get(c).map(|c| (c, c.is_ascii_digit())) {
        c += 1;
        number_value *= 10;
        number_value += (char - b'0') as u32;
    }
    number_value
}

#[aoc(day3, part2)]
pub fn part_2(input: &str) -> u32 {
    part_x(
        input,
        |c| c == b'*',
        (0, 1),
        |num, (c, p)| {
            if num > 0 {
                *c += 1;
                *p *= num;
            }
        },
        |(c, p)| {
            if c == 2 {
                p
            } else {
                0
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = indoc! {
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            "
        };
        assert_eq!(part_1(input), 4361);
        assert_eq!(part_2(input), 467835);
    }

    #[yare::parameterized(
        n = {
            indoc! {
                "
                .*.
                .1.
                ...
                "
            },
            1,
        },
        ne = {
            indoc! {
                "
                ..*
                .1.
                ...
                "
            },
            1,
        },
        e = {
            indoc! {
                "
                ...
                .1*
                ...
                "
            },
            1,
        },
        se = {
            indoc! {
                "
                ...
                .1.
                ..*
                "
            },
            1,
        },
        s = {
            indoc! {
                "
                ...
                .1.
                .*.
                "
            },
            1,
        },
        sw = {
            indoc! {
                "
                ...
                .1.
                *..
                "
            },
            1,
        },
        w = {
            indoc! {
                "
                ...
                *1.
                ...
                "
            },
            1,
        },
        nw = {
            indoc! {
                "
                *..
                .1.
                ...
                "
            },
            1,
        },
        same_line_n = {
            indoc! {
                "
                ..*..
                .1.1.
                .....
                "
            },
            2,
        },
        same_line_c = {
            indoc! {
                "
                .....
                .1*1.
                .....
                "
            },
            2,
        },
        same_line_s = {
            indoc! {
                "
                .....
                .1.1.
                ..*..
                "
            },
            2,
        },
        big_n = {
            indoc! {
                "
                ...*...
                .11111.
                .......
                "
            },
            11111,
        },
        big_s = {
            indoc! {
                "
                .......
                .11111.
                ...*...
                "
            },
            11111,
        },
        test = {
            indoc! {
                "
                ..512.......
                ........*228
                ....348.....
                "
            },
            228,
        },
    )]
    fn symbol_test(input: &str, expected_result: u32) {
        assert_eq!(part_1(input), expected_result);
    }

    #[yare::parameterized(
        alone = { "1", 0 },
        no_left = { "1.", 0 },
        no_right = { ".1", 0 },
        spaced_left = { "#.1", 0 },
        spaced_right = { "1.#", 0 },
    )]
    fn no_symbol_test(input: &str, expected_result: u32) {
        assert_eq!(part_1(input), expected_result);
    }

    #[yare::parameterized(
        a = { ".101.", 0, 0 },
        b = { ".101.", 1, 101 },
        c = { ".101.", 2, 101 },
        d = { ".101.", 3, 101 },
        e = { ".101.", 4, 0 },
    )]
    fn number_scanning(input: &str, starting_index: usize, expected_result: u32) {
        assert_eq!(
            scan_in_number(input.as_bytes(), starting_index),
            expected_result
        );
    }

    #[yare::parameterized(
        one_side = {
            indoc! {
                "
                ..512.......
                ........*228
                ....348.....
                "
            },
            0,
        },
        two_side = {
            indoc! {
                "
                ..512.......
                .....348*228
                ............
                "
            },
            79344,
        },
        two_top = {
            indoc! {
                "
                ..512.......
                .....348.228
                ........*...
                "
            },
            79344,
        },
        two_left = {
            indoc! {
                "
                .....348....
                ........*...
                .....228....
                "
            },
            79344,
        },
        two_right = {
            indoc! {
                "
                .....348....
                ....*.......
                .....228....
                "
            },
            79344,
        },
        two_bottom = {
            indoc! {
                "
                ..512...*...
                .....348.228
                ............
                "
            },
            79344,
        },
        across_top = {
            indoc! {
                "
                101
                .*2
                "
            },
            202,
        },
        across_bottom = {
            indoc! {
                "
                .*2
                111
                "
            },
            222,
        },
        sandwich3 = {
            indoc! {
                "
                ...1111
                ...*...
                1111...
                "
            },
            1234321,
        },
        sandwich2 = {
            indoc! {
                "
                ....111
                ...*...
                111....
                "
            },
            12321,
        },
        sandwich1 = {
            indoc! {
                "
                111*111
                ....1..
                "
            },
            0,
        },
        sandwich = {
            indoc! {
                "
                111
                .*.
                111
                "
            },
            12321,
        },
        overload = {
            indoc! {
                "
                ..512..1*1..
                .......111..
                ............
                "
            },
            0,
        },
    )]
    fn gear_test(input: &str, expected_result: u32) {
        assert_eq!(part_2(input), expected_result);
    }
}
