use std::collections::{HashMap, HashSet};

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    run(input, false)
}

fn run_2(input: &str) -> usize {
    run(input, true)
}

fn run(input: &str, count_all: bool) -> usize {
    let size = (
        input.lines().count() as isize,
        input.lines().next().unwrap().len() as isize,
    );
    let mut antennas: HashMap<u8, Vec<(isize, isize)>> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, byte) in line.bytes().enumerate() {
            if byte != b'.' {
                antennas
                    .entry(byte)
                    .or_default()
                    .push((row as isize, col as isize));
            }
        }
    }
    antennas
        .into_values()
        .flat_map(|antennas| get_anti_nodes(antennas, size, count_all))
        .filter(|&(x, y)| x < size.1 && y < size.0)
        .collect::<HashSet<_>>()
        .len()
}

fn get_anti_nodes(
    antennas: Vec<(isize, isize)>,
    boundary: (isize, isize),
    count_all: bool,
) -> HashSet<(isize, isize)> {
    let mut anti_nodes = HashSet::new();
    for (i, &(row_a, col_a)) in antennas.iter().enumerate() {
        for &(row_b, col_b) in antennas.iter().skip(i + 1) {
            let (y, x) = (row_b - row_a, col_b - col_a);
            if count_all {
                scan(boundary, &mut anti_nodes, (-y, -x), (row_a, col_a));
                scan(boundary, &mut anti_nodes, (y, x), (row_b, col_b));
            } else {
                if col_b >= -x {
                    anti_nodes.insert((row_b + y, col_b + x));
                }
                if row_a >= y && col_a >= x {
                    anti_nodes.insert((row_a - y, col_a - x));
                }
            }
        }
    }
    anti_nodes
}

fn scan(
    boundary: (isize, isize),
    anti_nodes: &mut HashSet<(isize, isize)>,
    to_move: (isize, isize),
    mut pos: (isize, isize),
) {
    while pos.0 >= 0 && pos.1 >= 0 && pos.0 < boundary.0 && pos.1 < boundary.1 {
        anti_nodes.insert((pos.0, pos.1));
        pos = (pos.0 + to_move.0, pos.1 + to_move.1);
    }
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
............";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 14);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 34);
    }
}
