// This library implements the soundex rules:
// 1. Retain the first letter. Drop all other occurrences of a, e, i, o, u, y, h, w.
// 2. Replace consonants with digits (after the first letter):
//    - b, f, p, v => 1
//    - c, g, j, k, q, s, x, z => 2
//    - d, t => 3
//    - l => 4
//    - m, n => 5
//    - r => 6
// 3a. If a first letter encodes to a first number
//     and is followed by a second character which h or w which is then followed by a third character that encodes to the first number,
//          then encode the three letters as the single first number.
// 3b. If a first letter encodes (or would encode for the first letter of the output) to a first number
//     and is followed by a second character which is a vowel (aeiouy) which is then followed by a third character that encodes to the first number,
//          then encode the first number twice.
//
// 4. If you have too few letters in your word that you can't assign three numbers, append with zeros
//    until there are three numbers.
// 5. Stop when you have four numbers.

// use core::num;

fn encode_digit(c: char) -> Option<char> {
    // Input: a character that is already lowercase.
    // returns the soundex digit for the character.
    // Probably could be faster using a hashmap.

    match c {
        'b' | 'f' | 'p' | 'v' => Some('1'),
        'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => Some('2'),
        'd' | 't' => Some('3'),
        'l' => Some('4'),
        'm' | 'n' => Some('5'),
        'r' => Some('6'),
        _ => None,
    }
}

fn encode_first_letter(input_word: &str) -> Option<char> {
    // If the first char is alphabetic, return the uppercase version of it
    // Otherwise, return None

    if let Some(ch) = input_word.chars().next() {
        if ch.is_alphabetic() {
            return ch.to_uppercase().next();
        } else {
            return None;
        }
    } else {
        return None;
    }
}

trait CharLike {
    fn is_vowel_like(&self) -> bool;
    fn is_h_or_w(&self) -> bool;
}

impl CharLike for char {
    fn is_vowel_like(&self) -> bool {
        match self {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => true,
            _ => false,
        }
    }

    fn is_h_or_w(&self) -> bool {
        match self {
            'h' | 'w' => true,
            _ => false,
        }
    }
}

pub fn soundex(input_word: &str) -> Option<String> {
    let input_word_lower = input_word.to_lowercase();
    let mut encoded_string: String = String::new();

    if let Some(first_letter) = encode_first_letter(&input_word_lower) {
        encoded_string.push(first_letter);
    } else {
        return None;
    }
    // Rule 2: -- digit encodings 2-4
    let mut taken = 1; // start at 1 because we've already taken the first letter
    let mut intervening_vowel: bool = false;
    for current_char in input_word_lower.chars().skip(1) {
        if current_char.is_vowel_like() {
            // Rule 1: Drop all other occurrences of a, e, i, o, u, y, h, w.
            intervening_vowel = true;
            continue;
        }
        if current_char.is_h_or_w() {
            continue;
        }

        if let Some(numeral) = encode_digit(current_char) {
            let last_digit_opt = last_digit(&encoded_string);
            if last_digit_opt != Some(numeral) {
                encoded_string.push(numeral);
                taken += 1;
                if taken >= 4 {
                    // stop when we have 4 characters.
                    break;
                }
            } else if last_digit_opt == Some(numeral) {
                if intervening_vowel {
                    encoded_string.push(numeral);
                    taken += 1;
                    if taken >= 4 {
                        break;
                    }
                }
            }
        }
        intervening_vowel = false;
    }
    // Rule 4: pad encoded with zeros to reach 4 characters in length
    let encoded_string = zero_pad(encoded_string);

    Some(encoded_string)
}

fn last_digit(encoded: &str) -> Option<char> {
    encoded.chars().last()
}

