#[cfg(test)]
mod tests {
    use super::{RESPONSE_LIMIT, shared_request, stream_failure, transport_failure};
    use crate::protocol::Request;
    use std::collections::BTreeMap;
    use swallowtail_transport_http::{StreamError, TransportError};

    #[test]
    fn route_is_appended_without_implicit_endpoint_fallback() {
        let request = Request::get("/provider").with_directory("/workspace/fixture");
        let shared = shared_request(&request).expect("request is shared");
        let url = shared
            .resolve("http://127.0.0.1:4096/base")
            .expect("URL is valid");
        assert_eq!(
            url.as_str(),
            "http://127.0.0.1:4096/base/provider?directory=%2Fworkspace%2Ffixture"
        );
        assert!(
            shared
                .resolve("http://user:secret@localhost:4096")
                .is_err()
        );
    }

    #[test]
    fn response_limit_maps_to_the_bounded_input_failure() {
        let error = transport_failure(TransportError::ResponseLimit);
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.response_limit"
        );
        assert_eq!(RESPONSE_LIMIT, 4 * 1024 * 1024);
    }

    #[test]
    fn rejected_streams_map_to_the_sse_failure() {
        let error = stream_failure(StreamError::Status {
            status: 500,
            headers: BTreeMap::new(),
            body: Vec::new(),
        });
        assert_eq!(error.diagnostic().code(), "swallowtail.opencode.sse_http_failed");
    }

    #[test]
    fn cancelled_exchanges_map_to_the_local_cancellation_failure() {
        let error = transport_failure(TransportError::Cancelled);
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.request_cancelled"
        );
    }
}
