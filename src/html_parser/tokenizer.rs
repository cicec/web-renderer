use super::{
    buffer::Buffer,
    interface::{AttrValueKind, Attribute, State, TagKind, TagToken, Token, TokenSink},
};

use self::AttrValueKind::{DoubleQuoted, SingleQuoted, Unquoted};
use self::TagKind::{EndTag, StartTag};

macro_rules! go {
    ($me:ident to $s:ident) => {{
        $me.state = State::$s;
    }};

    ($me:ident to $s:ident $k1:expr) => {{
        $me.state = State::$s($k1);
    }};

    ($me:ident $a:tt; $($rest:tt)*) => {{
        $me.$a();
        go!($me $($rest)*);
    }};

    ($me:ident $a:tt $b:tt; $($rest:tt)*) => {{
        $me.$a($b);
        go!($me $($rest)*);
    }};

    ($me:ident $a:tt $b:tt $c:tt; $($rest:tt)*) => {{
        $me.$a($b, $c);
        go!($me $($rest)*);
    }};
}

pub struct Tokenizer<Sink> {
    pub sink: Sink,
    state: State,
    current_char: char,
    current_chars: String,
    current_tag_kind: TagKind,
    current_tag_name: String,
    current_tag_self_closing: bool,
    current_tag_attrs: Vec<Attribute>,
    current_attr_name: String,
    current_attr_value: String,
}

impl<Sink: TokenSink> Tokenizer<Sink> {
    pub fn new(sink: Sink) -> Tokenizer<Sink> {
        Tokenizer {
            sink,
            state: State::Data,
            current_char: '\0',
            current_chars: String::new(),
            current_tag_kind: TagKind::StartTag,
            current_tag_name: String::new(),
            current_tag_self_closing: false,
            current_tag_attrs: Vec::new(),
            current_attr_name: String::new(),
            current_attr_value: String::new(),
        }
    }

    pub fn feed(&mut self, buffer: &mut Buffer) {
        while !buffer.is_empty() {
            let char = buffer.next();
            self.step(char);
        }
    }

    pub fn step(&mut self, char: char) {
        self.current_char = char;

        match self.state {
            State::Data => match char {
                '<' => go!(self emit_chars; to TagOpen),
                c => self.push_char(c),
            },

            State::TagOpen => match char {
                '/' => go!(self to EndTagOpen),
                c => match lower_ascii_letter(c) {
                    Some(c) => go!(self create_tag StartTag c; to TagName),
                    None => self.emit_error(),
                },
            },

            State::EndTagOpen => match char {
                '>' => self.emit_error(),
                c => match lower_ascii_letter(c) {
                    Some(c) => go!(self create_tag EndTag c; to TagName),
                    None => self.emit_error(),
                },
            },

            State::SelfClosingStartTag => match char {
                '>' => {
                    self.current_tag_self_closing = true;
                    go!(self emit_tag; to Data);
                }
                _ => self.emit_error(),
            },

            State::TagName => match char {
                '\t' | '\n' | '\x0C' | ' ' => go!(self to BeforeAttributeName),
                '/' => go!(self emit_chars; to SelfClosingStartTag),
                '>' => go!(self emit_tag; to Data),
                c => self.push_tag_name(c.to_ascii_lowercase()),
            },

            State::BeforeAttributeName => match char {
                '\t' | '\n' | '\x0C' | ' ' => (),
                '/' => go!(self to SelfClosingStartTag),
                '>' => go!(self emit_tag; to Data),
                c => match lower_ascii_letter(c) {
                    Some(c) => go!(self create_attribute c; to AttributeName),
                    None => self.emit_error(),
                },
            },

            State::AttributeName => match char {
                '\t' | '\n' | '\x0C' | ' ' => go!(self to AfterAttributeName),
                '/' => go!(self to SelfClosingStartTag),
                '=' => go!(self to BeforeAttributeValue),
                '>' => go!(self emit_tag; to Data),
                c => match lower_ascii_letter(c) {
                    Some(c) => self.push_attribute_name(c),
                    None => self.emit_error(),
                },
            },

            State::AfterAttributeName => match char {
                '\t' | '\n' | '\x0C' | ' ' => (),
                '/' => go!(self to SelfClosingStartTag),
                '=' => go!(self to BeforeAttributeValue),
                '>' => go!(self emit_tag; to Data),
                c => match lower_ascii_letter(c) {
                    Some(c) => go!(self create_attribute c; to AttributeName),
                    None => self.emit_error(),
                },
            },

            State::BeforeAttributeValue => match char {
                '\t' | '\n' | '\r' | '\x0C' | ' ' => (),
                '"' => go!(self to AttributeValue DoubleQuoted),
                '\'' => go!(self to AttributeValue SingleQuoted),
                '>' => go!(self emit_tag; to Data),
                _ => go!(self to AttributeValue Unquoted),
            },

            State::AttributeValue(AttrValueKind::DoubleQuoted) => match char {
                '"' => go!(self to AfterAttributeValueQuoted),
                c => self.push_attribute_value(c.to_ascii_lowercase()),
            },

            State::AttributeValue(AttrValueKind::SingleQuoted) => match char {
                '\'' => go!(self to AfterAttributeValueQuoted),
                c => self.push_attribute_value(c.to_ascii_lowercase()),
            },

            State::AttributeValue(AttrValueKind::Unquoted) => match char {
                '\t' | '\n' | '\r' | '\x0C' | ' ' => go!(self to BeforeAttributeName),
                '/' => go!(self to SelfClosingStartTag),
                '>' => go!(self emit_tag; to Data),
                c => self.push_attribute_value(c.to_ascii_lowercase()),
            },

            State::AfterAttributeValueQuoted => match char {
                '\t' | '\n' | '\r' | '\x0C' | ' ' => go!(self to BeforeAttributeName),
                '/' => go!(self to SelfClosingStartTag),
                '>' => go!(self emit_tag; to Data),
                _ => (),
            },
        }
    }

