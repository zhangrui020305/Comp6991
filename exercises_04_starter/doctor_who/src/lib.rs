//! #Caesar_shift
//! This crate provides functions that encrypt text by shifting alphabet characters

/// Default shift is 5, if not be set
const DEFAULT_SHIFT: i32 = 5;

/// ASCII value of character A
const UPPERCASE_A: i32 = 65;

/// ASCII value of character a
const LOWERCASE_A: i32 = 97;

/// The length of alphabet
const ALPHABET_SIZE: i32 = 26;

/// shift each line of input using caesar shift
/// 
/// # Examples
/// ```
/// use doctor_who:caesar_shift;
/// let lines = vec!["abc".toSting()];
/// let result = caesar_shift(Some(2), lines);
/// assert_eq!(result, vec!["cde"]);
/// ```
pub fn caesar_shift(shift_by: Option<i32>, lines: Vec<String>) -> Vec<String> {
    let shift_number = shift_by.unwrap_or(DEFAULT_SHIFT);
    
    // no idea what this is doing? Ask the forums and/or 
    // look back at the functional programming lectures!
    lines
        .iter()
        .map(|line| shift(shift_number, line.to_string()))
        .collect()
}

/// shift the string by given number
fn shift(shift_by: i32, line: String) -> String {
    let mut result: Vec<char> = Vec::new();

    // turn shift_by into a positive number between 0 and 25
    let shift_by = shift_by % ALPHABET_SIZE + ALPHABET_SIZE;

    line.chars().for_each(|c| {
        let ascii = c as i32;

        if ('A'..='Z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - UPPERCASE_A) + shift_by, ALPHABET_SIZE) + UPPERCASE_A,
            ));
        } else if ('a'..='z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - LOWERCASE_A) + shift_by, ALPHABET_SIZE) + LOWERCASE_A,
            ));
        } else {
            result.push(c)
        }
    });

    result.iter().collect()
}

/// return the positive reminder of the division
fn abs_modulo(a: i32, b: i32) -> i32 {
    (a % b).abs()
}

// convert integer to a character by ASCII
fn to_ascii(i: i32) -> char {
    char::from_u32(i as u32).unwrap()
}
