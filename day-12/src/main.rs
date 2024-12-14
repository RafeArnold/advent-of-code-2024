use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let boundary = (map[0].len(), map.len());
    let mut sum = 0;
    let mut visited = HashSet::new();
    for (row_idx, row) in map.iter().enumerate() {
        for col_idx in 0..row.len() {
            if !visited.contains(&(col_idx, row_idx)) {
                let (region, region_perimeter) = scan_region((col_idx, row_idx), &map, boundary);
                sum += region.len() * region_perimeter;
                visited.extend(region);
            }
        }
    }
    sum
}

fn scan_region(
    start: (usize, usize),
    map: &[&[u8]],
    boundary: (usize, usize),
) -> (HashSet<(usize, usize)>, usize) {
    let desired_plant_type = map[start.1][start.0];
    let mut region = HashSet::new();
    let mut perimeter = 0;
    let mut unvisited = Vec::new();
    unvisited.push(start);
    while let Some(pos) = unvisited.pop() {
        if region.contains(&pos) {
            continue;
        }
        let plant_type = map[pos.1][pos.0];
        if plant_type != desired_plant_type {
            perimeter += 1;
        } else {
            region.insert(pos);
            if pos.0 + 1 < boundary.0 {
                unvisited.push((pos.0 + 1, pos.1));
            } else {
                perimeter += 1;
            }
            if pos.0 > 0 {
                unvisited.push((pos.0 - 1, pos.1));
            } else {
                perimeter += 1;
            }
            if pos.1 + 1 < boundary.0 {
                unvisited.push((pos.0, pos.1 + 1));
            } else {
                perimeter += 1;
            }
            if pos.1 > 0 {
                unvisited.push((pos.0, pos.1 - 1));
            } else {
                perimeter += 1;
            }
        }
    }
    (region, perimeter)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "AAAA
BBCD
BBCC
EEEC";

    const INPUT_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const INPUT_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT_1), 140);
        assert_eq!(run_1(INPUT_2), 772);
        assert_eq!(run_1(INPUT_3), 1930);
    }
}
