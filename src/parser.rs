use std::process;

use crate::tokenizer::Token;

#[derive(Clone, Debug)]
pub enum NodeType {
    ADD,
    SUB,
    MUL,
    DIV,
    NUM(i64),
    LT,
    LTE,
    EQ,
    NEQ,
    ASSIGN,
    LVAR(String), // local variable
}

#[derive(Clone, Debug)]
pub struct Node {
    pub value: NodeType,
    pub children: Vec<Box<Node>>,
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
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
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

    pub fn parse_program(&mut self) -> Vec<Box<Node>> {
        let mut nodes = Vec::new();

        loop {
            match self.token() {
                Token::EOF => {
                    break;
                }
                _ => {
                    nodes.push(self.parse_expr());
                    match *self.token() {
                        Token::EOF => {}
                        Token::SEMICOLON => {
                            self.next();
                        }
                        _ => {
                            eprintln!("セミコロンがありません");
                            process::exit(1);
                        }
                    }
                }
            }
        }

        nodes
    }

    pub fn parse_expr(&mut self) -> Box<Node> {
        self.parse_assign()
    }

    pub fn parse_assign(&mut self) -> Box<Node> {
        let mut root = self.parse_equality();

        match self.token() {
            Token::Operator(op) => match op.as_str() {
                "=" => {
                    self.next();
                    root = Node::new(NodeType::ASSIGN)
                        .add_child(root)
                        .add_child(self.parse_assign())
                }
                _ => {}
            },
            _ => {}
        }

        root
    }

    pub fn parse_equality(&mut self) -> Box<Node> {
        let mut root = self.parse_relational();

        loop {
            match self.token() {
                Token::Operator(op) => match op.as_str() {
                    "==" => {
                        self.next();
                        root = Node::new(NodeType::EQ)
                            .add_child(root)
                            .add_child(self.parse_relational())
                    }
                    "!=" => {
                        self.next();
                        root = Node::new(NodeType::NEQ)
                            .add_child(root)
                            .add_child(self.parse_relational())
                    }
                    _ => break,
                },
                _ => break,
            }
        }

        root
    }

    pub fn parse_relational(&mut self) -> Box<Node> {
        let mut root = self.parse_add();

        loop {
            match self.token() {
                Token::Operator(op) => match op.as_str() {
                    "<" => {
                        self.next();
                        root = Node::new(NodeType::LT)
                            .add_child(root)
                            .add_child(self.parse_add())
                    }
                    "<=" => {
                        self.next();
                        root = Node::new(NodeType::LTE)
                            .add_child(root)
                            .add_child(self.parse_add())
                    }
                    ">" => {
                        self.next();
                        root = Node::new(NodeType::LT)
                            .add_child(self.parse_add())
                            .add_child(root)
                    }
                    ">=" => {
                        self.next();
                        root = Node::new(NodeType::LTE)
                            .add_child(self.parse_add())
                            .add_child(root)
                    }
                    _ => break,
                },
                _ => break,
            }
        }

        root
    }

    pub fn parse_add(&mut self) -> Box<Node> {
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
        let mut root = self.parse_unary();

        loop {
            match self.token() {
                Token::Operator(op) => match op.as_str() {
                    "*" => {
                        self.next();
                        root = Node::new(NodeType::MUL)
                            .add_child(root)
                            .add_child(self.parse_unary())
                    }
                    "/" => {
                        self.next();
                        root = Node::new(NodeType::DIV)
                            .add_child(root)
                            .add_child(self.parse_unary())
                    }
                    _ => break,
                },
                _ => break,
            }
        }

        root
    }

    fn parse_unary(&mut self) -> Box<Node> {
        match self.token() {
            Token::Operator(op) => match op.as_str() {
                "+" => {
                    self.next();
                    self.parse_primary()
                }
                "-" => {
                    self.next();
                    Node::new(NodeType::SUB)
                        .add_child(Node::new(NodeType::NUM(0)))
                        .add_child(self.parse_primary())
                }
                _ => {
                    return self.parse_primary();
                }
            },
            _ => {
                return self.parse_primary();
            }
        }
    }

    fn parse_primary(&mut self) -> Box<Node> {
        match self.token() {
            Token::Number(n) => {
                let n = Node::new(NodeType::NUM(*n));
                self.next();
                n
            }
            Token::Ident(s) => {
                let n = Node::new(NodeType::LVAR(s.clone()));
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
