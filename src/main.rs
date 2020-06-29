extern crate term_size;
extern crate arguably;

use arguably::ArgParser;
use intspector::min_bits;
use intspector::std_bits;
use intspector::add_spacers;
use intspector::bin_string;
use intspector::twos_complement;
use intspector::parse_int;
use intspector::ascii;


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

Command Help:
  help <command>        Print the specified command's help text.
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
        println!("lit: {}", c);
        println!("uni: U+{:04X}", c as u32);
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
            println!("uni: U+{:04X}", arg_as_i64);
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
        println!("uni: U+{:04X}", arg_as_u32);
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


fn uint_info(value: u64, num_bits: u32) -> String {
    format!(
        "hex: {}\ndec: {}\noct: {:o}\nbin: {}",
        add_spacers(&format!("{:X}", value), ' ', 2),
        add_spacers(&value.to_string(), ',', 3),
        value,
        bin_string(value, num_bits),
    )
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

