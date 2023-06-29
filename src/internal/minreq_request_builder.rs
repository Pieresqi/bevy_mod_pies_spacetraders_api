use super::{
    client::{ClientError, QueryConf},
    request::RequestError,
};

/// first stage of minreq request builder
pub struct MinreqRequestBuilderUnready<'a, B: serde::Serialize> {
    bearer_token: Option<String>,
    path: String,
    needs_bearer: bool,
    body: Option<&'a B>,
    query: &'a Option<QueryConf>,
    additional_path: Option<&'a str>,
}

impl<'a, B: serde::Serialize> MinreqRequestBuilderUnready<'a, B> {
    /// out of supplied bearer token and base endpoint path builds itself
    pub fn new(bearer_token: Option<String>, path: String) -> Self {
        Self {
            bearer_token,
            path,
            needs_bearer: false,
            body: None,
            query: &None,
            additional_path: None,
        }
    }

    /// what method should the http request be (get, post, etc)
    pub fn with_method(self, method: minreq::Method) -> MinreqRequestBuilderReady<'a, B> {
        MinreqRequestBuilderReady {
            builder: self,
            request_method: method,
        }
    }
}

/// second and final stage of minreq request builder
pub struct MinreqRequestBuilderReady<'a, B: serde::Serialize> {
    builder: MinreqRequestBuilderUnready<'a, B>,
    request_method: minreq::Method,
}

impl<'a, B: serde::Serialize> MinreqRequestBuilderReady<'a, B> {
    /// bearer token will be needed for request
    pub fn needs_bearer(mut self) -> Self {
        self.builder.needs_bearer = true;
        self
    }

    /// adds additional endpoint path after base endpoint path
    pub fn with_path(mut self, path: &'a str) -> Self {
        self.builder.additional_path = Some(path);
        self
    }

    /// adds limit and page query to the request
    pub fn with_query(mut self, query: &'a Option<QueryConf>) -> Self {
        self.builder.query = query;
        self
    }

    /// adds json payload to the request
    pub fn with_body(mut self, body: &'a B) -> Self {
        self.builder.body = Some(body);
        self
    }

    /// tries to build the http request
    pub fn build(self) -> Result<minreq::Request, ClientError> {
        let mut request = minreq::Request::new(
            self.request_method,
            format!(
                "{}{}",
                self.builder.path,
                self.builder.additional_path.unwrap_or("")
            ),
        );

        // add optional bearer token
        if self.builder.needs_bearer {
            let Some(token) = &self.builder.bearer_token else {
                return Err(ClientError::Request(RequestError::BearerPrivateTokenNotSet))
            };

            request = request.with_header("Authorization", token);
        }

        // add optional query
        if let Some(query) = self.builder.query {
            if let Some(limit) = query.limit {
                request = request.with_param("limit", limit.to_string())
            }

            if let Some(page) = query.page {
                request = request.with_param("page", page.to_string())
            }
        }

        // add optional json body
        if let Some(body) = self.builder.body {
            match serde_json::to_string(body) {
                Ok(body) => {
                    request = request
                        .with_body(body)
                        .with_header("Content-Type", "application/json")
                }
                Err(error) => return Err(ClientError::Request(RequestError::Serde(error))),
            }
        }

        Ok(request)
    }
}
