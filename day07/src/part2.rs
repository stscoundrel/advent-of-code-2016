use regex::Regex;

#[derive(Debug)]
pub struct IpAddress {
    pub sequences: Vec<String>,
    pub hypernets: Vec<String>,
}

impl IpAddress {
    pub fn supports_sls(&self) -> bool {
        for hypernet in &self.hypernets {
            let abas = get_abas(&hypernet);

            let sequence_abas = &self
                .sequences
                .iter()
                .flat_map(|s| get_abas(&s))
                .collect::<Vec<String>>();

            for aba in abas {
                let chars: Vec<char> = aba.chars().collect();
                let opposite_aba = format!("{}{}{}", chars[1], chars[0], chars[1]);

                for sequence_aba in sequence_abas {
                    if sequence_aba.as_str() == opposite_aba {
                        return true;
                    }
                }
            }
        }

        false
    }
}

pub fn get_abas(input: &str) -> Vec<String> {
    // Regex crate does not seem to support most of the useful operations,
    // so here we are looping like monkeys we are.
    let mut abas = Vec::new();

    for i in 0..input.len() - 2 {
        let first = input.chars().nth(i).unwrap();
        let second = input.chars().nth(i + 1).unwrap();
        let third = input.chars().nth(i + 2).unwrap();

        if first == third && first != second {
            abas.push(format!("{}{}{}", first, second, third));
        }
    }

    abas
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

pub fn solve_part_2(input: &str) -> i64 {
    let mut valids = 0;
    let ip_addresses = parse_input(input);

    for ip_address in ip_addresses {
        if ip_address.supports_sls() {
            valids += 1;
        }
    }

    valids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_2() {
        const TEST_INPUT: &str = r#"aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb"#;

        let result = solve_part_2(TEST_INPUT);
        assert_eq!(result, 3);
    }
}
