use std::vec;

use self::interface::{CSSValue, Declaration, Number, Rule, Selector, Stylesheet};
use self::tokenizer::{Token, Tokenizer};

mod interface;
mod tokenizer;

pub fn parse(input: &str) -> Stylesheet {
    let tokenizer = Tokenizer::new(input);
    let mut parser = Parser::new(tokenizer);

    parser.parse_stylesheet()
}

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer) -> Parser {
        Parser { tokenizer }
    }

    fn parse_stylesheet(&mut self) -> Stylesheet {
        let mut rules: Vec<Rule> = vec![];

        loop {
            match self.tokenizer.peek() {
                Token::EOS => break,

                _ => {
                    let selectors = self.parse_selectors();
                    let declarations = self.parse_declarations();

                    rules.push(Rule {
                        selectors,
                        declarations,
                    })
                }
            }
        }

        Stylesheet { rules }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors: Vec<Selector> = vec![];

        loop {
            match self.tokenizer.peek() {
                Token::EOS => break,

                Token::Comma | Token::OpenCurly => {
                    self.tokenizer.next();
                    selectors.push(self.parse_selector());
                }

                _ => {
                    selectors.push(self.parse_selector());
                }
            }
        }

        selectors
    }

    fn parse_selector(&mut self) -> Selector {
        let mut selector = Selector::new();

        for i in 0.. {
            match self.tokenizer.peek() {
                Token::Comma | Token::OpenCurly | Token::EOS => break,

                _ => (),
            }

            match self.tokenizer.next() {
                Token::Hash(identifier) => {
                    selector.identifier = Some(identifier);
                }

                Token::Dot => {
                    match self.tokenizer.next() {
                        Token::Ident(class_name) => {
                            selector
                                .class_names
                                .get_or_insert_with(|| vec![])
                                .push(class_name);
                        }

                        t => panic!("Expected token: {:?}", t),
                    };
                }

                Token::Ident(tag_name) => {
                    selector.tag_name = Some(tag_name);
                }

                Token::CloseAngle => {
                    selector.child = Some(Box::new(self.parse_selector()));
                }

                Token::WhiteSpace(_) => {
                    if i != 0 && self.is_selector_start() {
                        selector.descendant = Some(Box::new(self.parse_selector()));
                    }
                }

                t => panic!("Expected token: {:?}", t),
            }
        }

        selector
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations: Vec<Declaration> = vec![];

        declarations
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.tokenizer.peek() {
                Token::WhiteSpace(_) => {
                    self.tokenizer.next();
                }

                _ => break,
            }
        }
    }

    fn is_selector_start(&mut self) -> bool {
        match self.tokenizer.peek() {
            Token::Ident(_) | Token::Dot | Token::Hash(_) => true,
            _ => false,
        }
    }

    fn unexpected_token_error(&self, expected: &Token, unexpected: &Token) {
        panic!(
            "Expected token: \"{:?}\", but found \"{:?}\"",
            expected, unexpected
        );
    }
}
