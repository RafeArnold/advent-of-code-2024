use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    run(input, 25)
}

fn run_2(input: &str) -> usize {
    run(input, 75)
}

fn run(input: &str, iterations: u8) -> usize {
    let mut stones = HashMap::new();
    for stone in input.split_whitespace() {
        *stones.entry(stone.parse::<usize>().unwrap()).or_default() += 1;
    }
    for _ in 0..iterations {
        let mut new_stones = HashMap::new();
        for (stone, count) in stones {
            for new_stone in next_stones(stone) {
                *new_stones.entry(new_stone).or_default() += count;
            }
        }
        stones = new_stones;
    }
    stones.values().sum()
}

fn next_stones(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    }
    let log10 = stone.ilog10();
    if log10 % 2 == 1 {
        let pow = 10usize.pow(log10 / 2 + 1);
        vec![stone / pow, stone % pow]
    } else {
        vec![stone * 2024]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 55312);
    }
}
