use std::process;

#[derive(Debug, Clone)]
pub enum Token {
    Number(i64),
    Operator(String),
    LeftParen,
    RightParen,
    EOF,
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '+' | '-' | '*' | '/' => tokens.push(Token::Operator(c.to_string())),
            '0'..='9' => {
                let mut num = c.to_digit(10).unwrap() as i64;
                while let Some('0'..='9') = chars.peek() {
                    num = num * 10 + chars.next().unwrap().to_digit(10).unwrap() as i64;
                }
                tokens.push(Token::Number(num));
            }
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            ' ' | '\r' | '\n' | '\t' => {}
            _ => {
                tokenize_error(s, s.len() - chars.count() - 1);
                eprintln!("不正な文字です: {}", c);
                process::exit(1);
            }
        }
    }
    tokens
}

fn tokenize_error(s: &str, pos: usize) {
    println!("{}", s);
    println!("{}", " ".repeat(pos) + "^");
}
