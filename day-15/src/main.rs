fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let (mut map, moves, mut robot_pos) = parse_input(input);
    for next_move in moves {
        match next_move {
            Move::Up => perform_move(&mut map, &mut robot_pos, |pos| pos.1 -= 1),
            Move::Left => perform_move(&mut map, &mut robot_pos, |pos| pos.0 -= 1),
            Move::Down => perform_move(&mut map, &mut robot_pos, |pos| pos.1 += 1),
            Move::Right => perform_move(&mut map, &mut robot_pos, |pos| pos.0 += 1),
        }
    }
    map.into_iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.into_iter()
                .enumerate()
                .map(|(col_idx, tile)| {
                    if matches!(tile, Tile::Box) {
                        row_idx * 100 + col_idx
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn perform_move<F>(map: &mut [Vec<Tile>], robot_pos: &mut (usize, usize), mut move_pos: F)
where
    F: FnMut(&mut (usize, usize)),
{
    let mut first_pos = *robot_pos;
    move_pos(&mut first_pos);
    let mut pos = first_pos;
    while matches!(map[pos.1][pos.0], Tile::Box) {
        move_pos(&mut pos);
    }
    if matches!(map[pos.1][pos.0], Tile::Empty) {
        map[pos.1][pos.0] = Tile::Box;
        map[first_pos.1][first_pos.0] = Tile::Empty;
        move_pos(robot_pos);
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, Vec<Move>, (usize, usize)) {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let robot_pos = map
        .lines()
        .enumerate()
        .find_map(|(row, line)| line.bytes().position(|b| b == b'@').map(|col| (col, row)))
        .unwrap();
    let map = map.lines().map(parse_map_line).collect();
    let moves = moves.lines().flat_map(parse_moves).collect();
    (map, moves, robot_pos)
}

fn parse_map_line(line: &str) -> Vec<Tile> {
    line.bytes().map(parse_tile).collect()
}

fn parse_tile(byte: u8) -> Tile {
    match byte {
        b'#' => Tile::Wall,
        b'O' => Tile::Box,
        _ => Tile::Empty,
    }
}

fn parse_moves(moves: &str) -> Vec<Move> {
    moves.bytes().map(parse_move).collect()
}

fn parse_move(byte: u8) -> Move {
    match byte {
        b'^' => Move::Up,
        b'>' => Move::Right,
        b'v' => Move::Down,
        _ => Move::Left,
    }
}

enum Move {
    Up,
    Left,
    Down,
    Right,
}

enum Tile {
    Wall,
    Box,
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const INPUT_2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT_1), 2028);
        assert_eq!(run_1(INPUT_2), 10092);
    }
}
