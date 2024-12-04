use std::iter::Peekable;
use std::str::Chars;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    let mut chars = input.chars().peekable();
    let mut sum = 0;
    loop {
        let c = chars.peek();
        match c {
            Some('m') => match read_mul(&mut chars) {
                Some((first, second)) => sum += first * second,
                None => continue,
            },
            Some(_) => {
                chars.next();
            }
            None => return sum,
        }
    }
}

fn run_2(input: &str) -> usize {
    let mut chars = input.chars().peekable();
    let mut sum = 0;
    let mut ignore = false;
    loop {
        match chars.peek() {
            Some('m') if !ignore => match read_mul(&mut chars) {
                Some((first, second)) => sum += first * second,
                None => continue,
            },
            Some('d') => match read_do_or_dont(&mut chars) {
                Some(is_do) => ignore = !is_do,
                None => continue,
            },
            Some(_) => {
                chars.next();
            }
            None => return sum,
        }
    }
}

fn read_do_or_dont(chars: &mut Peekable<Chars>) -> Option<bool> {
    expect_char(chars, 'd')?;
    expect_char(chars, 'o')?;
    match chars.peek()? {
        '(' => {
            chars.next();
            expect_char(chars, ')')?;
            Some(true)
        },
        'n' => {
            chars.next();
            expect_char(chars, '\'')?;
            expect_char(chars, 't')?;
            expect_char(chars, '(')?;
            expect_char(chars, ')')?;
            Some(false)
        },
        _ => None,
    }
}

fn read_mul(chars: &mut Peekable<Chars>) -> Option<(usize, usize)> {
    expect_char(chars, 'm')?;
    expect_char(chars, 'u')?;
    expect_char(chars, 'l')?;
    expect_char(chars, '(')?;
    let first = expect_num(chars)?;
    expect_char(chars, ',')?;
    let second = expect_num(chars)?;
    expect_char(chars, ')')?;
    Some((first, second))
}

fn expect_num(chars: &mut Peekable<Chars>) -> Option<usize> {
    let mut digits = Vec::with_capacity(3);
    for _ in 0..3 {
        let c = *chars.peek()?;
        if matches!(c, '0'..='9') {
            chars.next();
            digits.push(c);
        } else {
            break;
        }
    }
    if digits.is_empty() {
        None
    } else {
        Some(parse_num(digits))
    }
}

fn parse_num(digits: Vec<char>) -> usize {
    digits.into_iter().collect::<String>().parse().unwrap()
}

fn expect_char(chars: &mut Peekable<Chars>, expected: char) -> Option<char> {
    let c = *chars.peek()?;
    if c == expected {
        chars.next();
        Some(c)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_1() {
        const INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(run_1(INPUT), 161);
    }

    #[test]
    fn challenge_2() {
        const INPUT: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(run_2(INPUT), 48);
    }

    #[test]
    fn valid_mul_intercepting_invalid_mul() {
        const INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(run_1(INPUT), 161);
    }

    #[test]
    fn mul_with_no_digit() {
        const INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(run_1(INPUT), 161);
    }

    #[test]
    fn eof_mid_number() {
        const INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)mul(8";
        assert_eq!(run_1(INPUT), 161);
    }
}
