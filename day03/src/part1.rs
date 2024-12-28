pub fn parse_input(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().expect("Invalid number"));

            (
                numbers.next().expect("Missing first number"),
                numbers.next().expect("Missing second number"),
                numbers.next().expect("Missing third number"),
            )
        })
        .collect()
}

pub fn solve_part_1(input: &str) -> i32 {
    let mut valids = 0;
    let parsed = parse_input(input);

    // I dont know how triangles work.
    for (side1, side2, side3) in parsed {
        if (side1 + side2) > side3 {
            if (side2 + side3) > side1 {
                if (side3 + side1) > side2 {
                    valids += 1;
                }
            }
        }
    }

    return valids;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        const TEST_INPUT: &str = r#"5 10 25
5 30 25
15 11 25"#;

        let result = solve_part_1(TEST_INPUT);
        assert_eq!(result, 2);
    }
}
