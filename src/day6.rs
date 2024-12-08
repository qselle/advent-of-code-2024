use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

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

#[derive(PartialEq, Eq, Copy, Clone)]
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
    let mut visited = input.map.clone();
    let (mut row, mut col) = input.current;
    let row_limit = input.map.len();
    let col_limit = input.map[0].len();
    let mut total: usize = 1;
    visited[row][col] = 'X';
    loop {
        let (next_row, next_col) = match direction {
            Direction::Up if row > 0 => (row - 1, col),
            Direction::Down if row < row_limit - 1 => (row + 1, col),
            Direction::Left if col > 0 => (row, col - 1),
            Direction::Right if col < col_limit - 1 => (row, col + 1),
            _ => break,
        };

        if visited[next_row][next_col] == '#' {
            direction = direction.rotate();
        } else if visited[next_row][next_col] == 'X' {
            row = next_row;
            col = next_col;
        } else {
            visited[next_row][next_col] = 'X';
            total += 1;
            row = next_row;
            col = next_col;
        }
    }
    // for i in visited {
    //     for j in i {
    //         print!("{j}");
    //     }
    //     println!();
    // }
    // println!();
    // println!();
    total
}

#[aoc(day6, part2)]
pub fn part2(input: &Map) -> usize {
    let mut total: usize = 0;

    let mut direction: Direction = Direction::Up;
    let row_limit = input.map.len();
    let col_limit = input.map[0].len();

    let (mut row, mut col) = input.current;
    let mut visited = input.map.clone();

    visited[row][col] = 'X';
    loop {
        let (next_row, next_col) = match direction {
            Direction::Up if row > 0 => (row - 1, col),
            Direction::Down if row < row_limit - 1 => (row + 1, col),
            Direction::Left if col > 0 => (row, col - 1),
            Direction::Right if col < col_limit - 1 => (row, col + 1),
            _ => break,
        };

        if input.map[next_row][next_col] == '#' {
            direction = direction.rotate();
        } else if visited[next_row][next_col] == 'X' {
            row = next_row;
            col = next_col;
        } else {
            visited[next_row][next_col] = 'X';
            let mut loop_visited = input.map.clone();
            let mut loop_direction = direction;
            loop_visited[next_row][next_col] = 'O';

            let mut visited: HashMap<(usize, usize), Direction> = HashMap::new();

            let (mut loop_row, mut loop_col) = (row, col);

            row = next_row;
            col = next_col;
            loop {
                let (next_loop_row, next_loop_col) = match loop_direction {
                    Direction::Up if loop_row > 0 => (loop_row - 1, loop_col),
                    Direction::Down if loop_row < row_limit - 1 => (loop_row + 1, loop_col),
                    Direction::Left if loop_col > 0 => (loop_row, loop_col - 1),
                    Direction::Right if loop_col < col_limit - 1 => (loop_row, loop_col + 1),
                    _ => break,
                };
                if loop_visited[next_loop_row][next_loop_col] == '#'
                    || loop_visited[next_loop_row][next_loop_col] == 'O'
                {
                    loop_direction = loop_direction.rotate();
                } else {
                    loop_row = next_loop_row;
                    loop_col = next_loop_col;
                    loop_visited[next_loop_row][next_loop_col] = 'X';
                    match visited.entry((loop_row, loop_col)) {
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(loop_direction);
                        }
                        std::collections::hash_map::Entry::Occupied(entry) => {
                            if *entry.get() == loop_direction {
                                loop_visited[loop_row][loop_col] = 'T';
                                // for i in loop_visited {
                                //     for j in i {
                                //         print!("{j}");
                                //     }
                                //     println!();
                                // }
                                // println!();
                                // println!();
                                total += 1;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    total
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

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(&input_generator(INPUT)))
    }
}
