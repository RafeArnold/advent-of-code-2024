fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let mut stones = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for _ in 0..25 {
        for i in (0..stones.len()).rev() {
            let stone = stones[i];
            if stone == 0 {
                stones[i] = 1;
                continue;
            }
            let log10 = stone.ilog10();
            if log10 % 2 == 1 {
                let pow = 10usize.pow(log10 / 2 + 1);
                let upper = stone / pow;
                stones[i] = upper;
                stones.insert(i + 1, stone - upper * pow);
            } else {
                stones[i] *= 2024;
            }
        }
    }
    stones.len()
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
