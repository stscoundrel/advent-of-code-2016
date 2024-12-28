mod part1;

use std::fs::File;
use std::io::{self, Read};

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let input = read_file("input.txt").unwrap();
    let result1 = part1::solve_part_1(&input);

    print!("Part 1: {}", result1);
}
