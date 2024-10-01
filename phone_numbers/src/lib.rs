pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn parse_line(line: &str) -> String {
    line.chars().filter(|c| c.is_digit(10)).collect::<String>()
}

fn build_database(input: &str) -> Vec<String> {
    let lines = input.split('\n');
    lines.map(|line| parse_line(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input_line = "o) Bob 91 12 54 26";
        let expected_result = "91125426";
        assert_eq!(parse_line(input_line), expected_result);
        assert_eq!(parse_line("Alice 97 625 992"), "97625992");
        assert_eq!(parse_line("Emergency 911"), "911");
    }

    #[test]
    fn test_build_database() {
        let input = "o) Bob 91 12 54 26\nAlice 97 625 992\nEmergency 911";
        let expected_result = vec![
            "91125426".to_string(),
            "97625992".to_string(),
            "911".to_string(),
        ];
        assert_eq!(build_database(input), expected_result);
    }
}
