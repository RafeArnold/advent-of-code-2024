use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let map = parse_input(input);
    let start = (1usize, map.len() - 2);
    let end = (map[1].len() - 2, 1);
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(Step {
        score: 0,
        distance_to_end: start.0.abs_diff(end.0) + start.1.abs_diff(end.1),
        pos: start,
        facing: (1isize, 0isize),
    }));
    let mut visited = HashSet::new();
    loop {
        let step = queue.pop().unwrap().0;
        if step.pos == end {
            return step.score;
        } else if visited.contains(&step.pos) {
            continue;
        }
        visited.insert(step.pos);
        if map[step.pos.1][step.pos.0] {
            continue;
        }
        for facing in DIRECTIONS {
            let pos = (
                (step.pos.0 as isize + facing.0) as usize,
                (step.pos.1 as isize + facing.1) as usize,
            );
            queue.push(Reverse(Step {
                score: step.score + if facing == step.facing { 1 } else { 1001 },
                distance_to_end: pos.0.abs_diff(end.0) + pos.1.abs_diff(end.1),
                pos,
                facing,
            }));
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.bytes().map(|byte| byte == b'#').collect())
        .collect()
}

struct Step {
    score: usize,
    distance_to_end: usize,
    pos: (usize, usize),
    facing: (isize, isize),
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.score + self.distance_to_end).cmp(&(other.score + other.distance_to_end))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Step {}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        (self.score + self.distance_to_end).eq(&(other.score + other.distance_to_end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    const INPUT_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT_1), 7036);
        assert_eq!(run_1(INPUT_2), 11048);
    }
}
