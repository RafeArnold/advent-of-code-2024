use std::iter::repeat_n;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let mut line = parse_input(input);
    let mut next_to_move = line.len() - 1;
    let mut next_space = 0;
    while next_to_move > next_space {
        if line[next_to_move].is_none() {
            next_to_move -= 1;
            continue;
        }
        if line[next_space].is_some() {
            next_space += 1;
            continue;
        }
        line.swap(next_to_move, next_space);
    }
    check_sum(line)
}

fn check_sum(line: Vec<Option<usize>>) -> usize {
    line.into_iter()
        .enumerate()
        .map(|(i, x)| x.map(|x| x * i).unwrap_or(0))
        .sum()
}

fn parse_input(input: &str) -> Vec<Option<usize>> {
    input
        .bytes()
        .enumerate()
        .flat_map(|(i, byte)| {
            let len = byte - b'0';
            if (i % 2) == 0 {
                repeat_n(Some(i / 2), len as usize)
            } else {
                repeat_n(None, len as usize)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 1928);
    }
}
