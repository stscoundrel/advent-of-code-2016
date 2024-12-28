use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub struct Room {
    pub encrypted: String,
    pub id: u32,
    pub checksum: String,
}

pub fn parse_input(input: &str) -> Vec<Room> {
    let room_regex = Regex::new(r"^(.*)-(\d+)\[([a-z]+)\]$").unwrap();
    let mut rooms = Vec::new();

    for line in input.lines() {
        if let Some(captures) = room_regex.captures(line) {
            let encrypted = captures[1].replace("-", "");
            let id = captures[2].parse::<u32>().unwrap();
            let checksum = captures[3].to_string();

            rooms.push(Room {
                encrypted,
                id,
                checksum,
            });
        }
    }

    rooms
}

pub fn is_real_room(room: &Room) -> bool {
    let mut counts = HashMap::new();

    for letter in room.encrypted.chars() {
        *counts.entry(letter).or_insert(0) += 1;
    }

    let mut sorted_letters: Vec<(char, usize)> = counts.into_iter().collect();
    sorted_letters.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    for (index, letter) in room.checksum.chars().enumerate() {
        if sorted_letters[index].0 != letter {
            return false;
        }
    }

    return true;
}

pub fn solve_part_1(input: &str) -> u32 {
    return parse_input(input)
        .into_iter()
        .filter(|room| is_real_room(room))
        .map(|room| room.id)
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        const TEST_INPUT: &str = r#"aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]"#;

        let result = solve_part_1(TEST_INPUT);
        assert_eq!(result, 1514);
    }
}
