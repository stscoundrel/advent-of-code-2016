#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    _ => panic!("Ei saatana"),
                })
                .collect()
        })
        .collect()
}

pub fn solve_part_1(input: &str) -> String {
    let mut keycode = String::new();
    let parsed = parse_input(input);
    let mut current = (2, 2);

    let keylock = vec![
        vec![666, 666, 666, 666, 666],
        vec![666, 1, 2, 3, 666],
        vec![666, 4, 5, 6, 666],
        vec![666, 7, 8, 9, 666],
        vec![666, 666, 666, 666, 666],
    ];

    for instructions in parsed {
        for direction in instructions {
            let new_pos = match direction {
                Direction::Up => (current.0, current.1 - 1),
                Direction::Down => (current.0, current.1 + 1),
                Direction::Left => (current.0 - 1, current.1),
                Direction::Right => (current.0 + 1, current.1),
            };

            if keylock[new_pos.1][new_pos.0] != 666 {
                current = new_pos;
            }
        }

        keycode.push_str(&keylock[current.1][current.0].to_string())
    }

    return keycode;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        const TEST_INPUT: &str = r#"ULL
RRDDD
LURDL
UUUUD"#;

        let result = solve_part_1(TEST_INPUT);
        assert_eq!(result, "1985");
    }
}
