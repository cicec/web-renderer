use super::{
    buffer::Buffer,
    interface::{AttrValueKind, Attribute, State, TagKind, TagToken, Token, TokenSink},
};

pub struct Tokenizer<Sink> {
    pub sink: Sink,
    state: State,
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
        match self.state {
            State::Data => match char {
                '<' => {
                    self.state = State::TagOpen;
                    self.emit_chars();
                }
                c => self.push_char(c),
            },

            State::TagOpen => match char {
                '/' => self.state = State::EndTagOpen,
                c => match lower_ascii_letter(c) {
                    Some(c) => {
                        self.state = State::TagName;
                        self.create_tag(TagKind::StartTag, c);
                    }
                    None => (),
                },
            },

            State::EndTagOpen => match char {
                '>' => (),
                c => match lower_ascii_letter(c) {
                    Some(_) => {
                        self.state = State::TagName;
                        self.create_tag(TagKind::EndTag, c);
                    }
                    None => (),
                },
            },

            State::SelfClosingStartTag => match char {
                '>' => {
                    self.current_tag_self_closing = true;
                    self.state = State::Data;
                    self.emit_tag();
                }
                _ => (),
            },

            State::TagName => match char {
                '\t' | '\n' | '\x0C' | ' ' => self.state = State::BeforeAttributeName,
                '/' => self.state = State::SelfClosingStartTag,
                '>' => {
                    self.state = State::Data;
                    self.emit_tag();
                }
                c => self.push_tag_name(c.to_ascii_lowercase()),
            },

            State::BeforeAttributeName => match char {
                '\t' | '\n' | '\x0C' | ' ' => (),
                '/' => self.state = State::SelfClosingStartTag,
                '>' => {
                    self.state = State::Data;
                    self.emit_tag();
                }
                c => match lower_ascii_letter(c) {
                    Some(c) => {
                        self.state = State::AttributeName;
                        self.create_attribute(c);
                    }
                    None => (),
                },
            },

            State::AttributeName => match char {
                '\t' | '\n' | '\x0C' | ' ' => self.state = State::AfterAttributeName,
                '/' => self.state = State::SelfClosingStartTag,
                '=' => self.state = State::BeforeAttributeValue,
                '>' => {
                    self.state = State::Data;
                    self.emit_tag();
                }
                c => match lower_ascii_letter(c) {
                    Some(c) => {
                        self.push_attribute_name(c);
                    }
                    None => (),
                },
            },

            State::AfterAttributeName => match char {
                '\t' | '\n' | '\x0C' | ' ' => (),
                '/' => self.state = State::SelfClosingStartTag,
                '=' => self.state = State::BeforeAttributeValue,
                '>' => {
                    self.state = State::Data;
                    self.emit_tag();
                }
                c => match lower_ascii_letter(c) {
                    Some(c) => {
                        self.state = State::AttributeName;
                        self.create_attribute(c);
                    }
                    None => (),
                },
            },

            State::BeforeAttributeValue => match char {
                '\t' | '\n' | '\r' | '\x0C' | ' ' => (),
                '"' => self.state = State::AttributeValue(AttrValueKind::DoubleQuoted),
                '\'' => self.state = State::AttributeValue(AttrValueKind::SingleQuoted),
                '>' => {
                    self.state = State::Data;
                    self.emit_tag();
                }
                _ => self.state = State::AttributeValue(AttrValueKind::Unquoted),
            },

            State::AttributeValue(AttrValueKind::DoubleQuoted) => match char {
                '"' => self.state = State::AfterAttributeValueQuoted,
                c => self.push_attribute_value(c.to_ascii_lowercase()),
            },

            State::AttributeValue(AttrValueKind::SingleQuoted) => match char {
                '\'' => self.state = State::AfterAttributeValueQuoted,
                c => self.push_attribute_value(c.to_ascii_lowercase()),
            },

            State::AttributeValue(AttrValueKind::Unquoted) => match char {
                '\t' | '\n' | '\r' | '\x0C' | ' ' => self.state = State::BeforeAttributeName,
                '/' => self.state = State::SelfClosingStartTag,
                '>' => {
                    self.state = State::Data;
                    self.emit_tag();
                }
                c => self.push_attribute_value(c.to_ascii_lowercase()),
            },

            State::AfterAttributeValueQuoted => match char {
                '\t' | '\n' | '\r' | '\x0C' | ' ' => self.state = State::BeforeAttributeName,
                '/' => self.state = State::SelfClosingStartTag,
                '>' => {
                    self.state = State::Data;
                    self.emit_tag();
                }
                _ => (),
            },
        }
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
