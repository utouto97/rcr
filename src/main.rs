use std::env;
use std::process;

mod tokenizer;
use tokenizer::Tokenizer;

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

    let tokens = Tokenizer::new(args[1].clone()).tokenize();
    // println!("{:?}", tokens);
    if tokens.len() == 0 {
        eprintln!("式が空です");
        process::exit(1);
    }
    let parsed = Parser::new(tokens).parse_expr();
    // println!("{:?}", parsed);

    println!(".globl main");
    println!("main:");

    generate(parsed);
    generate_pop("a0".to_string());

    println!("  ret");
}
