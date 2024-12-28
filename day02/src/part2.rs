use crate::part1::{parse_input, Direction};

pub fn solve_part_2(input: &str) -> String {
    let mut keycode = String::new();
    let parsed = parse_input(input);
    let mut current = (1, 3);

    let keylock = vec![
        vec!["666", "666", "666", "666", "666", "666", "666"],
        vec!["666", "666", "666", "1", "666", "666", "666"],
        vec!["666", "666", "2", "3", "4", "666", "666"],
        vec!["666", "5", "6", "7", "8", "9", "666"],
        vec!["666", "666", "A", "B", "C", "666", "666"],
        vec!["666", "666", "666", "D", "666", "666", "666"],
        vec!["666", "666", "666", "666", "666", "666", "666"],
    ];

    for instructions in parsed {
        for direction in instructions {
            let new_pos = match direction {
                Direction::Up => (current.0, current.1 - 1),
                Direction::Down => (current.0, current.1 + 1),
                Direction::Left => (current.0 - 1, current.1),
                Direction::Right => (current.0 + 1, current.1),
            };

            if keylock[new_pos.1][new_pos.0] != "666" {
                current = new_pos;
            }
        }

        keycode.push_str(&keylock[current.1][current.0])
    }

    return keycode;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_2() {
        const TEST_INPUT: &str = r#"ULL
RRDDD
LURDL
UUUUD"#;

        let result = solve_part_2(TEST_INPUT);
        assert_eq!(result, "5DB3");
    }
}
