use std::ops::Shl;

use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::{digit1, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Card {
    id: usize,
    winner_count: usize,
}
#[derive(Debug)]
pub struct Input {
    cards: Vec<Card>,
}

fn parse_id(input: &str) -> IResult<&str, usize> {
    tuple((
        tag_no_case("card"),
        space1,
        map_res(digit1, str::parse::<usize>),
    ))(input)
    .map(|(s, (_, _, id))| (s, id))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, map_res(digit1, str::parse::<u32>))(input)
}

fn parse_card(input: &str) -> Option<Card> {
    let (_, (id, _, _, _, winners, _, _, _, picks)) = tuple((
        parse_id,
        space0,
        tag(":"),
        space0,
        parse_numbers,
        space0,
        tag("|"),
        space0,
        parse_numbers,
    ))(input)
    .ok()?;
    let winner_count = picks.iter().filter(|pick| winners.contains(pick)).count();
    Some(Card { id, winner_count })
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    Input {
        cards: input.lines().filter_map(parse_card).collect(),
    }
}

#[aoc(day4, part1)]
pub fn part_1(input: &Input) -> u32 {
    input
        .cards
        .iter()
        .filter(|card| card.winner_count > 0)
        .fold(0, |acc, card| acc + 1usize.shl(card.winner_count - 1)) as u32
}

#[aoc(day4, part2)]
pub fn part_2(input: &Input) -> u32 {
    input
        .cards
        .iter()
        .fold(vec![1; input.cards.len()], |mut acc, card| {
            (card.id..card.id + card.winner_count)
                .for_each(|winner_id| acc[winner_id] += acc[card.id - 1]);
            acc
        })
        .iter()
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
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
            "
        });
        assert_eq!(part_1(&input), 13);
        assert_eq!(part_2(&input), 30);
    }
}
