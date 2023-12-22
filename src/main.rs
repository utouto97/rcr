use std::env;
use std::process;

mod tokenizer;
use tokenizer::tokenize;

mod parser;
use parser::Parser;

mod generator;
use generator::{generate, generate_pop};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }

    let tokens = tokenize(&args[1]);
    // println!("{:?}", tokens);
    if tokens.len() == 0 {
        eprintln!("式が空です");
        process::exit(1);
    }
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse_expr();
    // println!("{:?}", parsed);

    println!(".globl main");
    println!("main:");

    generate(parsed);
    generate_pop("a0".to_string());

    println!("  ret");
}
