// TODO: Get command-line stuff working
// -f (--file) arg for file input, -s (--string) arg for string input
// -t (--textify) arg to take in text and produce BF code? maybe later?

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
use clap::App;

#[allow(dead_code)]
/// Parses input into a stack (Vec) that is processed later
fn parse_input() {

}

#[allow(dead_code)]
fn run_bf_code() {

}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
}

