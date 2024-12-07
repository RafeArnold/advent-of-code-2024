fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    let (mut col1, mut col2) = parse_lines(input);
    quick_sort(&mut col1);
    quick_sort(&mut col2);
    col1.iter().zip(col2).map(|(a, b)| a.abs_diff(b)).sum()
}

fn run_2(input: &str) -> usize {
    let (col1, col2) = parse_lines(input);
    col1.iter()
        .map(|a| col2.iter().filter(|b| a == *b).count() * a)
        .sum()
}

fn parse_lines(input: &str) -> (Vec<usize>, Vec<usize>) {
    input.lines().map(parse_line).unzip()
}

fn parse_line(line: &str) -> (usize, usize) {
    let pos = line.chars().position(|c| c == ' ').unwrap();
    (
        line[0..pos].parse().unwrap(),
        line[pos + 3..].parse().unwrap(),
    )
}

fn quick_sort(slice: &mut [usize]) {
    if slice.len() <= 1 {
        return;
    }
    let p = partition(slice);
    quick_sort(&mut slice[..p]);
    quick_sort(&mut slice[p + 1..]);
}

fn partition(slice: &mut [usize]) -> usize {
    let pivot = slice[slice.len() - 1];
    let mut i = 0;
    for j in 0..slice.len() - 1 {
        if slice[j] <= pivot {
            slice.swap(i, j);
            i += 1;
        }
    }
    slice.swap(i, slice.len() - 1);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_test() {
        let mut slice = [3, 4, 2, 5, 1];
        quick_sort(&mut slice);
        assert_eq!([1, 2, 3, 4, 5], slice);
    }

    #[test]
    fn challenge_1() {
        const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(run_1(INPUT), 11);
    }

    #[test]
    fn challenge_2() {
        const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(run_2(INPUT), 31);
    }
}
