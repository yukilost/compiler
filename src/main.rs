extern crate regex;

use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Incorrect number of arguents");
    }

    println!(".intel_syntax noprefix");
    println!(".globl _main");
    println!("_main:");
    println!("  mov rax, {}", &args[1].parse::<i32>().expect("Not a number"));
    println!("  ret");

    let mut lexer = Lexer::new(" 5 + 7 - 40");
    while lexer.lex() {
        println!("{}", lexer.token);
    }
}
struct Lexer<'a> {
    pub line: &'a str,
    pub token: &'a str,
    regular_expression: Vec<Regex>
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {

        Lexer {
            line: input,
            token: "",
            regular_expression: vec![
                Regex::new(r"(\A\s+)(.*)").unwrap(),
                Regex::new(r"(\A\d+)(.*)").unwrap(),
                Regex::new(r"(\A\+)(.*)").unwrap(),
                Regex::new(r"(\A\-)(.*)").unwrap()
            ]
        }
    }

    pub fn lex(&mut self) -> bool {
        for re in self.regular_expression.iter() {
            let matches = re.captures(self.line);
            if matches.is_some() {
                let m = matches.unwrap();
                self.token = m.get(1).unwrap().as_str();
                self.line = m.get(2).unwrap().as_str();
                return true;
            }
        }
        return false;
    }
}