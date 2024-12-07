use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Calibration {
    value: usize,
    numbers: Vec<usize>,
}
#[aoc_generator(day7)]
pub fn input_generator(_input: &str) -> Vec<Calibration> {
    // input.lines().fold(vec![], |mut reports, l| {
    //     reports.push(l.trim().split(" ").map(|s| s.parse().unwrap()).collect());
    //     reports
    // })
    vec![]
}

#[aoc(day7, part1)]
pub fn part1(input: &[Calibration]) -> usize {
    dbg!(input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(0, part1(&input_generator(INPUT)))
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(0, part2(&input_generator(INPUT)))
    // }
}
