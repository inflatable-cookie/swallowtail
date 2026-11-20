use crate::TransportError;
use std::time::Duration;
use url::Url;
use zeroize::Zeroizing;

/// Default bound for one buffered response body or SSE event buffer.
pub const DEFAULT_RESPONSE_LIMIT: usize = 4 * 1024 * 1024;
/// Default bound on establishing a connection.
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// Default bound on one finite (non-streaming) exchange.
pub const DEFAULT_TRANSFER_TIMEOUT: Duration = Duration::from_secs(120);

/// HTTP method used by one bounded request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Method {
    /// Read-only request with no body.
    Get,
    /// Body-carrying request.
    Post,
    /// Whole-resource replacement request.
    Put,
    /// Partial-update request.
    Patch,
    /// Deliberate removal request.
    Delete,
}

impl Method {
    pub(crate) const fn curl_verb(self) -> Option<&'static str> {
        match self {
            Self::Get | Self::Post => None,
            Self::Put => Some("PUT"),
            Self::Patch => Some("PATCH"),
            Self::Delete => Some("DELETE"),
        }
    }
}

/// A request header whose value must not survive in ordinary memory.
///
/// The value is held in [`Zeroizing`] storage and the formatted header line
/// handed to libcurl is zeroized as soon as libcurl copies it.
#[derive(Clone)]
pub struct SecretHeader {
    name: String,
    value: Zeroizing<String>,
}

impl std::fmt::Debug for SecretHeader {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SecretHeader")
            .field("name", &self.name)
            .field("value", &"<redacted>")
            .finish()
    }
}

impl SecretHeader {
    /// Builds a secret header from an already-prefixed value.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::CredentialInvalid`] when the value is empty or
    /// contains a line break that could split the header.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Result<Self, TransportError> {
        let name = name.into();
        let value = Zeroizing::new(value.into());
        validate_secret(&name, &value)?;
        Ok(Self { name, value })
    }

    /// Builds a `authorization: Bearer <credential>` header.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::CredentialInvalid`] when the credential is not
    /// UTF-8, is empty, or contains a line break.
    pub fn bearer(credential: Vec<u8>) -> Result<Self, TransportError> {
        Self::prefixed("authorization", "Bearer ", credential)
    }

    /// Builds a provider-named secret header with an explicit prefix.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::CredentialInvalid`] when the credential is not
    /// UTF-8, is empty, or contains a line break.
    pub fn prefixed(
        name: impl Into<String>,
        prefix: &str,
        credential: Vec<u8>,
    ) -> Result<Self, TransportError> {
        let credential = Zeroizing::new(credential);
        let text = std::str::from_utf8(&credential)
            .ok()
            .filter(|text| !text.is_empty())
            .filter(|text| !text.bytes().any(|byte| matches!(byte, b'\r' | b'\n')))
            .ok_or(TransportError::CredentialInvalid)?;
        let mut value = String::with_capacity(prefix.len() + text.len());
        value.push_str(prefix);
        value.push_str(text);
        Self::new(name, value)
    }

    pub(crate) const fn name(&self) -> &str {
        self.name.as_str()
    }

    pub(crate) fn value(&self) -> &str {
        self.value.as_str()
    }
}

fn validate_secret(name: &str, value: &str) -> Result<(), TransportError> {
    if name.is_empty()
        || name
            .bytes()
            .any(|byte| !byte.is_ascii_graphic() || byte == b':')
        || value.is_empty()
        || value.bytes().any(|byte| matches!(byte, b'\r' | b'\n'))
    {
        return Err(TransportError::CredentialInvalid);
    }
    Ok(())
}

/// One bounded HTTP request, independent of any provider protocol.
#[derive(Clone)]
pub struct HttpRequest {
    method: Method,
    path: String,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    secret_headers: Vec<SecretHeader>,
    body: Option<Vec<u8>>,
    response_limit: usize,
    connect_timeout: Option<Duration>,
    transfer_timeout: Option<Duration>,
    emit_headers: bool,
    header_filter: Option<fn(&str) -> bool>,
}

