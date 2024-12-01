use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Locations {
    left: Vec<usize>,
    right: Vec<usize>,
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Locations {
    // input.lines().fold((vec![], vec![]), |mut lists, l| {
    //     let tmp = l.split("   ");
    //     (
    //         tmp.next().unwrap().parse().unwrap(),
    //         tmp.next().unwrap().parse().unwrap(),
    //     )
    // })
    let mut left = Vec::new();
    let mut right = Vec::new();

    for l in input.lines() {
        let mut tmp = l.split("   ");
        left.push(tmp.next().unwrap().parse().unwrap());
        right.push(tmp.next().unwrap().parse().unwrap());
    }
    right.sort();
    left.sort();
    Locations { right, left }
}

#[aoc(day1, part1)]
pub fn part1(input: &Locations) -> usize {
    let mut sum = 0;
    for n in 0..input.left.len() {
        sum += input.left[n].abs_diff(input.right[n])
    }
    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &Locations) -> usize {
    let mut sum = 0;
    for n in 0..input.left.len() {
        sum += input.left[n] * input.right.iter().filter(|&i| *i == input.left[n]).count()
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
