// use colour::{dark_green, yellow};
use dotenv::dotenv;
use newsapi::{get_articles, Articles};
use std::error::Error;

mod theme;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;

    let url: &str = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";
    let url = format!("{}{}", &url, api_key);
    let articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}

fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for i in &articles.articles {
        theme.print_text(&format!("`{}`", i.title));
        theme.print_text(&format!("> *{}*", i.url));
        theme.print_text("---");
    }
}
