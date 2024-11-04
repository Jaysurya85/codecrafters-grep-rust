use std::env;
use std::io;
use std::process;

enum Pattern {
    Single(String),
    Digit,
    Unknown,
}

impl From<String> for Pattern {
    fn from(string_pattern: String) -> Pattern {
        if string_pattern.chars().count() == 1 {
            return Pattern::Single(string_pattern);
        } else if string_pattern == "\\d" {
            return Pattern::Digit;
        } else {
            return Pattern::Unknown;
        }
    }
}

fn match_single_letter(input_line: &str, pattern: &str) -> bool {
    input_line.contains(pattern)
}

fn match_digit(input_line: &str) -> bool {
    input_line.contains(|c: char| c.is_digit(10))
}

fn match_pattern(input_line: &str, pattern: Pattern) -> bool {
    return match pattern {
        Pattern::Single(s) => match_single_letter(input_line, &s),
        Pattern::Digit => match_digit(input_line),
        Pattern::Unknown => false,
    };
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let string_pattern = env::args().nth(2).unwrap();
    let pattern: Pattern = Pattern::from(string_pattern);

    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
