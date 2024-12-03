use regex::Regex;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3, part1)]
pub fn input_generator_part1(input: &str) -> Vec<String> {
    let re = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    let matches: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();
    matches.iter().map(|&s| s.to_string()).collect()
}

#[aoc_generator(day3, part2)]
pub fn input_generator_part2(input: &str) -> Vec<String> {
    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
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
    let mut activated = true;
    let re = Regex::new(r"(?<a>\d+),(?<b>\d+)").unwrap();
    input.iter().fold(0, |mut acc, m| {
        match m.as_str() {
            "do()" => activated = true,
            "don't()" => activated = false,
            _ => {
                if activated {
                    let caps = re.captures(m).unwrap();
                    acc +=
                        caps["a"].parse::<usize>().unwrap() * caps["b"].parse::<usize>().unwrap();
                }
            }
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(161, part1(&input_generator_part1(INPUT_1)))
    }

    const INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part2() {
        assert_eq!(48, part2(&input_generator_part2(INPUT_2)))
    }
}
