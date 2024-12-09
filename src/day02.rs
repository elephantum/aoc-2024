use aoc_runner_derive::aoc;

fn is_safe_report(report: &Vec<i32>) -> bool {
    let is_decreasing = report
        .iter()
        .zip(report[1..].iter())
        .all(|(a, b)| a > b && a - b < 4);

    let is_increasing = report
        .iter()
        .zip(report[1..].iter())
        .all(|(a, b)| a < b && b - a < 4);

    is_decreasing || is_increasing
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|t| t.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    reports.iter().filter(|r| is_safe_report(r)).count()
}

#[test]
fn test_is_safe_report() {
    assert!(is_safe_report(&vec![7, 6, 4, 2, 1]));
    assert!(!is_safe_report(&vec![1, 2, 7, 8, 9]));
    assert!(is_safe_report(&vec![1, 3, 6, 7, 9]));
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        ),
        2
    )
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|t| t.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut count = 0;

    for report in reports.iter() {
        if is_safe_report(report) {
            count += 1;
        } else {
            for i in 0..report.len() {
                let mut report = report.clone();
                report.remove(i);
                if is_safe_report(&report) {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        ),
        4
    )
}
