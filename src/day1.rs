use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for l in input.lines() {
        let tmp = l.split_once("   ").unwrap();
        left.push(tmp.0.parse().unwrap());
        right.push(tmp.1.parse().unwrap());
    }
    right.sort();
    left.sort();

    (left, right)
}

#[aoc(day1, part1)]
pub fn part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut sum = 0;

    for n in 0..input.0.len() {
        sum += input.0[n].abs_diff(input.1[n])
    }

    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut sum = 0;

    for n in 0..input.0.len() {
        sum += input.0[n] * input.1.iter().filter(|&i| *i == input.0[n]).count()
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(11, part1(&input_generator(INPUT)))
    }

    #[test]
    fn test_part2() {
        assert_eq!(31, part2(&input_generator(INPUT)))
    }
}
