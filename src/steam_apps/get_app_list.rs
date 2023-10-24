use crate::utils::{Result, AUTHORITY};
use crate::SteamClient;

use hyper::body::to_bytes;
use hyper::Uri;

use serde::Deserialize;
use serde_json::from_slice;

const PATH: &str = "/ISteamApps/GetAppList/v0002";

#[derive(Deserialize)]
pub struct App {
    #[serde(rename = "appid")]
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize)]
struct Apps {
    apps: Vec<App>,
}

#[derive(Deserialize)]
struct Response {
    applist: Apps,
}

impl SteamClient {
    /// Gets full list of all applications available in the steam store
    ///
    /// App list is very long so it's not recommended to query often
    pub async fn get_app_list(&self) -> Result<Vec<App>> {
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(PATH)
            .build()?;

        let raw_response = self.client.get(uri).await?;
        let raw_body = raw_response.into_body();
        let response: Response = from_slice(&to_bytes(raw_body).await?)?;

        Ok(response.applist.apps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn csgo_is_in_the_list() {
        let client = SteamClient::new();
        let apps = block_on(client.get_app_list()).unwrap();
        assert!(apps
            .iter()
            .any(|app| app.id == 730 && app.name == "Counter-Strike 2"));
    }
}
