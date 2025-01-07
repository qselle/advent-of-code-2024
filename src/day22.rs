use aoc_runner_derive::{aoc, aoc_generator};

const PRUNE: usize = 16777216;

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day22, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut total = 0;

    for secret in input {
        let mut secret = *secret;

        for _ in 0..2000 {
            let mut tmp;

            let operations = [
                |s: usize| s * 64,   // step1
                |s: usize| s / 32,   // step2
                |s: usize| s * 2048, // step3
            ];

            for op in operations {
                tmp = op(secret);
                secret ^= tmp;
                secret %= PRUNE;
            }
        }
        total += secret;
    }
    total
}

#[aoc(day22, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut onces:Vec<Vec<usize>> = vec![];

    for secret in input {
        let mut secret = *secret;

        for _ in 0..2000 {
            let mut tmp;

            let operations = [
                |s: usize| s * 64,   // step1
                |s: usize| s / 32,   // step2
                |s: usize| s * 2048, // step3
            ];

            for op in operations {
                tmp = op(secret);
                secret ^= tmp;
                secret %= PRUNE;
            }
        }
        total += secret;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
10
100
2024";

    #[test]
    fn test_part1() {
        assert_eq!(37327623, part1(&input_generator(INPUT)))
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(65601038650482, part2(&input_generator(INPUT)))
    // }
}
