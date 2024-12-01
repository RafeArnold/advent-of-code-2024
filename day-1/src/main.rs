fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> u32 {
    let (mut col1, mut col2): (Vec<_>, Vec<_>) = input.lines().map(parse_line).unzip();
    quick_sort(&mut col1);
    quick_sort(&mut col2);
    col1.iter().zip(col2).map(|(a, b)| a.abs_diff(b)).sum()
}

fn parse_line(line: &str) -> (u32, u32) {
    let pos = line.chars().position(|c| c == ' ').unwrap();
    (
        line[0..pos].parse().unwrap(),
        line[pos + 3..].parse().unwrap(),
    )
}

fn quick_sort(slice: &mut [u32]) {
    if slice.len() <= 1 {
        return;
    }
    let p = partition(slice);
    quick_sort(&mut slice[..p]);
    quick_sort(&mut slice[p + 1..]);
}

fn partition(slice: &mut [u32]) -> usize {
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
}
