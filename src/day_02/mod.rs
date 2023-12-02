use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::{
        complete::{tag, take_while1},
        streaming::tag_no_case,
    },
    character::complete::space1,
    combinator::{map_res, recognize},
    multi::separated_list1,
    sequence::{preceded, separated_pair, Tuple},
    IResult, Parser,
};

type Colours = (u32, u32, u32);
fn add_colours(lhs: Colours, rhs: Colours) -> Colours {
    (lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
}
fn colours_scale(target: Colours, scale: u32) -> Colours {
    (target.0 * scale, target.1 * scale, target.2 * scale)
}

#[derive(Debug)]
struct Game {
    id: u32,
    // r, g, b
    max_colours: Colours,
}

pub struct Input {
    games: Vec<Game>,
}

fn game_id(input: &str) -> IResult<&str, u32> {
    preceded(
        tag_no_case("game "),
        map_res(recognize(take_while1(char::is_numeric)), str::parse),
    )(input)
}

fn game_view(input: &str) -> IResult<&str, Colours> {
    let result: IResult<&str, Vec<Colours>> = separated_list1(
        tag(", "),
        separated_pair(
            map_res(recognize(take_while1(char::is_numeric)), str::parse::<u32>),
            space1,
            alt((
                tag_no_case("red").map(|_| (1, 0, 0)),
                tag_no_case("green").map(|_| (0, 1, 0)),
                tag_no_case("blue").map(|_| (0, 0, 1)),
            )),
        )
        .map(|(num, colours)| colours_scale(colours, num)),
    )(input);
    result.map(|(result, parts)| {
        (
            result,
            parts.into_iter().reduce(add_colours).unwrap_or_default(),
        )
    })
}

fn parse_game_line(input: &str) -> Option<Game> {
    let (game_id, _, views) = (game_id, tag(": "), separated_list1(tag("; "), game_view))
        .parse(input)
        .ok()?
        .1;
    Some(Game {
        id: game_id,
        max_colours: views.iter().fold((0, 0, 0), |(r0, r1, r2), &(v0, v1, v2)| {
            (r0.max(v0), r1.max(v1), r2.max(v2))
        }),
    })
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    Input {
        games: input.lines().filter_map(parse_game_line).collect_vec(),
    }
}

#[aoc(day2, part1)]
pub fn part_1(input: &Input) -> u32 {
    input
        .games
        .iter()
        .filter_map(
            |&Game {
                 max_colours: (r, g, b),
                 id,
                 ..
             }| {
                match r <= 12 && g <= 13 && b <= 14 {
                    true => Some(id),
                    _ => None,
                }
            },
        )
        .sum()
}

#[aoc(day2, part2)]
pub fn part_2(input: &Input) -> u32 {
    input
        .games
        .iter()
        .map(
            |&Game {
                 max_colours: (r, g, b),
                 ..
             }| r * g * b,
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            "
        });
        assert_eq!(part_1(&input), 8);
        assert_eq!(part_2(&input), 2286);
    }

    #[yare::parameterized(
        single_digit = { "Game 1", 1 },
        double_digit = { "Game 10", 10 },
    )]
    fn game_id_parser(input: &str, expected_id: u32) {
        let (_, id) = game_id(input).unwrap();
        assert_eq!(id, expected_id);
    }

    #[yare::parameterized(
        just_red = { "1 red", (1, 0, 0) },
        just_green = { "1 green", (0, 1, 0) },
        just_blue = { "1 blue", (0, 0, 1) },
        two_colours = { "1 blue, 1 red", (1, 0, 1) },
        three_colours = { "1 blue, 1 green, 2 red", (2, 1, 1) },
    )]
    fn game_view_parser(input: &str, expected_view: Colours) {
        let (_, view) = game_view(input).unwrap();
        assert_eq!(view, expected_view);
    }

    #[yare::parameterized(
        game_1 = { "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", (4, 2, 6) } ,
        game_2 = { "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", (1, 3, 4) } ,
        game_3 = { "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", (20, 13, 6) } ,
        game_4 = { "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", (14, 3, 15) } ,
        game_5 = { "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", (6, 3, 2) } ,
    )]
    fn minimum_colours(input_line: &str, expected_mins: Colours) {
        let game = parse_game_line(input_line).unwrap();
        assert_eq!(game.max_colours, expected_mins);
    }
}
