mod css_parser;
mod html_parser;

fn main() {
    let html = "
        <h1>Title</h1>
        <div id='answer' class=\"note\">
            <p>Hello <em>world</em>!</p>
        </div>
    ";

    let nodes = html_parser::parse(html);

    dbg!(nodes);

    let css = "
        h1,
        div#answer.note,
        note > p em";

    let stylesheet = css_parser::parse(css);

    dbg!(stylesheet);
}
