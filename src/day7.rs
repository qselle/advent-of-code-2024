use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Calibration {
    value: usize,
    numbers: Vec<usize>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Calibration> {
    input.lines().fold(vec![], |mut acc, l| {
        let tmp = l.split_once(":").unwrap();
        acc.push(Calibration {
            value: tmp.0.parse().unwrap(),
            numbers: tmp
                .1
                .trim()
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect(),
        });
        acc
    })
}

pub fn test_calibration(
    current: usize,
    target: usize,
    index: usize,
    params: &[usize],
    part2: bool,
) -> bool {
    let index = index + 1;
    if index >= params.len() {
        if current == target {
            return true;
        }
        return false;
    }
    test_calibration(current * params[index], target, index, params, part2)
        || test_calibration(current + params[index], target, index, params, part2)
        || (part2
            && test_calibration(
                current * 10_usize.pow(params[index].to_string().len() as u32) + params[index],
                target,
                index,
                params,
                part2,
            ))
}

#[aoc(day7, part1)]
pub fn part1(input: &[Calibration]) -> usize {
    let mut sum = 0;
    for c in input {
        if test_calibration(c.numbers[0], c.value, 0, &c.numbers, false) {
            sum += c.value;
        }
    }
    sum
}

#[aoc(day7, part2)]
pub fn part2(input: &[Calibration]) -> usize {
    let mut sum = 0;
    for c in input {
        if test_calibration(c.numbers[0], c.value, 0, &c.numbers, true) {
            sum += c.value;
        }
    }
    sum
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
        assert_eq!(3749, part1(&input_generator(INPUT)))
    }

    #[test]
    fn test_part2() {
        assert_eq!(11387, part2(&input_generator(INPUT)))
    }
}
