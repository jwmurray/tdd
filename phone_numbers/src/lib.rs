use itertools::Itertools;

// Given a line with random characters, extract the digits and return those digits as a string.
fn parse_line(line: &str) -> String {
    line.chars().filter(|c| c.is_digit(10)).collect::<String>()
}

pub fn is_consistent(string_vector: &Vec<String>) -> bool {
    // return false if any database member is a substring of a subsequent member

    // Given a collection that is sorted and accessible by index,
    // start with the first element and check if it is a substring of any subsequent element.
    // Then move to the next element and repeat the process, only looking at elements that come after the current one.
    for db_index in 0..string_vector.len() {
        let number = &string_vector[db_index];
        let sub_vector = &string_vector[db_index + 1..];
        if is_number_contained_by_element_of_database(number, sub_vector) {
            return false;
        }
    }
    true
}

// Given a str of digits separated by new lines, return a Vec<String> of the digits for each line.
pub fn build_database(input: &str) -> Vec<String> {
    let db = input
        .split('\n')
        .map(|line: &str| parse_line(line))
        .collect::<Vec<String>>()
        .into_iter()
        .sorted()
        .collect();

    db
}

// Given a number and a database of numbers, return true if the number is a substring of any element in the database.
fn is_number_contained_by_element_of_database(number: &str, database: &[String]) -> bool {
    for element in database {
        if element.starts_with(number) {
            return true;
        }
    }
    false
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
            "911".to_string(),
            "91125426".to_string(),
            "97625992".to_string(),
        ];
        assert_eq!(build_database(input), expected_result);
    }

    #[test]
    fn test_build_database_sorted() {
        let input = "o) Bob 91 12 54 26\nAlice 97 625 992\nEmergency 911";
        let expected_result = vec![
            "911".to_string(),
            "91125426".to_string(),
            "97625992".to_string(),
        ];
        assert_eq!(build_database(input), expected_result);
    }

    #[test]
    fn test_is_number_contained_by_element_of_database() {
        let input = "o) Bob 91 12 54 26\nAlice 97 625 992\nEmergency 911";
        let database = build_database(input);
        assert_eq!(
            is_number_contained_by_element_of_database("911", &database),
            true
        );
        assert_eq!(
            is_number_contained_by_element_of_database("912", &database),
            false
        );
    }

    #[test]
    fn test_is_consistent() {
        let database = build_database("o) Bob 91 12 54 26\nEmergency 911");
        assert_eq!(is_consistent(&database), false);
        let database = build_database("o) Bob 91 12 54 26\non-Emergency 912");
        assert_eq!(is_consistent(&database), true);
    }
}
