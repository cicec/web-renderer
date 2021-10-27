use crate::tokenizer::Tokenizer as T;
use std::collections::HashMap;

pub enum Node {
    Text(String),
    Element(Element),
}

pub struct Element {
    pub tag: String,
    pub attrs: Attrs,
    pub children: Vec<Node>,
}

type Attrs = HashMap<String, String>;

pub fn parse(input: &str) -> Vec<Node> {
    let mut t = T {
        pos: 0,
        input: input.to_string(),
    };

    parse_nodes(&mut t)
}

pub fn print(nodes: Vec<Node>) {
    for node in nodes {
        match node {
            Node::Text(v) => {
                println!("text: {}", v);
            }
            Node::Element(e) => {
                let mut attrs: Vec<String> = Vec::new();

                for (name, value) in e.attrs {
                    attrs.push(format!("{}: {}", name, value));
                }

                println!("tag: {}, attrs: [ {} ]", e.tag, attrs.join(", "));

                if e.children.len() > 0 {
                    print(e.children);
                }

                println!("tag_end: {}", e.tag);
            }
        };
    }
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

    element(tag, attrs, children)
}

fn parse_text(t: &mut T) -> Node {
    text(t.consume_while(|c| c != '<'))
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
