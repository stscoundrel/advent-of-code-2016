#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum Facing {
    North,
    East,
    South,
    West,
}

pub fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    input
        .split(", ")
        .map(|part| {
            let (dir, dist) = part.split_at(1);
            let distance = dist.parse::<i32>().unwrap();
            let direction = match dir {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Roskaa"),
            };
            (direction, distance)
        })
        .collect()
}

pub fn solve_part_1(input: &str) -> i32 {
    let parsed = parse_input(input);

    let mut facing = Facing::North;
    let mut x = 0;
    let mut y = 0;

    for (direction, distance) in parsed {
        match facing {
            Facing::North => match direction {
                Direction::Left => {
                    x -= distance;
                    facing = Facing::West
                }
                Direction::Right => {
                    x += distance;
                    facing = Facing::East
                }
            },
            Facing::East => match direction {
                Direction::Left => {
                    y -= distance;
                    facing = Facing::North
                }
                Direction::Right => {
                    y += distance;
                    facing = Facing::South
                }
            },
            Facing::South => match direction {
                Direction::Left => {
                    x += distance;
                    facing = Facing::East
                }
                Direction::Right => {
                    x -= distance;
                    facing = Facing::West
                }
            },
            Facing::West => match direction {
                Direction::Left => {
                    y += distance;
                    facing = Facing::South
                }
                Direction::Right => {
                    y -= distance;
                    facing = Facing::North
                }
            },
        }
    }

    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        const TEST_INPUT_1: &str = "R2, L3";
        const TEST_INPUT_2: &str = "R2, R2, R2";
        const TEST_INPUT_3: &str = "R5, L5, R5, R3";

        let result1 = solve_part_1(TEST_INPUT_1);
        assert_eq!(result1, 5);

        let result2 = solve_part_1(TEST_INPUT_2);
        assert_eq!(result2, 2);

        let result3 = solve_part_1(TEST_INPUT_3);
        assert_eq!(result3, 12);
    }
}
