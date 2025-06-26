use serde::Deserialize;
use url::Url;

const BASE_URL: &str = "https://newsapi.org/v2";

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed(#[from] ureq::Error),
    #[error("Failed converting response to string")]
    FailedResponseToString(#[from] std::io::Error),
    #[error("Article Parsing failed")]
    ArticleParseFailed(#[from] serde_json::Error),
    #[error("Url parsing failed")]
    UrlParsing(#[from] url::ParseError),
    #[error("Request failed: {0}")]
    BadRequest(&'static str),
    #[error("Async request failed")]
    #[cfg(feature = "async")]
    AsyncRequestFailed(#[from] reqwest::Error)
}

#[derive(Deserialize, Debug)]
pub struct NewsAPIResponse {
    status: String,
    articles: Vec<Article>,
    code: Option<String>,
}

impl NewsAPIResponse {
    pub fn articles(&self) -> &Vec<Article> {
        &self.articles
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Article {
    title: String,
    url: String,
}

impl Article {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

use std::str::FromStr;

pub enum Endpoint {
    TopHeadlines,
    Everything,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadlines => "top-headlines".to_string(),
            Self::Everything => "everything".to_string(),
        }
    }
}

impl FromStr for Endpoint {
    type Err = NewsApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top-headlines" => Ok(Self::TopHeadlines),
            "everything" => Ok(Self::Everything),
            _ => Err(NewsApiError::BadRequest("Invalid endpoint")),
        }
    }
}

pub enum Country {
    Us,
    Gb,
    Ca,
    Au,
    In,
    Jp,
    Cn,
    De,
    Fr,
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::Us => "us".to_string(),
            Self::Gb => "gb".to_string(),
            Self::Ca => "ca".to_string(),
            Self::Au => "au".to_string(),
            Self::In => "in".to_string(),
            Self::Jp => "jp".to_string(),
            Self::Cn => "cn".to_string(),
            Self::De => "de".to_string(),
            Self::Fr => "fr".to_string(),
        }
    }
}

impl FromStr for Country {
    type Err = NewsApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "us" => Ok(Self::Us),
            "gb" => Ok(Self::Gb),
            "ca" => Ok(Self::Ca),
            "au" => Ok(Self::Au),
            "in" => Ok(Self::In),
            "jp" => Ok(Self::Jp),
            "cn" => Ok(Self::Cn),
            "de" => Ok(Self::De),
            "fr" => Ok(Self::Fr),
            _ => Err(NewsApiError::BadRequest("Invalid country")),
        }
    }
}

pub struct NewsAPI {
    api_key: String,
    endpoint: Endpoint,
    country: Country,
    query: Option<String>,
}

impl NewsAPI {
    pub fn new(api_key: &str) -> NewsAPI {
        NewsAPI {
            api_key: api_key.to_string(),
            endpoint: Endpoint::TopHeadlines,
            country: Country::Us,
            query: None,
        }
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsAPI {
        self.endpoint = endpoint;
        self
    }

    pub fn country(&mut self, country: Country) -> &mut NewsAPI {
        self.country = country;
        self
    }

    pub fn query(&mut self, query: &str) -> &mut NewsAPI {
        self.query = Some(query.to_string());
        self
    }

    fn prepare_url(&self) -> Result<String, NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut()
            .unwrap()
            .push(&self.endpoint.to_string());

        let mut pairs = vec![];

        match self.endpoint {
            Endpoint::TopHeadlines => {
                pairs.push(format!("country={}", self.country.to_string()));
            }
            Endpoint::Everything => {
                if let Some(q) = &self.query {
                    pairs.push(format!("q={}", q));
                } else {
                    return Err(NewsApiError::BadRequest("Query is required for 'everything' endpoint"));
                }
            }
        }

        url.set_query(Some(&pairs.join("&")));

        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<NewsAPIResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let req = ureq::get(&url).set("Authorization", &self.api_key);
        let response: NewsAPIResponse = req.call()?.into_json()?;
        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_err(response.code)),
        }
    }

    #[cfg(feature = "async")]
    pub async fn fetch_async(&self) -> Result<NewsAPIResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let client = reqwest::Client::builder()
            .user_agent("clinews-app")
            .build()
            .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;
        let request = client
            .request(reqwest::Method::GET, url)
            .header("Authorization", &self.api_key)
            .build()
            .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        let response_text = client
            .execute(request)
            .await?
            .text()
            .await
            .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        let response: NewsAPIResponse = serde_json::from_str(&response_text)
            .map_err(|e| NewsApiError::ArticleParseFailed(e))?;

        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_err(response.code)),
        }
    }
}

fn map_response_err(code: Option<String>) -> NewsApiError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => NewsApiError::BadRequest("Your API key has been disabled"),
            _ => NewsApiError::BadRequest("Unknown error"),
        }
    } else {
        NewsApiError::BadRequest("Unknown error")
    }
}



