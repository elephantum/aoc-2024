use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many1, separated_list0},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq)]
struct Data {
    rules: HashMap<i32, Vec<i32>>,
    updates: Vec<Vec<i32>>,
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list0(
        newline,
        separated_pair(
            nom::character::complete::i32,
            tag("|"),
            nom::character::complete::i32,
        ),
    )(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list0(
        newline,
        separated_list0(tag(","), nom::character::complete::i32),
    )(input)
}

fn parse_data(input: &str) -> IResult<&str, Data> {
    map(
        separated_pair(parse_rules, many1(newline), parse_updates),
        |(rules, updates)| Data {
            rules: {
                let mut map = HashMap::new();
                for (a, b) in rules {
                    map.entry(a).or_insert_with(Vec::new).push(b);
                }
                map
            },
            updates,
        },
    )(input)
}

#[test]
fn test_parse_data() {
    assert_eq!(
        parse_data(
            "47|53
97|13
97|61

75,47,61,53,29
97,61,53,29,13
97,13,75,29,47"
        ),
        Ok((
            "",
            Data {
                rules: HashMap::from_iter(vec![(47, vec![53]), (97, vec![13, 61])]),
                updates: vec![
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13],
                    vec![97, 13, 75, 29, 47],
                ]
            }
        ))
    );
}

fn is_update_correct(update: &[i32], rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut seen_pages = HashSet::new();

    for page in update.iter() {
        for rule in rules.get(page).unwrap_or(&vec![]).iter() {
            if seen_pages.contains(rule) {
                return false;
            }
        }

        seen_pages.insert(page);
    }

    true
}

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
    let data = parse_data(input).unwrap().1;

    let mut result = 0;

    for update in data.updates.iter() {
        if is_update_correct(update, &data.rules) {
            result += update[update.len() / 2];
        }
    }

    result
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        ),
        143
    );
}

fn reorder_pages(pages: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut pages = pages.clone();

    let mut res = vec![];

    while pages.len() > 0 {
        let mut rhs: HashSet<i32> = HashSet::new();
        for p in pages.iter() {
            rhs.extend(rules.get(p).unwrap_or(&vec![]));
        }

        for i in 0..pages.len() {
            let p = pages[i];
            if !rhs.contains(&p) {
                res.push(p.clone());
                pages.retain(|x| *x != p);
                break;
            }
        }
    }

    res
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
    let data = parse_data(input).unwrap().1;

    let mut result = 0;

    for update in data.updates.iter() {
        if !is_update_correct(update, &data.rules) {
            let reordered = reorder_pages(&update, &data.rules);
            result += reordered[reordered.len() / 2];
        }
    }

    result
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        ),
        123
    );
}
