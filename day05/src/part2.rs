use md5;

fn is_valid_hash(hash: &str) -> bool {
    return hash.starts_with("00000");
}

pub fn solve_part_2(input: &str) -> String {
    let mut count = 0;
    let mut letters_found: Vec<char> = vec!['-', '-', '-', '-', '-', '-', '-', '-'];

    while letters_found.contains(&'-') {
        let secret = format!("{}{}", input, count);
        let hash = format!("{:x}", md5::compute(secret));

        if is_valid_hash(&hash) {
            let chars: Vec<char> = hash.chars().collect();
            let index = chars[5] as usize - '0' as usize;

            if index < 8 && letters_found[index] == '-' {
                letters_found[index] = chars[6];
            }
        }

        count += 1;
    }

    let password: String = letters_found.iter().collect();

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_2() {
        const TEST_INPUT: &str = "abc";

        let result = solve_part_2(TEST_INPUT);
        assert_eq!(result, "05ace8e3");
    }
}
