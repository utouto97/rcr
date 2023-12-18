use std::env;
use std::process;

#[derive(Debug)]
enum Token {
    Number(i64),
    Operator(String),
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '+' | '-' => {
                tokens.push(Token::Operator(c.to_string()));
            }
            '0'..='9' => {
                let mut num = c.to_digit(10).unwrap() as i64;
                while let Some('0'..='9') = chars.peek() {
                    num = num * 10 + chars.next().unwrap().to_digit(10).unwrap() as i64;
                }
                tokens.push(Token::Number(num));
            }
            ' ' | '\r' | '\n' | '\t' => {}
            _ => {
                eprintln!("不正な文字です: {}", c);
                process::exit(1);
            }
        }
    }
    tokens
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }

    let tokens = tokenize(&args[1]);
    if tokens.len() == 0 {
        eprintln!("式が空です");
        process::exit(1);
    }
    match tokens[0] {
        Token::Number(n) => {
            println!(".globl main");
            println!("main:");
            println!("  addi a0, zero, {}", n);
        }
        _ => {
            eprintln!("式の最初は数値ではありません");
            process::exit(1);
        }
    }

    for i in 1..tokens.len() {
        let token = &tokens[i];
        match token {
            Token::Operator(op) => {
                if tokens.len() <= i + 1 {
                    eprintln!("演算子の後に式がありません");
                    process::exit(1);
                }

                let next_token = &tokens[i + 1];
                match next_token {
                    Token::Operator(_) => {
                        eprintln!("演算子の後に演算子があります");
                        process::exit(1);
                    }
                    Token::Number(n) => {
                        if op == "+" {
                            println!("  addi a0, a0, {}", n);
                        } else if op == "-" {
                            println!("  addi a0, a0, -{}", n);
                        } else {
                            eprintln!("不正な演算子です: {}", op);
                            process::exit(1);
                        }
                    }
                }
            }
            Token::Number(_n) => {}
        }
    }

    println!("  ret");
}
