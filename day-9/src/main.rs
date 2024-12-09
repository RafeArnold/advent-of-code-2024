use std::cmp::Ordering;
use std::iter::repeat_n;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
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
    line.into_iter()
        .filter(|x| x.is_some())
        .enumerate()
        .map(|(i, x)| x.unwrap() * i)
        .sum()
}

fn run_2(input: &str) -> usize {
    let mut line = input
        .bytes()
        .enumerate()
        .map(|(i, byte)| {
            let kind = if (i % 2) == 0 {
                SpanKind::File { id: i / 2 }
            } else {
                SpanKind::Space
            };
            let len = (byte - b'0') as usize;
            Span { kind, len }
        })
        .collect::<Vec<_>>();
    let mut next_to_move_id = (line.len() - 1) / 2;
    loop {
        let next_to_move_idx = line
            .iter()
            .position(|span| matches!(span.kind, SpanKind::File { id } if id == next_to_move_id))
            .unwrap();
        let to_move = &line[next_to_move_idx];
        let mut space_idx = 0;
        while space_idx < next_to_move_idx {
            let space = &line[space_idx];
            if space.kind != SpanKind::Space {
                space_idx += 1;
                continue;
            }
            match to_move.len.cmp(&space.len) {
                Ordering::Less => {
                    let space_to_swap = Span {
                        kind: SpanKind::Space,
                        len: to_move.len,
                    };
                    line.insert(
                        space_idx + 1,
                        Span {
                            kind: SpanKind::Space,
                            len: space.len - to_move.len,
                        },
                    );
                    line[space_idx] = space_to_swap;
                    line.swap(space_idx, next_to_move_idx + 1);
                    break;
                }
                Ordering::Equal => {
                    line.swap(space_idx, next_to_move_idx);
                    break;
                }
                Ordering::Greater => {
                    space_idx += 1;
                }
            }
        }
        if next_to_move_id == 0 {
            break;
        }
        next_to_move_id -= 1;
    }
    let mut sum = 0;
    let mut i = 0;
    for span in line {
        match span.kind {
            SpanKind::Space => (),
            SpanKind::File { id } => sum += id * (i * span.len + (span.len - 1) * span.len / 2),
        }
        i += span.len;
    }
    sum
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

#[derive(Debug)]
struct Span {
    kind: SpanKind,
    len: usize,
}

#[derive(Debug, PartialEq)]
enum SpanKind {
    File { id: usize },
    Space,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 1928);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 2858);
    }
}
