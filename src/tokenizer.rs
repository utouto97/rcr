use std::process;

#[derive(Debug, Clone)]
pub enum Token {
    Number(i64),
    Operator(String),
    LeftParen,
    RightParen,
    EOF,
}

pub struct Tokenizer {
    s: String,
}

impl Tokenizer {
    pub fn new(s: String) -> Tokenizer {
        Tokenizer { s }
    }

    pub fn tokenize(self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = self.s.chars().peekable();
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
                    let pos = self.s.len() - chars.count() - 1;
                    self.tokenize_error(pos);
                    eprintln!("不正な文字です: {}", c);
                    process::exit(1);
                }
            }
        }
        tokens
    }

    fn tokenize_error(self, pos: usize) {
        println!("{}", self.s);
        println!("{}", " ".repeat(pos) + "^");
    }
}