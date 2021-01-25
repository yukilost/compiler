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

    let mut lexer = Lexer::new(&args[1]);

    lexer.lex();
    println!("  mov rax, {}", lexer.token.parse::<i64>().expect("Not a number"));

    while lexer.lex() {
        match lexer.token {
            "+" => {
                lexer.lex();
                println!("  add rax, {}", lexer.token.parse::<i64>().expect("Not a number"));
            },
            "-" => {
                lexer.lex();
                println!("  sub rax, {}", lexer.token.parse::<i64>().expect("Not a number"));
            },
            _ => {
                panic!("Unexpected token");
            }
        }
    }

    println!("  ret");
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