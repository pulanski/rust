use std::error::Error;

struct Articles {
    articles: Vec<Article>,
}

struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    dbg!(response);
    todo!()
}

fn main() {
    let url: &str =
        "https://newsapi.org/v2/top-headlines?country=us&apiKey=2ad9df3027ad4445a31c0f4ddcc9a9e4";
    let _articles = get_articles(url);
}
