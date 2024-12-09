use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, value},
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
enum Entry {
    Do,
    DoNot,
    Mul { a: i32, b: i32 },
}

fn parse_data(input: &str) -> IResult<&str, Vec<Entry>> {
    let mul_parser = map(
        delimited(
            tag("mul("),
            separated_pair(
                nom::character::complete::i32,
                tag(","),
                nom::character::complete::i32,
            ),
            tag(")"),
        ),
        |(a, b)| Entry::Mul { a, b },
    );

    let do_parser = value(Entry::Do {}, tag("do()"));
    let do_not_parser = value(Entry::DoNot {}, tag("don't()"));

    many0(map(
        many_till(anychar, alt((mul_parser, do_parser, do_not_parser))),
        |(_, v)| v,
    ))(input)
}

#[test]
fn test_parse_data() {
    assert_eq!(
        parse_data("mul(1,2)"),
        Ok(("", vec![Entry::Mul { a: 1, b: 2 }]))
    );
    assert_eq!(
        parse_data("sdfsdafdsfamul(1,2)"),
        Ok(("", vec![Entry::Mul { a: 1, b: 2 }]))
    );
    assert_eq!(
        parse_data("mul(1,2)sdfsdgasdamul(3,4)"),
        Ok((
            "",
            vec![Entry::Mul { a: 1, b: 2 }, Entry::Mul { a: 3, b: 4 }]
        ))
    );
    assert_eq!(
        parse_data("mul(mul[2,4]mul(1,2)sdfsdgasdamul(3,4)"),
        Ok((
            "",
            vec![Entry::Mul { a: 1, b: 2 }, Entry::Mul { a: 3, b: 4 }]
        ))
    );
    assert_eq!(
        parse_data("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
        Ok((
            ")",
            vec![
                Entry::Mul { a: 2, b: 4 },
                Entry::Mul { a: 5, b: 5 },
                Entry::Mul { a: 11, b: 8 },
                Entry::Mul { a: 8, b: 5 }
            ]
        ))
    );
    assert_eq!(
        parse_data("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
        Ok((
            ")",
            vec![
                Entry::Mul { a: 2, b: 4 },
                Entry::DoNot,
                Entry::Mul { a: 5, b: 5 },
                Entry::Mul { a: 11, b: 8 },
                Entry::Do,
                Entry::Mul { a: 8, b: 5 }
            ]
        ))
    );
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Entry> {
    parse_data(input).unwrap().1
}

#[aoc(day3, part1)]
fn part1(input: &[Entry]) -> i32 {
    input
        .iter()
        .map(|x| match x {
            Entry::Mul { a, b } => a * b,
            _ => 0,
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(&[
            Entry::Mul { a: 2, b: 4 },
            Entry::Mul { a: 5, b: 5 },
            Entry::Mul { a: 11, b: 8 },
            Entry::Mul { a: 8, b: 5 }
        ]),
        161
    );
}

#[aoc(day3, part2)]
fn part2(input: &[Entry]) -> i32 {
    let mut enabled = true;

    input
        .iter()
        .map(|x| match x {
            Entry::Mul { a, b } => {
                if enabled {
                    a * b
                } else {
                    0
                }
            }
            Entry::Do => {
                enabled = true;
                0
            }
            Entry::DoNot => {
                enabled = false;
                0
            }
        })
        .sum()
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(&[
            Entry::Mul { a: 2, b: 4 },
            Entry::DoNot,
            Entry::Mul { a: 5, b: 5 },
            Entry::Mul { a: 11, b: 8 },
            Entry::Do,
            Entry::Mul { a: 8, b: 5 }
        ]),
        48
    );
}
