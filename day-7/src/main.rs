fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .filter(|(test_value, operands)| could_be_true(*test_value, operands))
        .map(|equation| equation.0)
        .sum()
}

fn could_be_true(test_value: u64, operands: &[u64]) -> bool {
    if operands.len() == 1 {
        return test_value == operands[0];
    }
    let (rest, tail) = operands.split_at(operands.len() - 1);
    let tail = tail[0];
    test_value > tail && could_be_true(test_value - tail, rest)
        || test_value % tail == 0 && could_be_true(test_value / tail, rest)
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let (test_value, operands) = line.split_once(": ").unwrap();
    let operands = operands.split(' ').map(|operand| operand.parse().unwrap());
    (test_value.parse().unwrap(), operands.collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 3749);
    }
}
