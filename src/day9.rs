use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
pub enum Disk {
    File { id: usize, blocks: usize },
    Free { blocks: usize },
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Disk> {
    input
        .chars()
        .enumerate()
        .map(|(i, f)| {
            if i % 2 == 0 {
                Disk::File {
                    id: i / 2,
                    blocks: f.to_digit(10).unwrap() as usize,
                }
            } else {
                Disk::Free {
                    blocks: f.to_digit(10).unwrap() as usize,
                }
            }
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[Disk]) -> usize {
    for i in input {
        match i {
            Disk::File { id, blocks } => {
                print!("{}", id.to_string().repeat(*blocks));
            }
            Disk::Free { blocks } => {
                print!("{}", ".".repeat(*blocks));
            }
        }
    }
    println!();

    let mut i = 0;
    let mut input = input.to_vec();
    let mut results: Vec<usize> = Vec::new();

    loop {
        if i > input.len() {
            break;
        }

        match input[i] {
            Disk::File { id, blocks } => results.extend(std::iter::repeat(id).take(blocks)),
            Disk::Free {
                blocks: free_blocks,
            } => loop {
                if let Some(last) = input.pop() {
                    match last {
                        Disk::Free { blocks } => continue,
                        Disk::File { id: _, blocks } => {
                            if blocks == *free_blocks {
                                results.push(last);
                            }
                        }
                    }
                } else {
                    break;
                }
            },
        }
    }

    for i in results {
        match i {
            Disk::File { id, blocks } => {
                print!("{}", id.to_string().repeat(blocks));
            }
            Disk::Free { blocks } => {
                print!("{}", ".".repeat(blocks));
            }
        }
    }
    println!();

    0
}

// #[aoc(day9, part2)]
// pub fn part2(input: &[Disk]) -> usize {
//     // dbg!(input);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(1, part1(&input_generator(INPUT)))
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(1, part2(&input_generator(INPUT)))
    // }
}
