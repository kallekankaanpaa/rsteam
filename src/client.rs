use hyper::client::HttpConnector;
use hyper::Body;
use hyper::Client as HyperClient;

use hyper_rustls::HttpsConnector;

/// Client to make API requests easily.
///
/// Client can be constructed with or without an API key. Only subset
/// of APIs are available for the client.
pub struct SteamClient {
    pub(crate) client: HyperClient<HttpsConnector<HttpConnector>, Body>,
    pub(crate) api_key: Option<String>,
}

impl SteamClient {
    /// Create a client with an API key.
    ///
    /// Client with API key can use all available APIs.
    pub fn with_api_key(key: &str) -> Self {
        let https_connector = HttpsConnector::with_native_roots();

        SteamClient {
            client: HyperClient::builder().build::<_, Body>(https_connector),
            api_key: Some(key.to_owned()),
        }
    }

    /// Create a client without an API key.
    ///
    /// Client without an API key can only use a subset of the APIs.
    pub fn new() -> Self {
        let https_connector = HttpsConnector::with_native_roots();

        SteamClient {
            client: HyperClient::builder().build::<_, Body>(https_connector),
            api_key: None,
        }
    }
}
