use std::cell::RefCell;

#[derive(Debug)]
pub enum Token {
    Tag(TagToken),
    Characters(String),
    ParseError(String),
}

pub trait TokenSink {
    fn process_token(&mut self, token: Token);
}

#[derive(Debug)]
pub enum State {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValue(AttrValueKind),
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
}

#[derive(Debug, Copy, Clone)]
pub enum TagKind {
    StartTag,
    EndTag,
}

#[derive(Debug, Clone)]
pub struct TagToken {
    pub kind: TagKind,
    pub name: String,
    pub self_closing: bool,
    pub attrs: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub enum AttrValueKind {
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
}

#[derive(Debug)]
pub struct Node {
    pub data: NodeData,
    pub children: RefCell<Vec<Node>>,
}

impl Node {
    pub fn new(data: NodeData) -> Node {
        Node {
            data,
            children: RefCell::new(vec![]),
        }
    }
}

#[derive(Debug)]
pub enum NodeData {
    Element(Element),
    Text(String),
}

#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub attrs: Vec<Attribute>,
}
