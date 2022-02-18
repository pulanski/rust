use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    FailedRequest(ureq::Error),
    #[error("Failed converting response into string")]
    FailedResponseToString(std::io::Error),
    #[error("Failed parsing article")]
    FailedArticleParse(serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| NewsApiError::FailedRequest(e))?
        .into_string()
        .map_err(|e| NewsApiError::FailedResponseToString(e))?;
    let articles: Articles =
        serde_json::from_str(&response).map_err(|e| NewsApiError::FailedArticleParse(e))?;

    Ok(articles)
}
