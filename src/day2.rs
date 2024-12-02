use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input.lines().fold(vec![], |mut reports, l| {
        reports.push(l.trim().split(" ").map(|s| s.parse().unwrap()).collect());
        reports
    })
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .filter(|report| {
            let increasing = if report[0] < report[1] { true } else { false };
            let mut first = true;
            let mut last = 0;
            for val in report.iter() {
                if first {
                    last = *val;
                    first = false;
                    continue;
                }
                if val.abs_diff(last) < 1 || val.abs_diff(last) > 3 {
                    return false;
                }
                if increasing && *val < last {
                    return false;
                }
                if !increasing && *val > last {
                    return false;
                }
                last = *val;
            }
            true
        })
        .fold(0, |acc, _| acc + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(&input_generator(INPUT)))
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(31, part2(&input_generator(INPUT)))
    // }
}
