fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
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

fn run_2(input: &str) -> usize {
    let lines = input.lines().map(|str| str.as_bytes()).collect::<Vec<_>>();
    let mut sum = 0;
    for (row, &line) in lines.iter().enumerate() {
        for (col, &byte) in line.iter().enumerate() {
            sum += is_x_mas(&lines, row, col, byte).is_some() as usize;
        }
    }
    sum
}

fn is_x_mas(lines: &[&[u8]], row: usize, col: usize, byte: u8) -> Option<()> {
    if byte != b'A' {
        return None;
    }
    {
        let point1 = (row.checked_sub(1)?, col.checked_sub(1)?);
        let point2 = (row + 1, col + 1);
        if point2.0 >= lines.len() || point2.1 >= lines[0].len() || !is_mas(lines, point1, point2) {
            return None;
        }
    }
    {
        let point1 = (row.checked_sub(1)?, col + 1);
        let point2 = (row + 1, col.checked_sub(1)?);
        if point2.0 >= lines.len() || point1.1 >= lines[0].len() || !is_mas(lines, point1, point2) {
            return None;
        }
    }
    Some(())
}

fn is_mas(lines: &[&[u8]], point1: (usize, usize), point2: (usize, usize)) -> bool {
    let byte1 = lines[point1.0][point1.1];
    let byte2 = lines[point2.0][point2.1];
    (byte1 == b'M' && byte2 == b'S') || (byte1 == b'S' && byte2 == b'M')
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

    #[test]
    fn challenge_2() {
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
        assert_eq!(run_2(INPUT), 9);
    }
}
