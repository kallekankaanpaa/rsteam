use hyper::client::HttpConnector;
use hyper::Body;
use hyper::Client as HyperClient;

use hyper_tls::HttpsConnector;

pub struct SteamClient {
    pub(crate) client: HyperClient<HttpsConnector<HttpConnector>, Body>,
    pub(crate) api_key: String,
}

impl SteamClient {
    pub fn new(key: &str) -> Self {
        let https_connector = HttpsConnector::new();

        SteamClient {
            client: HyperClient::builder().build::<_, Body>(https_connector),
            api_key: key.to_owned(),
        }
    }
}
