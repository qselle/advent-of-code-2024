use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().fold(vec![], |mut reports, l| {
        reports.push(l.chars().collect());
        reports
    })
}

#[aoc(day4, part1)]
pub fn part1(input: &[Vec<char>]) -> usize {
    let mut num = 0;
    let directions = [
        (1, 1),   // down-right
        (-1, -1), // up-left
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // up
        (-1, 0),  // down
    ];

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            for &(dy, dx) in &directions {
                if (0..4).all(|i| {
                    let ny = y as isize + i * dy;
                    let nx = x as isize + i * dx;
                    ny >= 0
                        && ny < input.len() as isize
                        && nx >= 0
                        && nx < input[y].len() as isize
                        && match i {
                            0 => input[ny as usize][nx as usize] == 'X',
                            1 => input[ny as usize][nx as usize] == 'M',
                            2 => input[ny as usize][nx as usize] == 'A',
                            3 => input[ny as usize][nx as usize] == 'S',
                            _ => false,
                        }
                }) {
                    num += 1;
                }
            }
        }
    }
    num
}

#[aoc(day4, part2)]
pub fn part2(input: &[Vec<char>]) -> usize {
    let mut num = 0;
    let directions = [
        ((1, 1), (-1, -1)), // down-right and up-left
        ((1, -1), (-1, 1)), // down-left and up-right
    ];

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 'A' {
                let mut mas = 0;
                for &((dy1, dx1), (dy2, dx2)) in &directions {
                    let ny1 = y as isize + dy1;
                    let nx1 = x as isize + dx1;
                    let ny2 = y as isize + dy2;
                    let nx2 = x as isize + dx2;

                    if ny1 >= 0
                        && ny1 < input.len() as isize
                        && nx1 >= 0
                        && nx1 < input[y].len() as isize
                        && ny2 >= 0
                        && ny2 < input.len() as isize
                        && nx2 >= 0
                        && nx2 < input[y].len() as isize
                        && ((input[ny1 as usize][nx1 as usize] == 'M'
                            && input[ny2 as usize][nx2 as usize] == 'S')
                            || (input[ny1 as usize][nx1 as usize] == 'S'
                                && input[ny2 as usize][nx2 as usize] == 'M'))
                    {
                        mas += 1;
                    }
                }
                if mas == 2 {
                    num += 1;
                }
            }
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(18, part1(&input_generator(INPUT)))
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, part2(&input_generator(INPUT)))
    }
}
