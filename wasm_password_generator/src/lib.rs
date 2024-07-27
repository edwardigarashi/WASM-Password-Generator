use wasm_bindgen::prelude::*;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;

#[wasm_bindgen]
pub fn generate_password(
    length: usize, 
    include_numbers: bool, 
    include_upper_alphabets: bool,
    include_lower_alphabets: bool,
    special_chars: &str, 
    exclude_chars: &str,
    no_similar: bool,
    no_duplicates: bool,
    no_sequential: bool
) -> String {
    let mut chars = String::new();
    if include_numbers {
        chars.push_str("0123456789");
    }
    if include_upper_alphabets {
        chars.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if include_lower_alphabets {
        chars.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if !special_chars.is_empty() {
        chars.push_str(special_chars);
    }
    // Remove excluded characters
    let mut chars: Vec<char> = chars.chars().filter(|c| !exclude_chars.contains(*c)).collect();
    // Remove similar characters if requested
    if no_similar {
        chars.retain(|&c| !"iIl1Lo0O".contains(c));
    }
    // Ensure we have characters to work with
    if chars.is_empty() {
        return String::from("Invalid character set");
    }
    let mut rng = SmallRng::from_entropy();
    let mut password = String::new();
    while password.len() < length {
        let idx = rng.gen_range(0..chars.len());
        let next_char = chars[idx];
        if no_duplicates && password.contains(next_char) {
            continue;
        }
        if no_sequential && password.len() > 0 {
            let last_char = password.chars().last().unwrap();
            if (last_char as u8) + 1 == (next_char as u8) || (last_char as u8) == (next_char as u8) + 1 {
                continue;
            }
        }
        password.push(next_char);
    }
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_length() {
        let password = generate_password(10, true, true,true, "!@#", "", false, false, false);
        assert_eq!(password.len(), 10);
    }

    #[test]
    fn test_generate_password_exclude_chars() {
        let exclude = "abc";
        let password = generate_password(20, true, true,true, "!@#", exclude, false, false, false);
        for ch in exclude.chars() {
            assert!(!password.contains(ch));
        }
    }

    #[test]
    fn test_generate_password_include_numbers() {
        let password = generate_password(20, true, false,false, "", "", false, false, false);
        assert!(password.chars().all(|c| c.is_digit(10)));
    }

    #[test]
    fn test_generate_password_include_alphabets() {
        let password = generate_password(20, false, true,true, "", "", false, false, false);
        assert!(password.chars().all(|c| c.is_alphabetic()));
    }

    #[test]
    fn test_generate_password_include_specials() {
        let specials = "!@#";
        let password = generate_password(20, false, false,false, specials, "", false, false, false);
        println!("Generated password: {}", &password);
        assert!(password.chars().all(|c| specials.contains(c)));
    }

    #[test]
    fn test_generate_password_no_similar() {
        let password = generate_password(20, true, true,true, "!@#", "", true, false, false);
        assert!(!password.chars().any(|c| "il1Lo0O".contains(c)));
    }

    #[test]
    fn test_generate_password_no_duplicates() {
        let password = generate_password(20, true, true,true, "!@#", "", false, true, false);
        let mut unique_chars: Vec<char> = password.chars().collect();
        unique_chars.sort();
        unique_chars.dedup();
        assert_eq!(unique_chars.len(), password.len());
    }

    #[test]
    fn test_generate_password_no_sequential() {
        let password = generate_password(20, true, true,true, "!@#", "", false, false, true);
        let mut prev_char: Option<char> = None;
        for c in password.chars() {
            if let Some(prev) = prev_char {
                assert!((prev as u8) + 1 != (c as u8) && (prev as u8) != (c as u8) + 1);
            }
            prev_char = Some(c);
        }
    }
}
