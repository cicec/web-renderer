use self::{buffer::Buffer, interface::Node, tokenizer::Tokenizer, tree_builder::TreeBuilder};

mod buffer;
mod interface;
mod tokenizer;
mod tree_builder;

pub fn parse(input: &str) -> Vec<Node> {
    let tree_builder = TreeBuilder::new();
    let mut buffer = Buffer::new(input);
    let mut tokenizer = Tokenizer::new(tree_builder);

    tokenizer.feed(&mut buffer);

    tokenizer.sink.nodes
}
