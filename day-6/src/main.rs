use std::collections::{HashMap, HashSet};

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    let ParseResult {
        mut pos,
        mut rot,
        obstacles,
        size,
    } = parse_input(input);
    let mut positions = HashSet::new();
    while pos.0 > 0 && pos.1 > 0 && pos.0 < size.0 - 1 && pos.1 < size.1 - 1 {
        positions.insert(pos);
        try_move(&mut pos, &mut rot, &obstacles);
    }
    positions.len() + 1
}

fn run_2(input: &str) -> usize {
    let ParseResult {
        mut pos,
        mut rot,
        obstacles,
        size,
    } = parse_input(input);
    let init_pos = pos;
    let init_rot = rot;
    let mut loop_count = 0;
    for (row, line) in input.lines().enumerate() {
        for col in 0..line.len() {
            if obstacles.contains(&(row, col)) || init_pos == (row, col) {
                continue;
            }
            let mut obstacles = obstacles.clone();
            obstacles.insert((row, col));
            pos = init_pos;
            rot = init_rot;
            let mut route = HashMap::new();
            while pos.0 > 0 && pos.1 > 0 && pos.0 < size.0 - 1 && pos.1 < size.1 - 1 {
                let rots = route.entry(pos).or_insert_with(Vec::new);
                if rots.contains(&rot) {
                    loop_count += 1;
                    break;
                }
                rots.push(rot);
                try_move(&mut pos, &mut rot, &obstacles);
            }
        }
    }
    loop_count
}

fn try_move(
    pos: &mut (usize, usize),
    rot: &mut (isize, isize),
    obstacles: &HashSet<(usize, usize)>,
) {
    let next_pos = (
        (pos.0 as isize + rot.0) as usize,
        (pos.1 as isize + rot.1) as usize,
    );
    if obstacles.contains(&next_pos) {
        *rot = (rot.1, -rot.0);
    } else {
        *pos = next_pos;
    }
}

fn parse_input(input: &str) -> ParseResult {
    let mut pos = (0, 0);
    let mut rot = (0, 0);
    let mut obstacles = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, byte) in line.bytes().enumerate() {
            if byte == b'#' {
                obstacles.insert((row, col));
            } else if byte != b'.' {
                pos = (row, col);
                rot = match byte {
                    b'^' => (-1, 0),
                    b'>' => (0, 1),
                    b'v' => (1, 0),
                    b'<' => (0, -1),
                    _ => unreachable!(),
                }
            }
        }
    }
    ParseResult {
        pos,
        rot,
        obstacles,
        size: (input.lines().count(), input.lines().next().unwrap().len()),
    }
}

struct ParseResult {
    pos: (usize, usize),
    rot: (isize, isize),
    obstacles: HashSet<(usize, usize)>,
    size: (usize, usize),
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
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 41);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 6);
    }
}
