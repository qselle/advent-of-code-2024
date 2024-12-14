use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct City {
    frequencies: HashMap<char, HashSet<(usize, usize)>>,
    y_len: usize,
    x_len: usize,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> City {
    let mut frequencies: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    let map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c != '.' {
                        frequencies.entry(c).or_default().insert((y, x));
                    }
                    c
                })
                .collect()
        })
        .collect();
    City {
        y_len: map.len(),
        x_len: map[0].len(),
        frequencies,
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &City) -> usize {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let mut count = 0;
    for positions in input.frequencies.values() {
        for current_pos in positions {
            for around_pos in positions {
                if current_pos == around_pos {
                    continue;
                }
                let y = (current_pos.0 * 2).wrapping_sub(around_pos.0);
                let x = (current_pos.1 * 2).wrapping_sub(around_pos.1);
                if y < input.y_len && x < input.x_len && !antinodes.contains(&(y, x)) {
                    count += 1;
                    antinodes.insert((y, x));
                }
            }
        }
    }
    count
}

#[aoc(day8, part2)]
pub fn part2(input: &City) -> usize {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let mut count = 0;
    for positions in input.frequencies.values() {
        for current_pos in positions {
            for around_pos in positions {
                if current_pos == around_pos {
                    continue;
                }
                let delta_y: isize = current_pos.0 as isize - around_pos.0 as isize;
                let delta_x: isize = current_pos.1 as isize - around_pos.1 as isize;

                let mut y: isize = around_pos.0 as isize;
                let mut x: isize = around_pos.1 as isize;
                loop {
                    y -= delta_y;
                    x -= delta_x;
                    if x < 0 || y < 0 || x >= input.x_len as isize || y >= input.y_len as isize {
                        break;
                    }
                    if !antinodes.contains(&(y as usize, x as usize)) {
                        count += 1;
                        antinodes.insert((y as usize, x as usize));
                    }
                }

                y = around_pos.0 as isize;
                x = around_pos.1 as isize;
                loop {
                    y = y.wrapping_add(delta_y);
                    x = x.wrapping_add(delta_x);
                    if x < 0 || y < 0 || x >= input.x_len as isize || y >= input.y_len as isize {
                        break;
                    }
                    if !antinodes.contains(&(y as usize, x as usize)) {
                        count += 1;
                        antinodes.insert((y as usize, x as usize));
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    //     const INPUT: &str = "T.........
    // ...T......
    // .T........
    // ..........
    // ..........
    // ..........
    // ..........
    // ..........
    // ..........
    // ..........";

    #[test]
    fn test_part1() {
        assert_eq!(14, part1(&input_generator(INPUT)))
    }

    #[test]
    fn test_part2() {
        assert_eq!(34, part2(&input_generator(INPUT)))
    }
}
