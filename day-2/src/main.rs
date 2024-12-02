fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|record| check_record(record))
        .count()
}

fn run_2(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|record| check_record_tolerant(record))
        .count()
}

fn check_record_tolerant(record: &[u8]) -> bool {
    if check_record(record) {
        return true;
    }
    for i in 0..record.len() {
        if check_record(&[&record[0..i], &record[i + 1..record.len()]].concat()) {
            return true;
        }
    }
    false
}

fn check_record(record: &[u8]) -> bool {
    let mut record = record.iter();
    let first = record.next().unwrap();
    let second = record.next().unwrap();
    if first.abs_diff(*second) > 3 || first == second {
        return false;
    }
    let increasing = first < second;
    let mut last = second;
    for next in record {
        if next.abs_diff(*last) > 3 || increasing && next <= last || !increasing && next >= last {
            return false;
        }
        last = next;
    }
    true
}

fn parse_line(line: &str) -> Vec<u8> {
    line.split_whitespace()
        .map(|level| level.parse::<u8>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_1() {
        const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(run_1(INPUT), 2);
    }

    #[test]
    fn challenge_2() {
        const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(run_2(INPUT), 4);
    }
}