fn zero_pad(encoded: String) -> String {
    if encoded.len() < 4 {
        let padding = "0".repeat(4 - encoded.len());
        return format!("{}{}", encoded, padding);
    } else {
        encoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_retain_first_letter_of_one_letter_word() {
        let test_word = "A";
        assert_eq!(
            soundex(test_word)
                .unwrap()
                .chars()
                .next()
                .unwrap()
                .to_string(),
            test_word.chars().next().unwrap().to_string()
        );
    }

    #[test]
    fn test_2_pad_one_letter_word_with_zeros() {
        let test_word = "A";
        let test_result = "A000";
        assert_eq!(soundex(test_word).unwrap(), test_result);
    }

    #[test]
    fn test_3_encode_second_letter_encode_to_1() {
        assert_eq!(soundex("Ab").unwrap(), "A100");
        assert_eq!(soundex("Af").unwrap(), "A100");
        assert_eq!(soundex("Ap").unwrap(), "A100");
        assert_eq!(soundex("Av").unwrap(), "A100");
    }

    #[test]
    fn test_4_encode_second_letter_c_g_j_k_q_s_x_z_encode_to_2() {
        assert_eq!(soundex("Ac").unwrap(), "A200");
        assert_eq!(soundex("Ag").unwrap(), "A200");
        assert_eq!(soundex("Aj").unwrap(), "A200");
        assert_eq!(soundex("Ak").unwrap(), "A200");
        assert_eq!(soundex("Aq").unwrap(), "A200");
        assert_eq!(soundex("As").unwrap(), "A200");
        assert_eq!(soundex("Ax").unwrap(), "A200");
        assert_eq!(soundex("Az").unwrap(), "A200");
    }

    #[test]
    pub fn test_5_encode_second_letter_d_t_encode_to_3() {
        assert_eq!(soundex("Ad").unwrap(), "A300");
        assert_eq!(soundex("At").unwrap(), "A300");
    }

    #[test]
    fn test_drop_vowels() {
        assert_eq!(soundex("Aaeiou").unwrap(), "A000");
        assert_eq!(soundex("AaTeiou").unwrap(), "A300");
        assert_eq!(soundex("Ae").unwrap(), "A000");
        assert_eq!(soundex("Ao").unwrap(), "A000");
        assert_eq!(soundex("Ay").unwrap(), "A000");
        assert_eq!(soundex("Ah").unwrap(), "A000");
        assert_eq!(soundex("Aw").unwrap(), "A000");
    }

    #[test]
    fn test_6_encode_second_letter_l_encode_to_4() {
        assert_eq!(soundex("Al").unwrap(), "A400");
    }
    #[test]
    fn test_7_encode_second_letter_m_n_encode_to_5() {
        assert_eq!(soundex("Am").unwrap(), "A500");
        assert_eq!(soundex("An").unwrap(), "A500");
    }
    #[test]
    fn test_8_encode_second_letter_r_encode_to_6() {
        assert_eq!(soundex("Ar").unwrap(), "A600");
    }
    #[test]
    fn test_9_encode_second_other_second_letter_encode_to_itself() {
        assert_eq!(soundex("Ar").unwrap(), "A600");
    }

    #[test]
    fn test_10_encode_repeated_adjacent_letters_to_same_number() {
        assert_eq!(soundex("ab").unwrap(), "A100");
        assert_eq!(soundex("abfcgdt").unwrap(), "A123");
        assert_eq!(soundex("abbfcgdt").unwrap(), "A123");

        // assert_eq!(soundex("Abb").unwrap(), "A100");
    }

    #[test]
    fn test_11_check_capitalization_of_first_letter() {
        assert_eq!(soundex("a").unwrap(), "A000");
        assert_eq!(soundex("A").unwrap(), "A000");
    }

    #[test]
    fn test_12_drop_h_w_between_two_like_digits() {
        assert_eq!(soundex("Abhfchq").unwrap(), "A120");
    }

    #[test]
    fn test_13_drop_vowel_between_two_like_digits_and_double_encode_digits() {
        assert_eq!(soundex("Abyfcaq").unwrap(), "A112");
    }

    #[test]
    fn test_14_double_encode_duplicate_digit_after_first_letter() {
        // when the second letter is a consonant that has the same digit as the first letter, double it only if there is a vowel between the two consonants.
        assert_eq!(soundex("Bab").unwrap(), "B100");
        assert_eq!(soundex("Aab").unwrap(), "A100");
    }
}
