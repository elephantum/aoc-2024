use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();

        assert!(nums.len() == 2);
        left.push(nums[0]);
        right.push(nums[1]);
    }

    left.sort();
    right.sort();

    let res: i32 = left.iter().zip(right.iter()).map(|(a,b)| (a-b).abs()).sum();

    res
}

#[test]
fn test_part1() {
    assert_eq!(
        part1("3   4
4   3
2   5
1   3
3   9
3   3
"),
        11
    )
}


#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();

        assert!(nums.len() == 2);
        left.push(nums[0]);

        right.entry(nums[1]).and_modify(|x|*x+=1).or_insert(1);
    }

    let res: i32 = left.iter().map(|l| right.get(l).unwrap_or(&0) * l).sum();

    res
}


#[test]
fn test_part2() {
    assert_eq!(
        part2("3   4
4   3
2   5
1   3
3   9
3   3
"),
        31
    )
}
