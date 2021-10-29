mod html_parser;
mod tokenizer;

fn main() {
    let html = "
        <h1>Title</h1>
        <div id=\"answer\" class=\"note\">
            <p>Hello <em>world</em>!</p>
        </div>
    ";

    let nodes = html_parser::parse(html);

    for node in nodes {
        println!("{}", node);
    }
}
