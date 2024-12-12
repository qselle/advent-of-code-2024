use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|l| l.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

// #[memoize]
// pub fn blink(stone: usize, counter: usize, limit: usize) -> usize {
//     let mut stack = 0;
//     if counter == limit {
//         return 1;
//     }

//     if stone == 0 {
//         stack += blink(1, counter + 1, limit);
//     } else {
//         let mut div = stone;
//         let mut count = 0;
//         while div > 0 {
//             div /= 10;
//             count += 1;
//         }
//         if count % 2 == 0 {
//             stack += blink(stone / 10_usize.pow(count / 2), counter + 1, limit);
//             stack += blink(stone % 10_usize.pow(count / 2), counter + 1, limit);
//         } else {
//             stack += blink(stone * 2024, counter + 1, limit);
//         }
//     }
//     stack
// }

#[aoc(day10, part1)]
pub fn part1(input: &[Vec<usize>]) -> usize {
    // let directions = [
    //     (0, 1),  // right
    //     (0, -1), // left
    //     (1, 0),  // up
    //     (-1, 0), // down
    // ];
    // for (row_idx, row) in input.iter().enumerate() {
    //     dbg!(row_idx, row);
    //     for (col_idx, col) in row.iter().enumerate() {
    //         dbg!(col_idx, col);
    //         if *col == 0 {
    //             for &(dy, dx) in &directions {
    //                 let ny: isize = y as isize + i * dy;
    //                 let nx = x as isize + i * dx;
    //                 if ny >= 0
    //                     && ny < input.len() as isize
    //                     && nx >= 0
    //                     && nx < input[y].len() as isize
    //                 {
    //                     match *col {}
    //                 }
    //             }
    //             // match input[row_idx][col_idx + 1]
    //         }
    //     }
    // }
    0
}

// #[aoc(day10, part2)]
// pub fn part2(input: &[usize]) -> usize {
//     let mut lenght = 0;
//     for stone in input {
//         lenght += blink(*stone, 0, 75);
//     }
//     lenght
// }

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
        assert_eq!(55312, part1(&input_generator(INPUT)))
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(55312, part2(&input_generator(INPUT)))
    // }
}
