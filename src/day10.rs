use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    let mut starts = vec![];
    input
        .lines()
        .enumerate()
        .map(|(y, s)| {
            s.chars()
            .enumerate()
                .map(|(x, l)| {
                    if l == '0' { 
                        starts.push((y,x))
                    } l.to_digit(10).unwrap() as usize
        })
                .collect()
        })
        .collect()
}

pub fn hike(input: &[Vec<usize>], pos: (usize, usize), current: usize, visited: &mut HashSet<(usize,usize)>) -> usize {
    if current == 9 {
        if !visited.contains(&(pos.0, pos.1)) {
            visited.insert((pos.0, pos.1));
            return 1;
        }
        return 0;
    }
    let mut path = 0;

    let directions = [
        (0, -1), // left,
        (0, 1), // right,
        (-1, 0), // up,
        (1, 0), // down,
    ];
    
    for (dy, dx) in directions {
        let ny = pos.0 as isize + dy;
        let nx = pos.1 as isize + dx;
        if ny >= 0
        && ny < input.len() as isize
        && nx >= 0
        && nx < input[pos.0].len() as isize
        && input[ny as usize][nx as usize] == current + 1 {
            path += hike(input, (ny as usize, nx as usize), current+1, visited);
        }

    }
    path
}

#[aoc(day10, part1)]
pub fn part1(input: &[Vec<usize>]) -> usize {
    let mut score = 0;
    let mut count = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 0 {
                let mut visited: HashSet<(usize,usize)> = HashSet::new();
                score += hike(input, (y, x), 0, &mut visited);    
                count +=1;
                dbg!(count, score);
            }
        }
    }
    dbg!(count, score);
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(36, part1(&input_generator(INPUT)))
    }
}
