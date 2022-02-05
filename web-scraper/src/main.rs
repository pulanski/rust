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
    let url: &str = "https://na.op.gg/summoner/userName=zhake";
    let _articles = get_articles(url);
}
