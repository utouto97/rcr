use std::env;
use std::process;

mod tokenizer;
use tokenizer::Tokenizer;

mod parser;
use parser::Parser;

mod generator;
use generator::{generate, generate_pop, generate_push};

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
    let parsed = Parser::new(tokens).parse_program();
    // println!("{:?}", parsed);

    println!(".globl main");
    println!("main:");

    // fpを退避・更新
    println!("  addi sp, sp, -4");
    println!("  sw fp, 0(sp)");
    println!("  addi fp, sp, 0");

    for _ in 0..=26 {
        generate_push("zero".to_string());
    }

    for p in parsed.iter() {
        generate(p.clone(), "0".to_string());
    }
    // generate(parsed, "0".to_string());
    generate_pop("a0".to_string());

    // fpを復元
    println!("  lw fp, 0(sp)");
    println!("  addi sp, sp, 4");

    println!("  ret");
}
