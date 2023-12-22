use std::process;

use crate::tokenizer::Token;

#[derive(Clone, Debug)]
pub enum NodeType {
    ADD,
    SUB,
    MUL,
    DIV,
    NUM(i64),
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

    pub fn parse_expr(&mut self) -> Box<Node> {
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