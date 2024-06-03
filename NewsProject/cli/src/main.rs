mod theme;

use dotenv::dotenv;
use newsapi::{Article, Country, Endpoint, NewsAPI};
use std::error::Error;

fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for i in articles {
        theme.print_text(&format!("`{}`", i.title()));
        if let Some(description) = i.description() {
            print!("Description: ");
            theme.print_text(&format!("{}\n\n", description));
        }
        theme.print_text(&format!("> *{}*", i.url()));
        theme.print_text("---");
    }
}

#[tokio::main] //for making the main function async
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = std::env::var("API_KEY")?;

    let mut newsapi = NewsAPI::new(&api_key);
    newsapi
        .endpoint(Endpoint::TopHeadlines)
        .country(Country::India);

    let newsapi_response = newsapi.fetch_async().await?; // ? for error handling
    render_articles(&newsapi_response.articles());

    Ok(())
}
