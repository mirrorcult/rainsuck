// TODO: Test and make sure BF commands work

#[macro_use]
extern crate clap;

#[cfg(test)]
mod tests;

#[allow(unused_imports)]
use std::io::stdin;
use std::io::Read;
use std::path::Path;
use std::fs::File;

#[allow(dead_code)]
/// Parses a snippet of BF code and evaluates it
/// ```rust
/// let p = evaluate_brainfuck(String::new("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+."))
/// println!({}, p)
/// // p = "Hello World!"
/// ```
fn evaluate_brainfuck(code: String) -> String {
    let mut memory: [u8; 30000] = [0; 30000]; // zero-initialize every cell
    let mut pointer: usize = 0; // pointer starts at 0
    let mut result = String::new();

    let code_stripped = strip_chars(&code); // strips a-zA-Z, etc everything not used in brainfuck
    let bracket_map = setup_bracket_mapping(&code_stripped);
    let mut code_pointer = 0;
    while code_pointer < code.len() {
        let c = code.chars().nth(code_pointer).unwrap(); // wow, thats tedious
        match c {
            '+' => memory[pointer] += 1,
            '-' => memory[pointer] -= 1,
            '>' => pointer += 1,
            '<' => pointer -= 1,
            '.' => result.push(memory[pointer] as char),
            ',' => {
                let mut input = String::new();
                print!("Input needed: ");
                stdin().read_line(&mut input).expect("Input not correctly entered");
                memory[pointer] = input.parse::<u8>().unwrap();
            },
            '[' => {
                if memory[pointer] == 0 { code_pointer = bracket_map[code_pointer]; }
            },
            ']' => {
                if memory[pointer] != 0 { code_pointer = bracket_map[code_pointer]; }
            },
            _ => panic!("How the hell did this get here?!")
        }
        code_pointer += 1;
    }
    result
}

/// Sets up a vector containing a list of each [] bracket's
/// pairing, to make evaluating them easier.
fn setup_bracket_mapping(code: &str) -> Vec<usize> {
    let length = code.len();
    println!("{}", length);
    let mut bracket_vec: Vec<usize> = Vec::with_capacity(length);
    let mut temp_vec: Vec<usize> = Vec::with_capacity(length);
    bracket_vec.resize(length, 0); // initialize with zeroes so no out of bound

    let mut ptr: usize = 0;
    for c in code.chars() {
        if c == '[' { temp_vec.push(ptr) }
        if c == ']' {
            let start_pos = temp_vec.pop().unwrap();
            bracket_vec[start_pos] = ptr;
            bracket_vec[ptr] = start_pos;
        }
        ptr += 1;
    }
    println!("{:?}", bracket_vec);
    bracket_vec
}

/// Strips all non-BF characters (all besides + - < > [ ] . ,)
fn strip_chars(code: &str) -> String {
    let mut result = String::new();
    for c in code.chars() {
        if "+-<>[].,".contains(c) {
            result.push(c);
        }
    }
    result
}

fn main() {
    //
    let matches = clap_app!(rainsuck =>
        (version: "0.1.1")
        (author: "cyclowns <cyclowns@protonmail.ch>")
        (about: "Brainfuck interpreter for rust")
        (@arg FILE: -f --file "Tells rainsuck that your input is a file. Otherwise, assumed that it is a string")
        (@arg INPUT: +required "Brainfuck code to be interpreted, either a file or string as set by -f")
    ).get_matches();

    if matches.is_present("FILE") {
        // get filepath from input + check if it is real and is a file
        let fpath = matches.value_of("INPUT").unwrap();
        let rpath = Path::new(fpath);

        if rpath.is_file() {
            let mut file = File::open(rpath).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            evaluate_brainfuck(contents);
        }
    } else { // not file
        let contents = matches.value_of("INPUT").unwrap().to_string();
        evaluate_brainfuck(contents);
    }
}

