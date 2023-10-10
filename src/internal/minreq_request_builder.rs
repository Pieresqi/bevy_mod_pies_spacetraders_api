use super::{client::QueryConf, request::Authorization};

#[derive(Debug)]
pub struct MinreqRequestBuilder<B: serde::Serialize> {
    auth_type: Authorization,
    body: Option<B>,
    query: QueryConf,
    additional_path: String,
    request_method: minreq::Method,
}

impl<B: serde::Serialize> MinreqRequestBuilder<B> {
    /// bearer token will be needed for request
    pub fn set_authorization(mut self, token: Authorization) -> Self {
        self.auth_type = token;
        self
    }

    /// adds additional endpoint path after base endpoint path
    pub fn set_additional_path<I: Into<String>>(mut self, path: I) -> Self {
        self.additional_path = path.into();
        self
    }

    /// adds limit and page query to the request
    pub fn set_query(mut self, query: QueryConf) -> Self {
        self.query = query;
        self
    }

    /// adds json payload to the request
    pub fn set_body(mut self, body: B) -> Self {
        self.body = Some(body);
        self
    }

    pub const fn new(method: minreq::Method, authorization: Authorization) -> Self {
        Self {
            auth_type: authorization,
            body: None,
            query: QueryConf {
                limit: None,
                page: None,
            },
            additional_path: String::new(),
            request_method: method,
        }
    }

    pub(crate) fn build(self, bearer_token: String, path: String) -> minreq::Request {
        let mut request =
            minreq::Request::new(self.request_method, path + self.additional_path.as_str())
                .with_timeout(33)
                .with_max_headers_size(10000)
                .with_max_status_line_length(1000);

        // add optional bearer token
        if let Authorization::Required = self.auth_type {
            request = request.with_header("Authorization", bearer_token);
        }

        // add optional query limit
        if let Some(limit) = self.query.limit {
            request = request.with_param("limit", limit.to_string());
        }

        // add optional query page
        if let Some(page) = self.query.page {
            request = request.with_param("page", page.to_string());
        }

        // add optional json body
        if let Some(body) = self.body {
            request = request
                .with_json(&body)
                .unwrap()
                .with_header("Content-Type", "application/json");
        }

        request
    }
}
