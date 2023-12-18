use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }

    println!(".globl main");
    println!("main:");
    println!("  addi a0, zero, {}", args[1]);
    println!("  ret");
}
