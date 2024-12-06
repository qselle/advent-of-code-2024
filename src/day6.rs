use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Map {
    map: Vec<Vec<char>>,
    current: (usize, usize),
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Map {
    Map {
        map: input.lines().map(|l| l.chars().collect()).collect(),
        current: input
            .lines()
            .enumerate()
            .find_map(|(row, line)| line.chars().position(|c| c == '^').map(|col| (row, col)))
            .unwrap(),
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // Clockwise 90-degree rotation
    fn rotate(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &Map) -> usize {
    let mut direction: Direction = Direction::Up;
    let (mut y, mut x) = input.current;
    let mut new_map = input.map.clone();
    let mut count = 0;
    while y < input.map.len() && x < input.map[y].len() {
        if input.map[y][x] == '#' {
            // put back
            match direction {
                Direction::Up => y += 1,
                Direction::Down => y -= 1,
                Direction::Left => x += 1,
                Direction::Right => x -= 1,
            }
            direction = direction.rotate();
        } else if new_map[y][x] != 'X' {
            count += 1;
            new_map[y][x] = 'X';
        }
        match direction {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(41, part1(&input_generator(INPUT)))
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(123, part2(&input_generator(INPUT)))
    // }
}
