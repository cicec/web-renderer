use std::{cell::RefCell, vec};

use super::interface::{Attribute, Element, Node, NodeData, TagKind, Token, TokenSink};

#[derive(Debug)]
pub struct TreeBuilder {
    pub nodes: Vec<Node>,
    stack: Vec<Node>,
}

impl TreeBuilder {
    pub fn new() -> TreeBuilder {
        TreeBuilder {
            nodes: vec![],
            stack: vec![],
        }
    }
}

impl TokenSink for TreeBuilder {
    fn process_token(&mut self, token: Token) {
        match token {
            Token::Tag(t) => match t.kind {
                TagKind::StartTag => {
                    let node = create_element(&t.name, &t.attrs);

                    self.stack.push(node)
                }

                TagKind::EndTag => match self.stack.pop() {
                    Some(node) => {
                        match self.stack.last() {
                            Some(parent) => parent.children.borrow_mut().push(node),
                            None => self.nodes.push(node),
                        };
                    }
                    None => (),
                },
            },
        }
    }
}

fn create_element(name: &String, attrs: &Vec<Attribute>) -> Node {
    Node {
        data: NodeData::Element(Element {
            name: String::from(name),
            attrs: Vec::clone(attrs),
        }),
        children: RefCell::new(vec![]),
    }
}
