use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s: &str| s.chars().collect()).collect()
}

#[aoc(day12, part1)]
pub fn part1(_input: &[Vec<char>]) -> usize {
    0
}

// #[aoc(day12, part2)]
// pub fn part2(_input: &[Vec<char>]) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(1930, part1(&input_generator(INPUT)))
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(65601038650482, part2(&input_generator(INPUT)))
    // }
}
