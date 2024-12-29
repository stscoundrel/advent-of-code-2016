use md5;

fn is_valid_hash(hash: &str) -> bool {
    return hash.starts_with("00000");
}

pub fn solve_part_1(input: &str) -> String {
    let mut password = String::new();
    let mut count = 0;

    while password.len() < 8 {
        let secret = format!("{}{}", input, count);
        let hash = format!("{:x}", md5::compute(secret));

        if is_valid_hash(&hash) {
            password.push(hash.chars().nth(5).unwrap())
        }

        count += 1;
    }

    return password;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        const TEST_INPUT: &str = "abc";

        let result = solve_part_1(TEST_INPUT);
        assert_eq!(result, "18f47a30");
    }
}
