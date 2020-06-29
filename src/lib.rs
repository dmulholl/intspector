use std::iter::FromIterator;


/// Returns the minimum number of bits required to represent the integer. For positive input, gives
/// the number of unsigned bits. For negative input, gives the number of two's complement bits.
pub fn min_bits(value: i64) -> u32 {
    if value == 0 {
        1
    } else if value > 0 {
        (value as f64).log2().floor() as u32 + 1
    } else {
        (value.abs() as f64).log2().ceil() as u32 + 1
    }
}


/// Returns the output from min_bits() rounded up to a standard integer size - either 8, 16, 32,
/// or 64 bits.
pub fn std_bits(value: i64) -> u32 {
    let min_bits = min_bits(value);
    for std_size in vec![8, 16, 32, 64] {
        if min_bits <= std_size {
            return std_size;
        }
    }
    min_bits
}


/// Adds spacer characters to a string.
pub fn add_spacers(string: &str, spacer: char, block_len: u32) -> String {
    let mut chars: Vec<char> = Vec::new();
    for (i, c) in string.chars().rev().enumerate() {
        chars.push(c);
        if i as u32 % block_len == block_len - 1 {
            chars.push(spacer);
        }
    }
    if chars.last().unwrap() == &spacer {
        chars.pop();
    }
    chars.reverse();
    String::from_iter(chars)
}


/// Converts an integer into a binary string, showing the specified number of low-order bits.
pub fn bin_string(mut value: u64, num_bits: u32) -> String {
    let mut chars: Vec<char> = Vec::new();

    for i in 0..num_bits {
        if value & 1 == 1 {
            chars.push('1');
        } else {
            chars.push('0');
        }
        value >>= 1;
        if i < num_bits - 1 {
            if i % 8 == 3 {
                chars.push('_');
            }
            if i % 8 == 7 {
                chars.push(' ');
            }
        }
    }

    chars.reverse();
    String::from_iter(chars)
}


/// Returns the n-bit two's complement of `value`. Will panic if `n > 64` or `value >= 2^n`.
pub fn twos_complement(value: u64, num_bits: u32) -> u64 {
    assert!(num_bits <= 64);
    if value == 0 {
        return 0;
    }
    if num_bits < 64 {
        let cap = (2 as u64).pow(num_bits);
        assert!(value < cap);
        return cap - value;
    }
    return (u64::MAX - value) + 1
}


/// Attempts to parse the string as a binary, octal, decimal, or hex integer.
pub fn parse_int(arg: &str) -> Option<i64> {
    if arg.is_empty() {
        return None;
    }

    let mut trimmed = arg.trim_start_matches('0');
    if trimmed.is_empty() {
        return Some(0);
    }

    let mut radix: u32 = 10;
    if trimmed.starts_with('b') {
        radix = 2;
        trimmed = trimmed.trim_start_matches('b');
    } else if trimmed.starts_with('o') {
        radix = 8;
        trimmed = trimmed.trim_start_matches('o');
    } else if trimmed.starts_with('d') {
        radix = 10;
        trimmed = trimmed.trim_start_matches('d');
    } else if trimmed.starts_with('x') {
        radix = 16;
        trimmed = trimmed.trim_start_matches('x');
    }

    match i64::from_str_radix(trimmed, radix) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}


/// If `value` is a valid ASCII code, returns a string representation - either the character itself
/// or a description if the character is in the unprintable range.
pub fn ascii(value: i64) -> Option<String> {
    if value < 0 || value > 127 {
        return None;
    }
    if value > 32 && value < 127 {
        return Some(format!("{}", value as u8 as char));
    }
    let name = match value {
        0 => "[null]",
        1 => "[start of heading]",
        2 => "[start of text]",
        3 => "[end of text]",
        4 => "[end of transmission]",
        5 => "[enquiry]",
        6 => "[acknowledge]",
        7 => "[bell]",
        8 => "[backspace]",
        9 => "[horizontal tab]",
        10 => "[line feed]",
        11 => "[vertical tab]",
        12 => "[form feed]",
        13 => "[carriage return]",
        14 => "[shift out]",
        15 => "[shift in]",
        16 => "[data link escape]",
        17 => "[device control 1]",
        18 => "[device control 2]",
        19 => "[device control 3]",
        20 => "[device control 4]",
        21 => "[negative acknowledge]",
        22 => "[synchronous idle]",
        23 => "[end of transmission block]",
        24 => "[cancel]",
        25 => "[end of medium]",
        26 => "[substitute]",
        27 => "[escape]",
        28 => "[file separator]",
        29 => "[group separator]",
        30 => "[record separator]",
        31 => "[unit separator]",
        32 => "[space]",
        127 => "[del]",
        _ => panic!("unexpected value"),
    };
    Some(name.to_string())
}

