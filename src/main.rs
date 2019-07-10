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

#[allow(dead_code)]
/// Parses input into a stack (Vec) that is processed later
fn parse_input() {

}

#[allow(dead_code)]
fn run_bf_code() {

}

fn main() {
    let matches = clap_app!(rainsuck =>
        (version: "0.1.1")
        (author: "cyclowns <cyclowns@protonmail.ch>")
        (about: "Brainfuck interpreter for rust")
        (@arg FILE: -f --file "Tells rainsuck that your input is a file")
        (@arg STRING: -s --string "Tells rainsuck that your input is a string")
        (@arg INPUT: +required "Brainfuck code to be interpreted, either a file or string as set by -f or -s")
    ).get_matches();
}

