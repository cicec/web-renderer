use self::{buffer::Buffer, dom_handler::DOMHandler, interface::Node, tokenizer::Tokenizer};

mod buffer;
mod dom_handler;
mod interface;
mod tokenizer;

pub fn parse(input: &str) -> Vec<Node> {
    let handler = DOMHandler::new();
    let mut buffer = Buffer::new(input);
    let mut tokenizer = Tokenizer::new(handler);

    tokenizer.feed(&mut buffer);
    tokenizer.sink.nodes
}
