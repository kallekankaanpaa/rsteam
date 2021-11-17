#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error while parsin json: {0}")]
    JsonParser(#[from] serde_json::Error),
    #[error("error while parsin xml: {0}")]
    XMLParser(#[from] serde_xml_rs::Error),
    #[error("error with the http connection: {0}")]
    HttpClient(#[from] hyper::Error),
    #[error("error with http: {0}")]
    Http(#[from] hyper::http::Error),
    #[error("client error: {0}")]
    Client(String),
}

impl Error {
    pub(crate) fn client(reason: &str) -> Self {
        Self::Client(reason.to_owned())
    }
}
