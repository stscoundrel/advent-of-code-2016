use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    let length = lines[0].len();
    let mut columns: Vec<Vec<char>> = vec![Vec::new(); length];

    for line in &lines {
        for (column, letter) in line.chars().enumerate() {
            columns[column].push(letter);
        }
    }

    columns
}

fn most_used_char(counts: HashMap<char, usize>) -> char {
    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(ch, _)| ch)
        .unwrap()
}

pub fn solve_part_1(input: &str) -> String {
    let mut version = String::new();
    let columns = parse_input(input);

    for column in columns {
        let mut counts = HashMap::new();

        for letter in column {
            *counts.entry(letter).or_insert(0) += 1;
        }

        version.push(most_used_char(counts))
    }

    version
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        const TEST_INPUT: &str = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#;

        let result = solve_part_1(TEST_INPUT);
        assert_eq!(result, "easter");
    }
}
