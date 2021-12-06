use std::vec;

use super::interface::{Attribute, Element, Node, NodeData, TagKind, Token, TokenSink};

#[derive(Debug)]
pub struct DOMHandler {
    pub nodes: Vec<Node>,
    stack: Vec<Node>,
}

impl TokenSink for DOMHandler {
    fn process_token(&mut self, token: Token) {
        match token {
            Token::Characters(text) => self.append_node(create_text(text)),

            Token::Tag(t) => match t.kind {
                TagKind::StartTag => self.stack.push(create_element(&t.name, &t.attrs)),

                TagKind::EndTag => match self.stack.pop() {
                    Some(node) => self.append_node(node),
                    None => (),
                },
            },

            Token::ParseError(msg) => panic!("{}", msg),
        }
    }
}

impl DOMHandler {
    pub fn new() -> DOMHandler {
        DOMHandler {
            nodes: vec![],
            stack: vec![],
        }
    }

    fn append_node(&mut self, node: Node) {
        match self.stack.last() {
            Some(parent) => parent.children.borrow_mut().push(node),
            None => self.nodes.push(node),
        };
    }
}

fn create_text(text: String) -> Node {
    Node::new(NodeData::Text(text))
}

fn create_element(name: &String, attrs: &Vec<Attribute>) -> Node {
    Node::new(NodeData::Element(Element {
        name: String::from(name),
        attrs: Vec::clone(attrs),
    }))
}
