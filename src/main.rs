use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    country: Option<String>,
}

mod theme;

use dotenv::dotenv;
use newsapi::{Article, Country, Endpoint, NewsAPI};
use std::error::Error;

fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for i in articles {
        theme.print_text(&format!("`{}`", i.title()));
        theme.print_text(&format!("> *{}*", i.url()));
        theme.print_text("---");
    }
}

'''async fn fetch_news(country: Country, api_key: &str) -> Result<Vec<Article>, Box<dyn Error>> {
    let mut newsapi = NewsAPI::new(api_key);
    newsapi
        .endpoint(Endpoint::TopHeadlines)
        .country(country);

    let newsapi_response = newsapi.fetch_async().await?;
    Ok(newsapi_response.articles().to_vec())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    dotenv().ok();

    let api_key = std::env::var("API_KEY").map_err(|_| "Failed to get API_KEY. Make sure you have a .env file with an API_KEY variable".to_string())?;

    let country = args.country.map(|s| Country::from_str(&s)).transpose()?.unwrap_or(Country::Us);

    let articles = fetch_news(country, &api_key)?;
    render_articles(&articles);

    Ok(())
}'''
