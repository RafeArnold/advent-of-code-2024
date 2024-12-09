use std::collections::{HashMap, HashSet};

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let size = (input.lines().count(), input.lines().next().unwrap().len());
    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, byte) in line.bytes().enumerate() {
            if byte != b'.' {
                antennas.entry(byte).or_default().push((row, col));
            }
        }
    }
    antennas
        .into_values()
        .map(get_anti_nodes)
        .flatten()
        .filter(|&(x, y)| x < size.1 && y < size.0)
        .collect::<HashSet<_>>()
        .len()
}

fn get_anti_nodes(antennas: Vec<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut anti_nodes = HashSet::new();
    for (i, &(row_a, col_a)) in antennas.iter().enumerate() {
        for &(row_b, col_b) in antennas.iter().skip(i + 1) {
            // We're assuming row_b >= row_a
            let (y, x) = (row_b - row_a, col_b as isize - col_a as isize);
            if col_b as isize >= -x {
                anti_nodes.insert((row_b + y, (col_b as isize + x) as usize));
            }
            if row_a >= y && col_a as isize >= x {
                anti_nodes.insert((row_a - y, (col_a as isize - x) as usize));
            }
        }
    }
    anti_nodes
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
}
