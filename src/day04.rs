use aoc_runner_derive::aoc;

fn parse_data(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day04, part1)]
fn part1(input: &str) -> usize {
    let data = parse_data(input);

    let mut count = 0;

    for i in 0..data.len() - 3 {
        for j in 0..data[i].len() {
            let candidate = (0..4).map(|k| data[i + k][j]).collect::<String>();
            if candidate == "XMAS" || candidate == "SAMX" {
                count += 1;
            }
        }
    }

    for i in 0..data.len() {
        for j in 0..data[i].len() - 3 {
            let candidate = (0..4).map(|k| data[i][j + k]).collect::<String>();
            if candidate == "XMAS" || candidate == "SAMX" {
                count += 1;
            }
        }
    }

    for i in 0..data.len() - 3 {
        for j in 0..data[i].len() - 3 {
            let candidate = (0..4).map(|k| data[i + k][j + k]).collect::<String>();
            if candidate == "XMAS" || candidate == "SAMX" {
                count += 1;
            }
        }
    }

    for i in 0..data.len() - 3 {
        for j in 3..data[i].len() {
            let candidate = (0..4).map(|k| data[i + k][j - k]).collect::<String>();
            if candidate == "XMAS" || candidate == "SAMX" {
                count += 1;
            }
        }
    }

    count
}

#[test]
fn test_part1() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    assert_eq!(part1(input), 18);
}

#[aoc(day04, part2)]
fn part2(input: &str) -> usize {
    let data = parse_data(input);

    let mut count = 0;

    for i in 1..data.len() - 1 {
        for j in 1..data[i].len() - 1 {
            let candidate1 = (0..3)
                .map(|k| data[i + k - 1][j + k - 1])
                .collect::<String>();
            let candidate2 = (0..3)
                .map(|k| data[i + k - 1][j + 1 - k])
                .collect::<String>();

            if (candidate1 == "MAS" || candidate1 == "SAM") && (candidate2 == "MAS" || candidate2 == "SAM") {
                count += 1;
            }
        }
    }

    count
}


#[test]
fn test_part2() {
    let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

    assert_eq!(part2(input), 9);
}