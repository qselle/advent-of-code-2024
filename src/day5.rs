use aoc_runner_derive::{aoc, aoc_generator};
use std::{cmp::Ordering, collections::HashMap, vec};

#[derive(Debug)]
pub struct Rules {
    ordering: HashMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Rules {
    let mut ordering = HashMap::new();
    let mut updates = vec![];
    let (one, two) = input.split_once("\n\n").unwrap();

    for l in one.lines() {
        let sides = l.split_once("|").unwrap();
        ordering
            .entry(sides.1.parse().unwrap())
            .or_insert_with(Vec::new)
            .push(sides.0.parse().unwrap());
    }
    for l in two.lines() {
        updates.push(l.split(",").map(|s| s.parse().unwrap()).collect());
    }
    Rules { ordering, updates }
}

#[aoc(day5, part1)]
pub fn part1(input: &Rules) -> usize {
    let mut total = 0;
    'outer: for update in &input.updates {
        for n in 0..update.len() {
            if let Some(must_be_before) = input.ordering.get(&update[n]) {
                for m in update.iter().skip(n) {
                    if must_be_before.contains(m) {
                        continue 'outer;
                    }
                }
            }
        }
        total += update[update.len() / 2]
    }
    total
}

#[aoc(day5, part2)]
pub fn part2(input: &Rules) -> usize {
    let mut invalids = vec![];
    'outer: for update in &input.updates {
        for n in 0..update.len() {
            if let Some(must_be_before) = input.ordering.get(&update[n]) {
                for m in update.iter().skip(n) {
                    if must_be_before.contains(m) {
                        invalids.push(update);
                        continue 'outer;
                    }
                }
            }
        }
    }

    let mut total = 0;
    for invalid in invalids {
        let mut inv = invalid.clone();
        inv.sort_by(|a, b| {
            if let Some(must_be_before) = input.ordering.get(b) {
                if must_be_before.contains(a) {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });
        total += inv[inv.len() / 2]
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(143, part1(&input_generator(INPUT)))
    }

    #[test]
    fn test_part2() {
        assert_eq!(123, part2(&input_generator(INPUT)))
    }
}
