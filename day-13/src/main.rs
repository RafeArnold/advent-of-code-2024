use std::convert::identity;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> u64 {
    run(input, identity)
}

fn run_2(input: &str) -> u64 {
    run(input, convert)
}

fn run(input: &str, convert: fn(Machine) -> Machine) -> u64 {
    input
        .split("\n\n")
        .map(parse_machine)
        .map(convert)
        .filter_map(tokens_to_win)
        .sum()
}

fn tokens_to_win(machine: Machine) -> Option<u64> {
    // solve the simultaneous equation:
    // machine.a_button.0 + machine.b_button.0 = machine.prize_pos.0
    // machine.a_button.1 + machine.b_button.1 = machine.prize_pos.1
    let a =
        (machine.a_button.0 * machine.b_button.1).abs_diff(machine.a_button.1 * machine.b_button.0);
    let c = (machine.prize_pos.0 * machine.b_button.1)
        .abs_diff(machine.prize_pos.1 * machine.b_button.0);
    let (a_presses, a_rem) = div_mod(c, a);
    if a_rem == 0 {
        let (b_presses, b_rem) = div_mod(
            machine.prize_pos.0 - a_presses * machine.a_button.0,
            machine.b_button.0,
        );
        if b_rem == 0 {
            return Some(3 * a_presses + b_presses);
        }
    }
    None
}

fn div_mod(a: u64, b: u64) -> (u64, u64) {
    (a / b, a % b)
}

fn convert(machine: Machine) -> Machine {
    Machine {
        a_button: machine.a_button,
        b_button: machine.b_button,
        prize_pos: (
            10000000000000 + machine.prize_pos.0,
            10000000000000 + machine.prize_pos.1,
        ),
    }
}

fn parse_machine(input: &str) -> Machine {
    let mut lines = input.lines();
    let a_button = parse_button(lines.next().unwrap());
    let b_button = parse_button(lines.next().unwrap());
    let prize_pos = parse_prize(lines.next().unwrap());
    Machine {
        a_button,
        b_button,
        prize_pos,
    }
}

fn parse_button(input: &str) -> (u64, u64) {
    parse(input, "Button A: X+".len())
}

fn parse_prize(input: &str) -> (u64, u64) {
    parse(input, "Prize: X=".len())
}

fn parse(input: &str, prefix_len: usize) -> (u64, u64) {
    let input = &input[prefix_len..];
    let (dx, input) = input.split_once(',').unwrap();
    let input = &input[3..];
    (dx.parse().unwrap(), input.parse().unwrap())
}

struct Machine {
    a_button: (u64, u64),
    b_button: (u64, u64),
    prize_pos: (u64, u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 480);
    }
}
