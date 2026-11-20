use std::collections::BTreeMap;

/// One completed bounded HTTP response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpResponse {
    /// Final HTTP status code reported by the peer.
    pub status: u32,
    /// Filtered response headers, keyed by lower-cased name.
    pub headers: BTreeMap<String, String>,
    /// Buffered response body bytes.
    pub body: Vec<u8>,
}
