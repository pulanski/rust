use colour::{dark_green, yellow};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let url: &str =
        "https://newsapi.org/v2/top-headlines?country=us&apiKey=2ad9df3027ad4445a31c0f4ddcc9a9e4";
    let articles = get_articles(url)?;
    // dbg!(articles);

    render_articles(&articles);
    Ok(())
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        // dark_green!("> {:20}\n", );
        let text_width = 120;
        dark_green!("> {}\n", textwrap::fill(&i.title, text_width));
        yellow!("> {}\n\n", textwrap::fill(&i.url, text_width));
    }
}
