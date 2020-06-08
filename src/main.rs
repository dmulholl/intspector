extern crate term_size;
extern crate arguably;

use std::iter::FromIterator;
use arguably::ArgParser;


#[cfg(test)]
mod tests;


fn help() -> &'static str {
"
Usage: intspector [FLAGS] [OPTIONS] [ARGUMENTS]

  Integer conversion utility. Accepts integer input in [b]inary, [o]ctal,
  [d]ecimal, or he[x]adecimal base, then displays the number in all four bases.

  Use a single letter prefix to declare the base of the input, e.g. b1010.
  The base defaults to decimal if the prefix is omitted.

  This utility:

  - Accepts integer literals with a leading zero, e.g. 0x123.
  - Accepts multiple arguments.
  - Accepts input in the signed 64-bit integer range.
  - Displays the two's complement value for negative integers.

Arguments:
  [integers]            List of integers to convert.

Options:
  -b, --bits <n>        Number of binary digits to display. (Determines the
                        two's complement value for negative integers.)

Flags:
  -h, --help            Print this help text.
  -v, --version         Print the application's version number.

Commands:
  c2u, char2unicode     Convert character literals to unicode code points.
  u2c, unicode2char     Convert unicode code points to character literals.
"
}


fn help_c2u() -> &'static str {
"
Usage: intspector c2u|char2unicode [ARGUMENTS]

  Converts character literals to unicode code points, i.e. takes a list of
  chacters as input and prints out the unicode code point for each character
  in the list.

Arguments:
  [characters]      List of character literals.

Flags:
  -h, --help        Print this help text.
"
}


fn help_u2c() -> &'static str {
"
Usage: intspector u2c|unicode2char [ARGUMENTS]

  Converts unicode code points to character literals. Code points can be
  specified in binary, octal, decimal, or hexadecimal base.

Arguments:
  [integers]        List of unicode code points.

Flags:
  -h, --help        Print this help text.
"
}


fn main() {
    let mut parser = ArgParser::new()
        .helptext(help())
        .version(env!("CARGO_PKG_VERSION"))
        .option("bits b")
        .command("c2u char2unicode", ArgParser::new()
            .helptext(help_c2u())
            .callback(cmd_char2unicode)
        )
        .command("u2c unicode2char", ArgParser::new()
            .helptext(help_u2c())
            .callback(cmd_unicode2char)
        );

    if let Err(err) = parser.parse() {
        err.exit();
    }

    if !parser.has_cmd() {
        default_action(&parser);
    }
}


fn default_action(parser: &ArgParser) {
    let bits_arg: Option<u32> = match parser.value("bits").unwrap() {
        Some(arg) => {
            match arg.parse::<u32>() {
                Ok(value) => Some(value),
                Err(_) => {
                    eprintln!("Error: cannot parse '{}' as a 32-bit unsigned integer.", arg);
                    std::process::exit(1);
                }
            }
        },
        None => None
    };
    if parser.has_args() {
        print_termline();
        for arg in parser.args() {
            match parse_int(&arg) {
                Some(value) => println!("{}", int_info(value, bits_arg)),
                None => println!("Error: cannot parse '{}' as a 64-bit signed integer.", arg),
            };
            print_termline();
        }
    }
}


fn cmd_char2unicode(_cmd: &str, parser: &ArgParser) {
    let mut argstring = String::new();
    for arg in parser.args() {
        argstring.push_str(&arg);
    }
    if !argstring.is_empty() {
        print_termline();
    }
    for c in argstring.chars() {
        println!("lit: '{}'", c);
        println!("{}", uint_info(c as u64, std_bits(c as i64)));
        print_termline();
    }
}


fn cmd_unicode2char(_cmd: &str, parser: &ArgParser) {
    if parser.has_args() {
        print_termline();
    }
    for arg in parser.args() {
        let arg_as_i64 = match parse_int(&arg) {
            Some(value) => value,
            None => {
                println!("Error: cannot parse '{}' as an integer.", arg);
                print_termline();
                continue;
            }
        };
        if arg_as_i64 < 0 {
            println!("Error: invalid input '{}'.", arg);
            print_termline();
            continue;
        }
        if let Some(ascii) = ascii(arg_as_i64) {
            println!("lit: {}", ascii);
            print_termline();
            continue;
        }
        let arg_as_u32 = arg_as_i64 as u32;
        let arg_as_char = match std::char::from_u32(arg_as_u32) {
            Some(value) => value,
            None => {
                println!("Error: {} is not a valid unicode scalar value.", arg_as_u32);
                print_termline();
                continue;
            }
        };
        println!("lit: {}", arg_as_char);
        print_termline();
    }
}


fn int_info(value: i64, user_bits: Option<u32>) -> String {
    let min_bits = min_bits(value);
    let std_bits = std_bits(value);

    let num_bits = if value >= 0 {
        user_bits.unwrap_or(min_bits)
    } else {
        user_bits.unwrap_or(std_bits)
    };

    if num_bits == 0 || num_bits > 64 {
        return format!("Error: unsupported bit size.");
    }
    if num_bits < min_bits {
        return format!("Error: {} requires at least {} bits.", value, min_bits);
    }

    let disp_value: u64 = if value >= 0 {
        value as u64
    } else {
        twos_complement(value.abs() as u64, num_bits)
    };

    let plural = if min_bits == 1 { "" } else { "s" };
    let requires = if value >= 0 {
        format!("req: {} bit{} (unsigned)\n", min_bits, plural)
    } else {
        format!(
            "req: {} bit{} (signed), showing {}-bit two's complement\n",
            min_bits, plural, num_bits
        )
    };

    let mut output = requires + &uint_info(disp_value, num_bits);
    if let Some(ascii) = ascii(value) {
        output += &format!("\nasc: {}", ascii);
    }
    output
}


fn ascii(value: i64) -> Option<String> {
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


fn twos_complement(value: u64, num_bits: u32) -> u64 {
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


// Minimum number of bits required to represent the integer. For positive input,
// gives the number of signed bits. For negative input, gives the number of two's
// complement bits.
fn min_bits(value: i64) -> u32 {
    if value == 0 {
        1
    } else if value > 0 {
        (value as f64).log2().floor() as u32 + 1
    } else {
        (value.abs() as f64).log2().ceil() as u32 + 1
    }
}


// Returns the min_bits() value rounded up to a standard integer size.
fn std_bits(value: i64) -> u32 {
    let min_bits = min_bits(value);
    for std_size in vec![8, 16, 32, 64] {
        if min_bits <= std_size {
            return std_size;
        }
    }
    min_bits
}


fn uint_info(value: u64, num_bits: u32) -> String {
    format!(
        "hex: {}\ndec: {}\noct: {:o}\nbin: {}",
        add_spacers(&format!("{:X}", value), ' ', 2),
        add_spacers(&value.to_string(), ',', 3),
        value,
        bin_string(value, num_bits),
    )
}


fn add_spacers(string: &str, spacer: char, block_len: u32) -> String {
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


fn print_termline() {
    if let Some((w, _)) = term_size::dimensions() {
        print!("\u{001B}[90m");
        for _ in 0..w {
            print!("─");
        }
        print!("\u{001B}[0m\n");
    }
}

