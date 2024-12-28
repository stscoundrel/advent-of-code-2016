use std::collections::HashSet;

use crate::part1::{parse_input, Direction, Facing};

fn get_steps_between(start: i32, end: i32) -> Vec<i32> {
    if start < end {
        (start + 1..=end).collect()
    } else {
        (end..=start - 1).rev().collect()
    }
}

pub fn solve_part_2(input: &str) -> i32 {
    let parsed = parse_input(input);
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let mut facing = Facing::North;
    let mut x = 0;
    let mut y = 0;

    for (direction, distance) in parsed {
        let mut steps: HashSet<(i32, i32)> = HashSet::new();

        /**
         * Ugliest repetition I've seen since Trumps second term.
         */
        match facing {
            Facing::North => match direction {
                Direction::Left => {
                    for step in get_steps_between(x, x - distance) {
                        steps.insert((step, y));
                    }
                    x -= distance;
                    facing = Facing::West;
                }
                Direction::Right => {
                    for step in get_steps_between(x, x + distance) {
                        steps.insert((step, y));
                    }
                    x += distance;
                    facing = Facing::East;
                }
            },
            Facing::East => match direction {
                Direction::Left => {
                    for step in get_steps_between(y, y - distance) {
                        steps.insert((x, step));
                    }
                    y -= distance;
                    facing = Facing::North;
                }
                Direction::Right => {
                    for step in get_steps_between(y, y + distance) {
                        steps.insert((x, step));
                    }
                    y += distance;
                    facing = Facing::South;
                }
            },
            Facing::South => match direction {
                Direction::Left => {
                    for step in get_steps_between(x, x + distance) {
                        steps.insert((step, y));
                    }
                    x += distance;
                    facing = Facing::East;
                }
                Direction::Right => {
                    for step in get_steps_between(x, x - distance) {
                        steps.insert((step, y));
                    }
                    x -= distance;
                    facing = Facing::West;
                }
            },
            Facing::West => match direction {
                Direction::Left => {
                    for step in get_steps_between(y, y + distance) {
                        steps.insert((x, step));
                    }
                    y += distance;
                    facing = Facing::South;
                }
                Direction::Right => {
                    for step in get_steps_between(y, y - distance) {
                        steps.insert((x, step));
                    }
                    y -= distance;
                    facing = Facing::North;
                }
            },
        }

        for step in steps {
            if visited.contains(&step) {
                return step.0.abs() + step.1.abs();
            }

            visited.insert(step);
        }
    }

    panic!("Not found :(")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_2() {
        const TEST_INPUT: &str = "R8, R4, R4, R8";

        let result = solve_part_2(TEST_INPUT);
        assert_eq!(result, 4);
    }
}
