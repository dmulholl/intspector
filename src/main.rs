extern crate term_size;
use std::iter::FromIterator;


fn main() {
    print_termline();
    for arg in std::env::args().skip(1) {
        match parse_int(&arg) {
            Some(value) => println!("{}", int_info(value, None)),
            None => println!("Error: cannot parse '{}' as a 64-bit signed integer.", arg),
        };
        print_termline();
    }
}


fn info_header(value: i64, min_bits: u32, num_bits: u32) -> String {
    if value == 0 || value == 1 {
        format!("req: 1 bit (unsigned)\n")
    } else if value > 1 {
        format!("req: {} bits (unsigned)\n", min_bits)
    } else {
        format!("req: {} bits (signed), showing {}-bit two's complement\n", min_bits, num_bits)
    }
}


fn int_info(value: i64, user_bits: Option<u32>) -> String {
    let (min_bits, num_bits) = bit_size(value, user_bits);
    let disp_value: u64;

    if value >= 0 {
        disp_value = value as u64;
    } else {
        if num_bits < 64 {
            disp_value = (2 as u64).pow(num_bits as u32) - value.abs() as u64;
        } else if num_bits == 64 {
            disp_value = (u64::MAX - value.abs() as u64) + 1;
        } else {
            return "Error: unsupported bit size.".to_string();
        }
    }

    info_header(value, min_bits, num_bits) + &uint_info(disp_value, num_bits)
}


fn bit_size(value: i64, user_bits: Option<u32>) -> (u32, u32) {
    let min_bits: u32;
    let mut num_bits: u32;

    if value == 0 {
        min_bits = 1;
        num_bits = user_bits.unwrap_or(1);
    } else if value > 0 {
        min_bits = (value as f64).log2().floor() as u32 + 1;
        num_bits = user_bits.unwrap_or(min_bits);
    } else {
        min_bits = (value.abs() as f64).log2().floor() as u32 + 2;
        num_bits = match user_bits {
            Some(value) => value,
            None => {
                let mut best_fit = 64;
                for std_size in vec![8, 16, 32] {
                    if min_bits <= std_size {
                        best_fit = std_size;
                        break;
                    }
                }
                best_fit
            }
        };
    }

    if num_bits < min_bits {
        num_bits = min_bits;
    }
    (min_bits, num_bits)
}


fn uint_info(value: u64, num_bits: u32) -> String {
    format!(
        "hex: {}\ndec: {}\noct: {:o}\nbin: {}",
        spaced_string(&format!("{:X}", value), ' ', 2),
        spaced_string(&value.to_string(), ',', 3),
        value,
        bin_string(value, num_bits),
    )
}


fn spaced_string(string: &str, spacer: char, block_len: u32) -> String {
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


fn bin_string(mut value: u64, num_bits: u32) -> String {
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


fn parse_int(arg: &str) -> Option<i64> {
    if arg.is_empty() {
        return None;
    }

    let trimmed = arg.trim_start_matches('0');
    if trimmed.is_empty() {
        return Some(0);
    }

    let radix: u32 = match trimmed.chars().next().unwrap().to_ascii_lowercase() {
        'b' => 2,
        'o' => 7,
        'x' => 16,
        _ => 10,
    };

    match i64::from_str_radix(trimmed, radix) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}


fn print_termline() {
    if let Some((w, _)) = term_size::dimensions() {
        print!("\u{001B}[90m");
        for _ in 0..w {
            print!("─");
        }
        print!("\u{001B}[0m\n");
    }
}

