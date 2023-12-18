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
    println!("  addi a0, zero, 0");

    let mut num: i32 = 0;
    let mut op = '+';
    for c in args[1].chars() {
        match c {
            '+' => {
                println!("  addi a0, a0, {}{}", op, num);
                num = 0;
                op = '+';
            }
            '-' => {
                println!("  addi a0, a0, {}{}", op, num);
                num = 0;
                op = '-';
            }
            '0'..='9' => {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            }
            _ => {
                eprintln!("不正な文字です: {}", c);
                process::exit(1);
            }
        }
    }
    println!("  addi a0, a0, {}{}", op, num);

    println!("  ret");
}
