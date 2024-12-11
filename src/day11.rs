use aoc_runner_derive::{aoc, aoc_generator};
use memoize::memoize;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(" ").map(|s| s.parse().unwrap()).collect()
}

#[memoize]
pub fn blink(stone: usize, counter: usize, limit: usize) -> usize {
    let mut stack = 0;
    if counter == limit {
        return 1;
    }

    if stone == 0 {
        stack += blink(1, counter + 1, limit);
    } else {
        let mut div = stone;
        let mut count = 0;
        while div > 0 {
            div /= 10;
            count += 1;
        }
        if count % 2 == 0 {
            stack += blink(stone / 10_usize.pow(count / 2), counter + 1, limit);
            stack += blink(stone % 10_usize.pow(count / 2), counter + 1, limit);
        } else {
            stack += blink(stone * 2024, counter + 1, limit);
        }
    }
    stack
}

#[aoc(day11, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut lenght = 0;
    for stone in input {
        lenght += blink(*stone, 0, 25);
    }
    lenght
}

#[aoc(day11, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut lenght = 0;
    for stone in input {
        lenght += blink(*stone, 0, 75);
    }
    lenght
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(55312, part1(&input_generator(INPUT)))
    }

    #[test]
    fn test_part2() {
        assert_eq!(55312, part2(&input_generator(INPUT)))
    }
}

// First implementation: too slow and no memoization possible.

// pub fn blink(stones: Vec<usize>, counter: usize, limit: usize) -> Vec<usize> {
//     if counter == limit {
//         return stones;
//     }
//     let stones = stones.iter().fold(vec![], |mut acc, stone| {
//         if *stone == 0 {
//             acc.push(1);
//             return acc;
//         }
//         let mut div = *stone;
//         let mut count = 0;
//         while div > 0 {
//             div /= 10;
//             count += 1;
//         }
//         if count % 2 == 0 {
//             acc.push(*stone / 10_usize.pow(count / 2));
//             acc.push(*stone % 10_usize.pow(count / 2));
//             return acc;
//         }
//         acc.push(2024 * *stone);
//         acc
//     });
//     blink(stones, counter + 1, limit)
// }

// #[aoc(day11, part1)]
// pub fn part1(input: &[usize]) -> usize {
//     blink(input.to_vec(), 0, 25).len()
// }
