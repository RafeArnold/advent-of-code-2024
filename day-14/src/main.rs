use std::cmp::Ordering;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT, (101, 103)));
}

fn run_1(input: &str, boundary: (i64, i64)) -> i64 {
    input
        .lines()
        .map(parse_line)
        .map(|robot| move_robot(robot, boundary, 100))
        .fold([0; 4], |mut quadrants, robot| {
            if let Some(quadrant) = quadrant(robot, boundary) {
                quadrants[quadrant] += 1;
            }
            quadrants
        })
        .into_iter()
        .product()
}

fn quadrant(robot: Robot, boundary: (i64, i64)) -> Option<usize> {
    let x = match robot.position.0.cmp(&(boundary.0 / 2)) {
        Ordering::Less => 0,
        Ordering::Equal => return None,
        Ordering::Greater => 1,
    };
    let y = match robot.position.1.cmp(&(boundary.1 / 2)) {
        Ordering::Less => 0,
        Ordering::Equal => return None,
        Ordering::Greater => 1,
    };
    Some((x + 2 * y) as usize)
}

fn move_robot(mut robot: Robot, boundary: (i64, i64), i: i32) -> Robot {
    for _ in 0..i {
        robot.position.0 += robot.velocity.0;
        robot.position.0 = robot.position.0.rem_euclid(boundary.0);
        robot.position.1 += robot.velocity.1;
        robot.position.1 = robot.position.1.rem_euclid(boundary.1);
    }
    robot
}

fn parse_line(input: &str) -> Robot {
    let input = &input["p=".len()..];
    let (p1, input) = input.split_once(",").unwrap();
    let (p2, input) = input.split_once(" ").unwrap();
    let input = &input["v=".len()..];
    let (v1, v2) = input.split_once(",").unwrap();
    Robot {
        position: (p1.parse().unwrap(), p2.parse().unwrap()),
        velocity: (v1.parse().unwrap(), v2.parse().unwrap()),
    }
}

struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT, (11, 7)), 12);
    }
}
