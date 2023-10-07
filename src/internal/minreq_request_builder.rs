use super::{client::QueryConf, request::Authorization};

pub struct MinreqRequestBuilder<B: serde::Serialize> {
    bearer_token: String,
    path: String,
    needs_bearer: Authorization,
    body: Option<B>,
    query: Option<QueryConf>,
    additional_path: Option<String>,
    request_method: minreq::Method,
}

impl<B: serde::Serialize> MinreqRequestBuilder<B> {

    pub fn new(bearer_token: String, path: String, request_method: minreq::Method) -> Self {
        Self {
            bearer_token,
            path,
            needs_bearer: Authorization::Unnecessary,
            body: None,
            query: None,
            additional_path: None,
            request_method,
        }
    }

    /// bearer token will be needed for request
    pub fn with_bearer(mut self, token: Authorization) -> Self {
        self.needs_bearer = token;
        self
    }

    /// adds additional endpoint path after base endpoint path
    pub fn with_path(mut self, path: Option<String>) -> Self {
        self.additional_path = path;
        self
    }

    /// adds limit and page query to the request
    pub fn with_query(mut self, query: Option<QueryConf>) -> Self {
        self.query = query;
        self
    }

    /// adds json payload to the request
    pub fn with_body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }

    #[must_use]
    /// tries to build the http request
    pub fn build(self) -> minreq::Request {
        let mut request = minreq::Request::new(
            self.request_method,
            format!(
                "{}{}",
                self.path,
                self.additional_path.unwrap_or("".to_string())
            ),
        );

        // add optional bearer token
        if let Authorization::Required = self.needs_bearer {
            request = request.with_header("Authorization", self.bearer_token);
        }

        // add optional query
        if let Some(query) = self.query {
            if let Some(limit) = query.limit {
                request = request.with_param("limit", limit.to_string())
            }

            if let Some(page) = query.page {
                request = request.with_param("page", page.to_string())
            }
        }

        // add optional json body
        if let Some(body) = self.body {
            request = request
                .with_body(serde_json::to_string(&body).unwrap())
                .with_header("Content-Type", "application/json")
        }

        request
    }
}
