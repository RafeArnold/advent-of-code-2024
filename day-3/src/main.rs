use std::str::Chars;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let mut chars = input.chars();
    let mut sum = 0;
    loop {
        match read_mul(&mut chars) {
            Ok((first, second)) => sum += first * second,
            Err(true) => return sum,
            Err(false) => continue,
        }
    }
}

macro_rules! expect_char_or {
    ($iter:ident, $pattern:pat, $or:ident $(,)?) => {{
        let c = next_char($iter)?;
        if (matches!(c, $pattern)) {
            Ok(Some(c))
        } else if c == $or {
            Ok(None)
        } else {
            Err(false)
        }
    }};
}

fn read_mul(chars: &mut Chars) -> Result<(usize, usize), bool> {
    expect_char(chars, 'm')?;
    expect_char(chars, 'u')?;
    expect_char(chars, 'l')?;
    expect_char(chars, '(')?;
    let first = expect_num(chars, ',')?;
    let second = expect_num(chars, ')')?;
    Ok((first, second))
}

fn expect_num(chars: &mut Chars, suffix: char) -> Result<usize, bool> {
    let mut digits = Vec::with_capacity(3);
    for _ in 0..3 {
        let c = expect_char_or!(chars, '0'..='9', suffix);
        if let Some(c) = c? {
            digits.push(c);
        } else {
            return Ok(parse_num(digits));
        }
    }
    expect_char(chars, suffix)?;
    Ok(parse_num(digits))
}

fn parse_num(digits: Vec<char>) -> usize {
    digits.into_iter().collect::<String>().parse().unwrap()
}

fn expect_char(chars: &mut Chars, expected: char) -> Result<char, bool> {
    let c = next_char(chars)?;
    if c == expected {
        Ok(c)
    } else {
        Err(false)
    }
}

fn next_char(chars: &mut Chars) -> Result<char, bool> {
    chars.next().ok_or(true)
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
}
