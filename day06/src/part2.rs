use std::collections::HashMap;

use crate::part1::parse_input;

fn least_used_char(counts: HashMap<char, usize>) -> char {
    counts
        .into_iter()
        .min_by_key(|&(_, count)| count)
        .map(|(ch, _)| ch)
        .unwrap()
}

pub fn solve_part_2(input: &str) -> String {
    let mut version = String::new();
    let columns = parse_input(input);

    for column in columns {
        let mut counts = HashMap::new();

        for letter in column {
            *counts.entry(letter).or_insert(0) += 1;
        }

        version.push(least_used_char(counts))
    }

    version
}
