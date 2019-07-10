// TODO: Have parse_input remove

// TODO: Figure out a way to represent each brainfuck command
// < > . , + - [ ]

// TODO: Figure out how to represent the 'tape'
// 30k length was used for the array in original BF, so lets do that

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
fn evaluate_brainfuck(mut code: String) {
    let memory: [i32; 30000] = [0; 30000]; // zero-initialize every cell
    let pointer: i32 = 0; // pointer starts at 0

    code = strip_chars(code); // strips a-zA-Z, etc everything not used in brainfuck
    for c in code.chars() {

    }
}

/// Sets up a vector containing a list of each [] bracket's
/// pairing, to make evaluating them easier.
fn setup_bracket_mapping(code: String) {

}

/// Strips all non-BF characters (all besides + - < > [ ] . ,)
fn strip_chars(code: String) -> String {
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
    } else {
        println!("no file :(");
    }
}

