#[cfg(test)]
mod tests {
    use super::request_url;
    use crate::protocol::Request;

    #[test]
    fn route_is_appended_without_implicit_endpoint_fallback() {
        let request = Request::get("/provider").with_directory("/workspace/fixture");
        let url = request_url("http://127.0.0.1:4096/base", &request).expect("URL is valid");
        assert_eq!(
            url.as_str(),
            "http://127.0.0.1:4096/base/provider?directory=%2Fworkspace%2Ffixture"
        );
        assert!(request_url("http://user:secret@localhost:4096", &request).is_err());
    }
}

