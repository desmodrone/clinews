use clap::Parser;
use std::str::FromStr;

mod theme;

use dotenv::dotenv;
use newsapi::{Article, Country, Endpoint, NewsAPI};
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Country to fetch news from (us, gb, ca, au, in, jp, cn, de, fr)")]
    country: Option<String>,
    #[arg(short, long, default_value = "top-headlines")]
    endpoint: String,
    #[arg(short, long)]
    query: Option<String>,
}

fn render_articles(articles: &Vec<Article>, theme: &termimad::MadSkin) {
    theme.print_text("# Top headlines\n\n");
    for (i, article) in articles.iter().enumerate() {
        theme.print_text(&format!("**{}.** `{}`", i + 1, article.title()));
        theme.print_text(&format!("> *{}*", article.url()));
        theme.print_text("---");
    }
}

fn get_user_choice(articles: &Vec<Article>) -> Result<Option<String>, Box<dyn Error>> {
    println!("\nEnter a number to open an article in your browser, or just press Enter to quit:");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let choice = buffer.trim();
    if choice.is_empty() {
        return Ok(None);
    }
    let choice: usize = choice.parse()?;
    if choice > 0 && choice <= articles.len() {
        Ok(Some(articles[choice - 1].url().to_string()))
    } else {
        Err("Invalid number".into())
    }
}

async fn fetch_news(
    country: Country,
    endpoint: &str,
    query: &Option<String>,
    api_key: &str,
) -> Result<Vec<Article>, Box<dyn Error>> {
    let mut newsapi = NewsAPI::new(api_key);
    let endpoint = match endpoint {
        "top-headlines" => Endpoint::TopHeadlines,
        "everything" => Endpoint::Everything,
        _ => return Err("Invalid endpoint".into()),
    };
    newsapi.endpoint(endpoint).country(country);
    if let Some(query) = query {
        newsapi.query(query);
    }

    let newsapi_response = newsapi.fetch_async().await?;
    Ok(newsapi_response.articles().to_vec())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    dotenv().ok();

    let api_key = std::env::var("API_KEY")
        .map_err(|_| {
            "Failed to get API_KEY. Make sure you have a .env file with an API_KEY variable"
                .to_string()
        })?;

    let country = args.country.map(|s| Country::from_str(&s)).transpose()?.unwrap_or(Country::Us);

    let articles = fetch_news(country, &args.endpoint, &args.query, &api_key).await?;
    let theme = theme::default();

    loop {
        render_articles(&articles, &theme);

        if let Some(url) = get_user_choice(&articles)? {
            webbrowser::open(&url)?;
        } else {
            break;
        }
    }

    Ok(())
}