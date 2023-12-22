use std::env;
use std::process;

#[derive(Debug, Clone)]
enum Token {
    Number(i64),
    Operator(String),
    EOF,
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

#[derive(Clone, Debug)]
enum NodeType {
    ROOT,
    ADD,
    SUB,
    MUL,
    DIV,
    NUM(i64),
}

#[derive(Clone, Debug)]
struct Node {
    value: NodeType,
    children: Vec<Box<Node>>,
}

impl Node {
    fn new(value: NodeType) -> Box<Node> {
        Box::new(Node {
            value,
            children: Vec::new(),
        })
    }

    fn add_child(&mut self, child: Box<Node>) -> Box<Node> {
        Box::new(Node {
            value: self.value.clone(),
            children: [self.children.clone(), vec![child]].concat(),
        })
    }
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: [tokens.clone(), vec![Token::EOF]].concat(),
            pos: 0,
        }
    }

    fn token(&mut self) -> &Token {
        &self.tokens[self.pos]
    }

    fn next(&mut self) {
        self.pos += 1;
    }

    fn parse_expr(&mut self) -> Box<Node> {
        let mut root = self.parse_mul();

        loop {
            match self.token() {
                Token::Operator(op) => match op.as_str() {
                    "+" => {
                        self.next();
                        root = Node::new(NodeType::ADD)
                            .add_child(root)
                            .add_child(self.parse_mul())
                    }
                    "-" => {
                        self.next();
                        root = Node::new(NodeType::SUB)
                            .add_child(root)
                            .add_child(self.parse_mul())
                    }
                    _ => break,
                },
                _ => break,
            }
        }

        root
    }

    fn parse_mul(&mut self) -> Box<Node> {
        let mut root = self.parse_primary();
        self.next();

        loop {
            match self.token() {
                Token::Operator(op) => match op.as_str() {
                    "*" => {
                        self.next();
                        root = Node::new(NodeType::MUL)
                            .add_child(root)
                            .add_child(self.parse_primary())
                    }
                    "/" => {
                        self.next();
                        root = Node::new(NodeType::DIV)
                            .add_child(root)
                            .add_child(self.parse_primary())
                    }
                    _ => break,
                },
                _ => break,
            }
        }

        root
    }

    fn parse_primary(&mut self) -> Box<Node> {
        match self.token() {
            Token::Number(n) => Node::new(NodeType::NUM(*n)),
            _ => {
                eprintln!("数値ではありません");
                process::exit(1);
            }
        }
    }
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
                    _ => {}
                }
            }
            Token::Number(_n) => {}
            _ => {}
        }
    }

    println!("  ret");

    println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);
    println!("{:?}", parser.parse_expr());
}
