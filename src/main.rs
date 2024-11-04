use std::env;
use std::io;
use std::process;

enum Pattern {
    Single,
    Digit,
    Unknown,
}

impl Form<String> for Pattern {
    fn from(string_pattern: String) -> Pattern {
        if string_pattern.chars().count() == 1 {
            return Pattern::Single;
        } else if string_pattern == "\\d" {
            return Pattern::Digit;
        } else {
            return Pattern::Unknown;
        }
    }
}

fn match_pattern(input_line: &str, pattern: &Pattern) -> bool {
    if *pattern == Pattern::Single {
        return input_line.contains(pattern);
    } else if *pattern == Pattern::Digit {
        return input_line.contains(|c: char| c.is_digit(10));
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
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
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