impl HttpRequest {
    /// Starts a request for one static route beneath the endpoint base path.
    #[must_use]
    pub fn new(method: Method, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            query: Vec::new(),
            headers: Vec::new(),
            secret_headers: Vec::new(),
            body: None,
            response_limit: DEFAULT_RESPONSE_LIMIT,
            connect_timeout: Some(DEFAULT_CONNECT_TIMEOUT),
            transfer_timeout: None,
            emit_headers: false,
            header_filter: None,
        }
    }

    /// Appends one query parameter.
    #[must_use]
    pub fn with_query(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.push((name.into(), value.into()));
        self
    }

    /// Appends one non-secret header.
    #[must_use]
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    /// Appends one secret header.
    #[must_use]
    pub fn with_secret_header(mut self, header: SecretHeader) -> Self {
        self.secret_headers.push(header);
        self
    }

    /// Attaches the request body bytes.
    #[must_use]
    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Sets the bound on the buffered response or SSE event bytes.
    #[must_use]
    pub const fn with_response_limit(mut self, limit: usize) -> Self {
        self.response_limit = limit;
        self
    }

    /// Overrides the connection-establishment bound.
    #[must_use]
    pub const fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    /// Removes the connection-establishment bound.
    #[must_use]
    pub const fn without_connect_timeout(mut self) -> Self {
        self.connect_timeout = None;
        self
    }

    /// Bounds the whole exchange, including a streaming one.
    #[must_use]
    pub const fn with_transfer_timeout(mut self, timeout: Duration) -> Self {
        self.transfer_timeout = Some(timeout);
        self
    }

    /// Emits the filtered response headers as the first streaming item.
    #[must_use]
    pub const fn stream_response_headers(mut self) -> Self {
        self.emit_headers = true;
        self
    }

    /// Keeps only the response headers the filter accepts.
    ///
    /// The filter receives the lower-cased header name. Without a filter no
    /// provider response header is retained.
    #[must_use]
    pub const fn capture_headers(mut self, filter: fn(&str) -> bool) -> Self {
        self.header_filter = Some(filter);
        self
    }

    /// Resolves the endpoint and route into one absolute request URL.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::EndpointInvalid`] when the endpoint is not an
    /// absolute `http`/`https` URL, carries credentials, a query or a fragment,
    /// or when the route would escape the endpoint base path.
    pub fn resolve(&self, endpoint: &str) -> Result<Url, TransportError> {
        let mut url = Url::parse(endpoint).map_err(|_| TransportError::EndpointInvalid)?;
        if !matches!(url.scheme(), "http" | "https")
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(TransportError::EndpointInvalid);
        }
        let base = url.path().trim_end_matches('/').to_owned();
        let (route, embedded_query) = match self.path.split_once('?') {
            Some((route, query)) => (route, Some(query)),
            None => (self.path.as_str(), None),
        };
        if route.contains('#') {
            return Err(TransportError::EndpointInvalid);
        }
        url.set_path(&format!("{base}/{}", route.trim_start_matches('/')));
        if let Some(query) = embedded_query {
            if query.is_empty() {
                return Err(TransportError::EndpointInvalid);
            }
            url.set_query(Some(query));
        }
        if !self.query.is_empty() {
            let mut pairs = url.query_pairs_mut();
            for (name, value) in &self.query {
                pairs.append_pair(name, value);
            }
            drop(pairs);
        }
        Ok(url)
    }

    pub(crate) const fn method(&self) -> Method {
        self.method
    }

    pub(crate) fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    pub(crate) fn secret_headers(&self) -> &[SecretHeader] {
        &self.secret_headers
    }

    pub(crate) fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }

    pub(crate) const fn response_limit(&self) -> usize {
        self.response_limit
    }

    pub(crate) const fn emits_headers(&self) -> bool {
        self.emit_headers
    }

    pub(crate) const fn header_filter(&self) -> Option<fn(&str) -> bool> {
        self.header_filter
    }

    pub(crate) fn effective_connect_timeout(&self) -> Option<Duration> {
        self.connect_timeout
    }

    /// Finite exchanges are bounded by default; streamed ones never are.
    pub(crate) fn effective_transfer_timeout(&self, streaming: bool) -> Option<Duration> {
        match (streaming, self.transfer_timeout) {
            (_, Some(timeout)) => Some(timeout),
            (true, None) => None,
            (false, None) => Some(DEFAULT_TRANSFER_TIMEOUT),
        }
    }
}
