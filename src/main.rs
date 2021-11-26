mod html_parser;

fn main() {
    let html = "
        <h1>Title</h1>
        <div id='answer' class=\"note\" data-id=exp>
            <p>Hello <em>world</em>!</p>
        </div>
    ";

    let nodes = html_parser::parse(html);

    dbg!(nodes);
}
