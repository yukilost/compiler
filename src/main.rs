use std::env;

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
}
