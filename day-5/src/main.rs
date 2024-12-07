use std::cmp::Ordering::{Greater, Less};
use std::collections::BTreeSet;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    let (rules, updates) = parse_input(input);
    updates
        .into_iter()
        .filter(|update| is_sorted(&rules, update))
        .map(|update| update[update.len() / 2] as usize)
        .sum()
}

fn run_2(input: &str) -> usize {
    let (rules, updates) = parse_input(input);
    updates
        .into_iter()
        .filter(|update| !is_sorted(&rules, update))
        .map(|mut update| {
            update.sort_by(|&a, &b| {
                if rules.contains(&(a, b)) {
                    Less
                } else {
                    Greater
                }
            });
            update[update.len() / 2] as usize
        })
        .sum()
}

fn is_sorted(rules: &BTreeSet<(u8, u8)>, update: &[u8]) -> bool {
    update.is_sorted_by(|&a, &b| rules.contains(&(a, b)))
}

fn parse_input(input: &str) -> (BTreeSet<(u8, u8)>, Vec<Vec<u8>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().map(parse_rule).collect();
    let updates = updates.lines().map(parse_update).collect();
    (rules, updates)
}

fn parse_rule(rule: &str) -> (u8, u8) {
    let (lesser, greater) = rule.split_once('|').unwrap();
    (lesser.parse().unwrap(), greater.parse().unwrap())
}

fn parse_update(update: &str) -> Vec<u8> {
    update.split(',').map(|s| s.parse().unwrap()).collect()
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
97,13,75,29,47
";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 143);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 123);
    }
}
