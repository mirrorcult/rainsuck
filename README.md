![rainsuck logo](https://i.imgur.com/NMXFise.png)

[![Build Status](https://travis-ci.org/cyclowns/rainsuck.svg?branch=master)](https://travis-ci.org/cyclowns/rainsuck) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

# rainsuck

rainsuck (or rAINsUCK, since, y'know, rs) is a brainfuck interpreter written in Rust. This was
just a little side-project to get myself back into coding in Rust, so don't expect it to be amazingly
well-maintained.

## Using rainsuck

Run `rainsuck -h` or `rainsuck --help` after installing for help.

```
USAGE:
    rainsuck [FLAGS] <INPUT>

FLAGS:
    -f, --file       Tells rainsuck that your input is a file. Otherwise, assumed that it is a string
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <INPUT>    Brainfuck code to be interpreted, either a file or string as set by -f
```

## Example rainsuck usage

Using rainsuck with a file:
```bash
$ rainsuck -f testing/helloworld.bf
Program ended with: Hello World!
Time elapsed was: 249.863s
```

Using rainsuck with a string:
```bash
$ rainsuck "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+."
Program ended with: Hello World!
Time elapsed was: 249.863µs
```

## Install rainsuck

Building and installing rainsuck is simple.

- First, make sure you have rustc and cargo installed--you probably already do if you found
this.

- Then, run these commands:

```bash
$ git clone https://github.com/cyclowns/rainsuck
$ cd rainsuck
$ cargo build --release
```

Your rainsuck executable will be located at ./target/release/rainsuck. From here, you can
move it into your /usr/bin (on *nix) or anywhere else you please to run it. Untested on Windows
or Mac, but I would be genuinely surprised if it didn't work.

## Contributing

..wait, you want to contribute? Okay, I guess. If you want to submit a PR, just make sure that
you've documented the important bits and ran `rustfmt` on your code.