fn get_articles(url: &str) {
    let response = ureq::get(url);
}

fn main() {
    let url: &str = "GET https://newsapi.org/v2/top-headlines?country=us&apiKey=2ad9df3027ad4445a31c0f4ddcc9a9e4";
    let articles = get_articles(url);
}
