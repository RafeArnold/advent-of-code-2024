fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];

const ROTATIONS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn run_1(input: &str) -> usize {
    let lines = input.lines().map(|str| str.as_bytes()).collect::<Vec<_>>();
    let mut sum = 0;
    for (row, &line) in lines.iter().enumerate() {
        for (col, &byte) in line.iter().enumerate() {
            if byte == b'X' {
                'rotate: for rotation in ROTATIONS {
                    for i in 1..4 {
                        let row = i * rotation.1 + row as isize;
                        let col = i * rotation.0 + col as isize;
                        if row < 0 || col < 0 {
                            continue 'rotate;
                        }
                        let byte = lines
                            .get(row as usize)
                            .and_then(|line| line.get(col as usize));
                        if byte.is_none_or(|byte| *byte != XMAS[i as usize]) {
                            continue 'rotate;
                        }
                    }
                    sum += 1;
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_1() {
        const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(run_1(INPUT), 18);
    }
}
