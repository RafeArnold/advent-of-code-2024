use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    run(input, calculate_score)
}

fn run_2(input: &str) -> usize {
    run(input, calculate_rating)
}

fn run(input: &str, calculate: fn((usize, usize), &[(usize, usize)], &[Vec<u8>]) -> usize) -> usize {
    let map = input
        .lines()
        .map(|line| line.bytes().map(|byte| byte - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let trail_ends = map
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, byte)| {
                if *byte == 9 {
                    Some((col_idx, row_idx))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, byte) in row.iter().enumerate() {
            if *byte == 0 {
                sum += calculate((col_idx, row_idx), &trail_ends, &map);
            }
        }
    }
    sum
}

fn calculate_score(
    trail_head_pos: (usize, usize),
    trail_ends: &[(usize, usize)],
    map: &[Vec<u8>],
) -> usize {
    trail_ends
        .iter()
        .filter(|&&end| route_exists(trail_head_pos, end, map))
        .count()
}

fn route_exists(start: (usize, usize), end: (usize, usize), map: &[Vec<u8>]) -> bool {
    let boundary = ((map[0].len() - 1) as isize, (map.len() - 1) as isize);
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(State::new(start, 0)));
    loop {
        let Some(Reverse(State { pos, travelled, .. })) = queue.pop() else {
            return false;
        };
        if pos == end {
            return true;
        }
        for to_move in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let next_pos = (pos.0 as isize + to_move.0, pos.1 as isize + to_move.1);
            if next_pos.0 < 0
                || next_pos.1 < 0
                || next_pos.0 > boundary.0
                || next_pos.1 > boundary.1
            {
                continue;
            }
            let next_pos = (next_pos.0 as usize, next_pos.1 as usize);
            if map[next_pos.1][next_pos.0] == map[pos.1][pos.0] + 1 {
                queue.push(Reverse(State::new(next_pos, travelled + 1)));
            }
        }
    }
}

fn calculate_rating(
    trail_head_pos: (usize, usize),
    trail_ends: &[(usize, usize)],
    map: &[Vec<u8>],
) -> usize {
    let boundary = ((map[0].len() - 1) as isize, (map.len() - 1) as isize);
    trail_ends
        .iter()
        .map(|&end| count_routes(trail_head_pos, end, map, boundary))
        .sum()
}

fn count_routes(pos: (usize, usize), end: (usize, usize), map: &[Vec<u8>], boundary: (isize, isize)) -> usize {
    if pos == end {
        return 1;
    }
    let mut count = 0;
    for to_move in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let next_pos = (pos.0 as isize + to_move.0, pos.1 as isize + to_move.1);
        if next_pos.0 < 0
            || next_pos.1 < 0
            || next_pos.0 > boundary.0
            || next_pos.1 > boundary.1
        {
            continue;
        }
        let next_pos = (next_pos.0 as usize, next_pos.1 as usize);
        if map[next_pos.1][next_pos.0] == map[pos.1][pos.0] + 1 {
            count += count_routes(next_pos, end, map, boundary);
        }
    }
    count
}

struct State {
    travelled: usize,
    pos: (usize, usize),
}

impl State {
    fn new(pos: (usize, usize), travelled: usize) -> Self {
        Self { pos, travelled }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.travelled.cmp(&(other.travelled))
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for State {}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.travelled.eq(&other.travelled)
    }
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
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 36);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 81);
    }
}
