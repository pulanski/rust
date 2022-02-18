use csv::Writer;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use std::fs::OpenOptions;

async fn test(i: &i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://na.op.gg/leaderboards/tier?page={}&region=na", i);
    let response = reqwest::get(&url).await?.text().await?;
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("test2.csv")
        .unwrap();
    dbg!(&file);
    let mut wtr = Writer::from_writer(file);

    let document = Document::from(response.as_str());

    for node in document.find(Name("article")) {
        let name = match node.find(Name("h3")).next() {
            Some(h3) => h3.find(Name("a")).next().unwrap().text(),
            None => "".to_string(),
        };
        let price = node
            .find(Attr("class", "price_color"))
            .next()
            .unwrap()
            .text();

        println!("{:#?} ", url);
        wtr.write_record(&[&url, &price, &name]).unwrap();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handles: std::vec::Vec<_> = Vec::new();
    for i in 1..50 {
        let job = tokio::spawn(async move { test(&i).await });
        handles.push(job);
    }

    let mut results = Vec::new();
    for job in handles {
        results.push(job.await);
    }

    Ok(())
}
