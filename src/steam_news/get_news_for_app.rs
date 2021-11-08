use crate::utils::{u64_from_str, Result, AUTHORITY};
use crate::SteamClient;

use hyper::body::to_bytes;
use hyper::Uri;

use serde::Deserialize;
use serde_json::from_slice;

const PATH: &str = "/ISteamNews/GetNewsForApp/v0002/";

#[derive(Debug, Deserialize)]
pub struct NewsItem {
    #[serde(rename = "gid")]
    #[serde(deserialize_with = "u64_from_str")]
    pub id: u64,
    pub title: String,
    pub url: String,
    pub is_external_url: bool,
    pub author: String,
    pub contents: String,
    #[serde(rename = "feedlabel")]
    pub feed_label: String,
    #[serde(rename = "feedname")]
    pub feed_name: String,
    #[serde(rename = "feed_type")]
    pub feed_type: u32,
    /// As unix timestamp
    pub date: u32,
    #[serde(rename = "appid")]
    pub app_id: u32,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Appnews {
    #[serde(rename = "appid")]
    _app_id: u32,
    newsitems: Vec<NewsItem>,
    #[serde(rename = "count")]
    _count: u32,
}

#[derive(Debug, Deserialize)]
struct Response {
    appnews: Appnews,
}

impl SteamClient {
    pub async fn get_news_for_app(
        &self,
        app_id: u32,
        content_len: Option<u32>,
        end_date: Option<u32>,
        count: Option<u32>,
        feeds: Vec<String>,
        tags: Vec<String>,
    ) -> Result<Vec<NewsItem>> {
        let query_content = optional_query!(content_len, "maxlength");
        let query_date = optional_query!(end_date, "enddate");
        let query_count = optional_query!(count);
        let query_feeds = vec_query!(feeds);
        let query_tags = vec_query!(tags, "tags");

        let query = format!(
            "appid={}{}{}{}{}{}",
            app_id, query_content, query_date, query_count, query_feeds, query_tags
        );

        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;

        let raw_response = self.client.get(uri).await?;
        let raw_body = raw_response.into_body();
        let response: Response = from_slice(&to_bytes(raw_body).await?)?;

        Ok(response.appnews.newsitems)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn default_csgo_news() {
        let client = SteamClient::new();
        let news =
            block_on(client.get_news_for_app(730, None, None, None, vec![], vec![])).unwrap();

        assert_eq!(news.len(), 20);
    }
}
