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
    let mut i = 0;
    let mut input = input.to_vec();

    loop {
        // for j in &input {
        //     match j {
        //         Disk::File { id, blocks } => {
        //             print!("{}", id.to_string().repeat(*blocks));
        //         }
        //         Disk::Free { blocks } => {
        //             print!("{}", ".".repeat(*blocks));
        //         }
        //     }
        // }
        // println!();

        match input[i] {
            Disk::File { id: _, blocks: _ } => {
                i += 1;
            }
            Disk::Free {
                blocks: free_blocks,
            } => {
                if let Some(last) = input.pop() {
                    match last {
                        Disk::Free { blocks: _ } => continue,
                        Disk::File { id, blocks } => match blocks.cmp(&free_blocks) {
                            std::cmp::Ordering::Equal => {
                                input[i] = last;
                                i += 1;
                            }
                            std::cmp::Ordering::Less => {
                                input.insert(i, last);
                                input[i + 1] = Disk::Free {
                                    blocks: free_blocks - blocks,
                                };
                                i += 1;
                            }
                            std::cmp::Ordering::Greater => {
                                input[i] = Disk::File {
                                    id,
                                    blocks: free_blocks,
                                };
                                input.push(Disk::File {
                                    id,
                                    blocks: blocks - free_blocks,
                                });
                                i += 1;
                            }
                        },
                    }
                }
            }
        }
        if i >= input.len() {
            break;
        }
    }

    let mut res = 0;
    let mut pos = 0;
    for i in input {
        match i {
            Disk::File { id, blocks } => {
                for _ in 0..blocks {
                    res += pos * id;
                    pos += 1;
                }
            }
            _ => unreachable!(),
        }
    }
    res
}

#[aoc(day9, part2)]
pub fn part2(input: &[Disk]) -> usize {
    let mut j = input.len() - 1;
    let mut input = input.to_vec();

    loop {
        // for file in &input {
        //     match file {
        //         Disk::File { id, blocks } => {
        //             print!("{}", id.to_string().repeat(*blocks));
        //         }
        //         Disk::Free { blocks } => {
        //             print!("{}", ".".repeat(*blocks));
        //         }
        //     }
        // }
        // println!();

        match input[j] {
            Disk::Free { blocks: _ } => {
                j -= 1;
            }
            Disk::File {
                id: file_id,
                blocks: file_blocks,
            } => {
                let mut i = 0;
                while i < j {
                    match input[i] {
                        Disk::File { id: _, blocks: _ } => i += 1,
                        Disk::Free {
                            blocks: free_blocks,
                        } => match file_blocks.cmp(&free_blocks) {
                            std::cmp::Ordering::Equal => {
                                input[i] = Disk::File {
                                    id: file_id,
                                    blocks: file_blocks,
                                };
                                input[j] = Disk::Free {
                                    blocks: free_blocks,
                                };
                                break;
                            }
                            std::cmp::Ordering::Less => {
                                input.insert(
                                    i,
                                    Disk::File {
                                        id: file_id,
                                        blocks: file_blocks,
                                    },
                                );
                                j += 1;
                                input[i + 1] = Disk::Free {
                                    blocks: free_blocks - file_blocks,
                                };
                                input[j] = Disk::Free {
                                    blocks: file_blocks,
                                };
                                break;
                            }
                            std::cmp::Ordering::Greater => i += 1,
                        },
                    }
                }
                j -= 1;
            }
        }
        if j == 0 {
            break;
        }
    }

    let mut res = 0;
    let mut pos = 0;
    for i in input {
        match i {
            Disk::File { id, blocks } => {
                for _ in 0..blocks {
                    res += pos * id;
                    pos += 1;
                }
            }
            Disk::Free { blocks } => {
                for _ in 0..blocks {
                    pos += 1;
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(1928, part1(&input_generator(INPUT)))
    }

    #[test]
    fn test_part2() {
        assert_eq!(2858, part2(&input_generator(INPUT)))
    }
}
