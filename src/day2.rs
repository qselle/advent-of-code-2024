use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input.lines().fold(vec![], |mut reports, l| {
        reports.push(l.trim().split(" ").map(|s| s.parse().unwrap()).collect());
        reports
    })
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .filter(|&report| {
            let increasing = report[0] < report[1];
            let mut last_level = 0;
            for (i, level) in report.iter().enumerate() {
                if i == 0 {
                    last_level = *level;
                    continue;
                }
                if level.abs_diff(last_level) < 1 || level.abs_diff(last_level) > 3 {
                    return false;
                }
                if increasing && *level < last_level {
                    return false;
                }
                if !increasing && *level > last_level {
                    return false;
                }
                last_level = *level;
            }
            true
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .filter(|&report| {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);

                let increasing = new_report[0] < new_report[1];
                let mut last_level = 0;

                let mut bad_level = false;
                for (i, level) in new_report.iter().enumerate() {
                    if i == 0 {
                        last_level = *level;
                        continue;
                    }
                    if level.abs_diff(last_level) < 1 || level.abs_diff(last_level) > 3 {
                        bad_level = true;
                        break;
                    }
                    if increasing && *level < last_level {
                        bad_level = true;
                        break;
                    }
                    if !increasing && *level > last_level {
                        bad_level = true;
                        break;
                    }
                    last_level = *level;
                }
                if !bad_level {
                    return true;
                }
            }
            false
        })
        .count()
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

    #[test]
    fn test_part2() {
        assert_eq!(4, part2(&input_generator(INPUT)))
    }
}
