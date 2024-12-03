use regex::Regex;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    let re = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    let matches: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();
    matches.iter().map(|&s| s.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> usize {
    let re = Regex::new(r"(?<a>\d+),(?<b>\d+)").unwrap();
    input.iter().fold(0, |acc, m| {
        let caps = re.captures(m).unwrap();
        acc + (caps["a"].parse::<usize>().unwrap() * caps["b"].parse::<usize>().unwrap())
    })
}

#[aoc(day3, part2)]
pub fn part2(input: &[String]) -> usize {
    let re = Regex::new(r"(?<a>\d+),(?<b>\d+)").unwrap();
    input.iter().fold(0, |acc, m| {
        let caps = re.captures(m).unwrap();
        acc + (caps["a"].parse::<usize>().unwrap() * caps["b"].parse::<usize>().unwrap())
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(161, part1(&input_generator(INPUT)))
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2(&input_generator(INPUT)))
    }
}
