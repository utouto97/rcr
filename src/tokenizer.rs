use std::process;

#[derive(Debug, Clone)]
pub enum Token {
    Number(i64),
    Operator(String),
    LeftParen,
    RightParen,
    Ident(String),
    SEMICOLON,
    RESERVED(String),
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
                '<' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::Operator("<=".to_string()));
                    }
                    _ => {
                        tokens.push(Token::Operator("<".to_string()));
                    }
                },
                '>' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::Operator(">=".to_string()));
                    }
                    _ => {
                        tokens.push(Token::Operator(">".to_string()));
                    }
                },
                '=' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::Operator("==".to_string()));
                    }
                    _ => {
                        tokens.push(Token::Operator("=".to_string()));
                    }
                },
                '!' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::Operator("!=".to_string()));
                    }
                    _ => {
                        let pos = self.s.len() - chars.count() - 1;
                        self.tokenize_error(pos);
                        eprintln!("不正な文字です: {}", c);
                        process::exit(1);
                    }
                },
                '0'..='9' => {
                    let mut num = c.to_digit(10).unwrap() as i64;
                    while let Some('0'..='9') = chars.peek() {
                        num = num * 10 + chars.next().unwrap().to_digit(10).unwrap() as i64;
                    }
                    tokens.push(Token::Number(num));
                }
                ';' => tokens.push(Token::SEMICOLON),
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                'a'..='z' => {
                    let mut ident = c.to_string();
                    while let Some('a'..='z') = chars.peek() {
                        ident.push(chars.next().unwrap());
                    }
                    match ident.as_str() {
                        "return" => tokens.push(Token::RESERVED("return".to_string())),
                        _ => tokens.push(Token::Ident(ident)),
                    }
                }
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
        eprintln!("{}", self.s);
        eprintln!("{}", " ".repeat(pos) + "^");
    }
}
