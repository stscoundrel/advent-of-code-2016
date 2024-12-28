use crate::part1::{is_real_room, parse_input, Room};

fn decrypt(room: &Room) -> String {
    room.encrypted
        .chars()
        .map(|c| {
            let cur_shift = c as u8 - 'a' as u8;
            let new_shift = ((cur_shift as u32 + room.id) % 26) as u8;
            ('a' as u8 + new_shift) as char
        })
        .collect()
}

pub fn solve_part_2(input: &str) -> u32 {
    let rooms = parse_input(input)
        .into_iter()
        .filter(|room| is_real_room(room))
        .map(|room| (room.id, decrypt(&room)));

    for room in rooms {
        println!("{} - {}", room.0, room.1)
    }

    return 666;
}
