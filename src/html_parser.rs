use crate::tokenizer::Tokenizer as T;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

pub enum Node {
    Text(String),
    Element(Element),
}

impl Node {
    fn element(tag: String, attrs: Attrs, children: Vec<Node>) -> Node {
        Node::Element(Element {
            tag,
            attrs,
            children,
        })
    }

    fn text(value: String) -> Node {
        Node::Text(value)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Node::Element(e) => {
                write!(f, "{}", e)
            }

            Node::Text(s) => {
                write!(f, "{{ text: {} }}", s)
            }
        }
    }
}

pub struct Element {
    tag: String,
    attrs: Attrs,
    children: Vec<Node>,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let tag = &self.tag;

        let attrs = &self
            .attrs
            .iter()
            .map(|(name, value)| format!("{}: {}", name, value))
            .collect::<Vec<_>>()
            .join(", ");

        let children = &self
            .children
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(
            f,
            "{{ tag: {}, attrs: [ {} ], children: [ {} ] }}",
            tag, attrs, children
        )
    }
}

type Attrs = HashMap<String, String>;

pub fn parse(input: &str) -> Vec<Node> {
    let mut t = T::new(input);

    parse_nodes(&mut t)
}

fn parse_nodes(t: &mut T) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();

    loop {
        t.consume_whitespace();

        if t.eof() || t.starts_with("</") {
            break;
        }

        nodes.push(parse_node(t));
    }

    nodes
}

fn parse_node(t: &mut T) -> Node {
    match t.next_char() {
        '<' => parse_element(t),
        _ => parse_text(t),
    }
}
fn parse_element(t: &mut T) -> Node {
    assert_eq!(t.consume_char(), '<');

    let tag = parse_identifier(t);
    let attrs = parse_attrs(t);

    assert_eq!(t.consume_char(), '>');

    let children = parse_nodes(t);

    assert_eq!(t.consume_char(), '<');
    assert_eq!(t.consume_char(), '/');
    assert_eq!(parse_identifier(t), tag);
    assert_eq!(t.consume_char(), '>');

    Node::element(tag, attrs, children)
}

fn parse_text(t: &mut T) -> Node {
    Node::text(t.consume_while(|c| c != '<'))
}

fn parse_attrs(t: &mut T) -> Attrs {
    let mut attrs: Attrs = HashMap::new();

    loop {
        t.consume_whitespace();

        if t.next_char() == '>' {
            break;
        }

        let (name, value) = parse_attribute(t);

        attrs.insert(name, value);
    }

    attrs
}

fn parse_attribute(t: &mut T) -> (String, String) {
    t.consume_whitespace();

    let name = parse_identifier(t);

    assert_eq!(t.consume_char(), '=');

    let value = parse_attr_value(t);

    (name, value)
}

fn parse_attr_value(t: &mut T) -> String {
    assert_eq!(t.consume_char(), '"');

    let value = t.consume_while(|c| c != '"');

    assert_eq!(t.consume_char(), '"');

    value
}

fn parse_identifier(t: &mut T) -> String {
    t.consume_while(|c| match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' => true,
        _ => false,
    })
}