    fn emit_error(&mut self) {
        let msg = format!(
            "Unexpected character '{}' in state {:?}",
            self.current_char, self.state
        );

        self.process_token(Token::ParseError(msg))
    }

    fn emit_chars(&mut self) {
        self.process_token(Token::Characters(String::from(&self.current_chars)));
        self.current_chars.clear();
    }

    fn emit_tag(&mut self) {
        self.finish_attribute();

        let token = Token::Tag(TagToken {
            name: String::from(&self.current_tag_name),
            kind: self.current_tag_kind,
            self_closing: self.current_tag_self_closing,
            attrs: Vec::clone(&self.current_tag_attrs),
        });

        self.process_token(token)
    }

    fn push_char(&mut self, c: char) {
        self.current_chars.push(c)
    }

    fn create_tag(&mut self, kind: TagKind, c: char) {
        self.discard_tag();
        self.current_tag_name.push(c);
        self.current_tag_kind = kind;
    }

    fn push_tag_name(&mut self, c: char) {
        self.current_tag_name.push(c)
    }

    fn discard_tag(&mut self) {
        self.current_tag_name.clear();
        self.current_tag_attrs.clear();
        self.current_tag_self_closing = false;
    }

    fn create_attribute(&mut self, c: char) {
        self.finish_attribute();

        self.current_attr_name.push(c);
    }

    fn push_attribute_name(&mut self, c: char) {
        self.current_attr_name.push(c);
    }

    fn push_attribute_value(&mut self, c: char) {
        self.current_attr_value.push(c);
    }

    fn finish_attribute(&mut self) {
        if self.current_attr_name.is_empty() {
            return;
        }

        self.current_tag_attrs.push(Attribute {
            name: String::from(&self.current_attr_name),
            value: String::from(&self.current_attr_value),
        });

        self.current_attr_name.clear();
        self.current_attr_value.clear();
    }

    fn process_token(&mut self, token: Token) {
        self.sink.process_token(token);
    }
}

pub fn lower_ascii_letter(c: char) -> Option<char> {
    match c {
        'a'..='z' => Some(c),
        'A'..='Z' => Some((c as u8 - b'A' + b'a') as char),
        _ => None,
    }
}
