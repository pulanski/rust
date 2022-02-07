fn main() {
    use scraper::Html;

    let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
    "#;

    let document = Html::parse_document(html);
    dbg!(document);
}
