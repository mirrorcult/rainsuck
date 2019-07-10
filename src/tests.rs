use super::*;
#[test]
fn hello_world() {
    assert_eq!(evaluate_brainfuck(String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.")), String::from("Hello World!"));
}
