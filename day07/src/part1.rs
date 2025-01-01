use regex::Regex;

#[derive(Debug)]
pub struct IpAddress {
    pub sequences: Vec<String>,
    pub hypernets: Vec<String>,
}

impl IpAddress {
    pub fn supports_tls(&self) -> bool {
        for hypernet in &self.hypernets {
            if has_abba(hypernet) {
                return false;
            }
        }

        for sequence in &self.sequences {
            if has_abba(sequence) {
                return true;
            }
        }

        false
    }
}

pub fn has_abba(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        if i + 1 < chars.len() {
            let current = (chars[i], chars[i + 1]);

            if i + 3 < chars.len() {
                let next = (chars[i + 3], chars[i + 2]);

                if current == next && chars[i] != chars[i + 1] {
                    return true;
                }
            }
        }
    }

    false
}

pub fn parse_input(input: &str) -> Vec<IpAddress> {
    let mut ip_addresses: Vec<IpAddress> = vec![];

    for line in input.lines() {
        let re = Regex::new(r"\[([^\]]+)\]|([^\[\]]+)").unwrap();

        let mut sequences = Vec::new();
        let mut hypernets = Vec::new();

        for cap in re.captures_iter(line) {
            if let Some(hypernet) = cap.get(1) {
                hypernets.push(hypernet.as_str().to_string());
            }
            if let Some(sequence) = cap.get(2) {
                sequences.push(sequence.as_str().to_string());
            }
        }

        ip_addresses.push(IpAddress {
            sequences,
            hypernets,
        });
    }

    ip_addresses
}

pub fn solve_part_1(input: &str) -> i64 {
    let mut valids = 0;
    let ip_addresses = parse_input(input);

    for ip_address in ip_addresses {
        if ip_address.supports_tls() {
            valids += 1;
        }
    }

    valids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        const TEST_INPUT: &str = r#"abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn"#;

        let result = solve_part_1(TEST_INPUT);
        assert_eq!(result, 2);
    }
}
