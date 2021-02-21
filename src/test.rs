pub const SECRET: &'static str = "SECRET";
pub const SERDE_ERROR: &'static str = "Failed to serialize something.";

pub type Connector = hyper::client::HttpConnector;

pub struct TestCase {
    pub server: httpmock::MockServer,
    pub client: std::sync::Arc<chatex_sdk_rust::ChatexClient<Connector>>,
}

impl Default for TestCase {
    fn default() -> TestCase {
        let server = httpmock::MockServer::start();
        let client = std::sync::Arc::new(chatex_sdk_rust::ChatexClient::new(
            hyper::client::HttpConnector::new(),
            server.base_url().parse().expect("Invalid url"),
            SECRET.to_owned()));
        TestCase {
            server,
            client,
        }
    }
}

impl TestCase {
    pub fn mock_access_token(&self) -> httpmock::MockRef {
        self.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .header("Authorization", "Bearer SECRET")
                .header("Accept", "application/json")
                .path("/auth/access-token");
            let access_token = serde_json::to_string(
                &chatex_sdk_rust::models::AccessToken::default()).expect(SERDE_ERROR);
            then.status(200)
                .header("Content-Type", "application/json")
                .body(access_token);
        })
    }
}
