use std::env;
use std::process;

#[derive(Debug, Clone)]
enum Token {
    Number(i64),
    Operator(String),
    LeftParen,
    RightParen,
    EOF,
}

fn tokenize(s: &str) -> Vec<Token> {
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
            Token::Number(n) => {
                let n = Node::new(NodeType::NUM(*n));
                self.next();
                n
            }
            Token::LeftParen => {
                self.next();
                let n = self.parse_expr();
                match *self.token() {
                    Token::RightParen => {}
                    _ => {
                        eprintln!("開きカッコに対応する閉じカッコがありません");
                        process::exit(1);
                    }
                }
                self.next();
                n
            }
            _ => {
                eprintln!("数値ではありません");
                process::exit(1);
            }
        }
    }
}

fn generate(node: Box<Node>) {
    match node.value {
        NodeType::NUM(n) => {
            generate_li("t0".to_string(), n);
            generate_push("t0".to_string());
            return;
        }
        _ => {}
    }

    for child in node.children {
        generate(child);
    }

    match node.value {
        NodeType::ADD => {
            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  add t2, t1, t0");
            generate_push("t2".to_string());
        }
        NodeType::SUB => {
            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  sub t2, t1, t0");
            generate_push("t2".to_string());
        }
        NodeType::MUL => {
            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  mul t2, t1, t0");
            generate_push("t2".to_string());
        }
        NodeType::DIV => {
            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  div t2, t1, t0");
            generate_push("t2".to_string());
        }
        _ => {}
    }
}

fn generate_push(register: String) {
    println!("  addi sp, sp, -4");
    println!("  sw {}, 0(sp)", register);
}

fn generate_pop(register: String) {
    println!("  lw {}, 0(sp)", register);
    println!("  addi sp, sp, 4");
}

fn generate_li(register: String, n: i64) {
    println!("  addi {}, zero, {}", register, n);
}

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
