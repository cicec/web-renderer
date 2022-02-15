use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Token {
    WhiteSpace(String),
    Ident(String),
    Comment(String),
    /// e.g #my-idx, #ffffff
    Hash(String),
    /// e.g 42
    Number(String),
    /// e.g "SF Pro Display"
    String(String),
    OpenCurly,
    CloseCurly,
    OpenBracket,
    CloseBracket,
    Colon,
    SemiColon,
    Dot,
    CloseAngle,
    Comma,
    Asterisk,
    Percentage,
    /// END of source
    EOS,
}

pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
    current: Option<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer {
            source: input.chars().peekable(),
            current: None,
        }
    }

    pub fn peek(&mut self) -> &Token {
        if self.current.is_none() {
            self.current = Some(self.consume_token());
        }

        self.current.as_ref().unwrap()
    }

    pub fn next(&mut self) -> Token {
        match self.current.take() {
            Some(t) => t,
            _ => self.consume_token(),
        }
    }

    fn consume_token(&mut self) -> Token {
        let current_char = match self.source.peek() {
            None => return Token::EOS,
            Some(c) => c,
        };

        match current_char {
            '\t' | '\n' | '\r' | '\x0C' | ' ' => Token::WhiteSpace(self.consume_whitespace()),
            c if is_valid_start_ident(*c) => Token::Ident(self.consume_identifier()),
            _ => match self.source.next() {
                Some(c) => match c {
                    '#' => Token::Hash(self.consume_identifier()),
                    '{' => Token::OpenCurly,
                    '}' => Token::CloseCurly,
                    '(' => Token::OpenBracket,
                    ')' => Token::CloseBracket,
                    ':' => Token::Colon,
                    ';' => Token::SemiColon,
                    ',' => Token::Comma,
                    '>' => Token::CloseAngle,
                    '.' => Token::Dot,
                    '*' => Token::Asterisk,
                    '%' => Token::Percentage,
                    _ => Token::EOS,
                },
                None => Token::EOS,
            },
        }
    }

    fn consume_identifier(&mut self) -> String {
        self.consume_while(is_valid_ident)
    }

    fn consume_number(&mut self) -> String {
        self.consume_while(|c| match c {
            '0'..='9' => true,
            _ => false,
        })
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(char::is_whitespace)
    }

    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();

        while self.source.peek().map_or(false, |c| condition(*c)) {
            result.push(self.source.next().unwrap())
        }

        result
    }
}

fn is_valid_start_ident(c: char) -> bool {
    is_letter(c) || is_non_ascii(c) || c == '_'
}

fn is_valid_ident(c: char) -> bool {
    is_valid_start_ident(c) || c.is_digit(10) || c == '-'
}

fn is_letter(c: char) -> bool {
    is_upper_letter(c) || is_lower_letter(c)
}

fn is_upper_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn is_lower_letter(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

fn is_non_ascii(c: char) -> bool {
    c >= '\u{0080}'
}
