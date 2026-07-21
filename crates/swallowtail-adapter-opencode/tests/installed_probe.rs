use curl::easy::Easy;

#[test]
#[ignore = "requires SWALLOWTAIL_OPENCODE_PROBE_ENDPOINT and an operator-started OpenCode server"]
fn installed_opencode_health_and_schema_match_the_frozen_subset() {
    let endpoint = std::env::var("SWALLOWTAIL_OPENCODE_PROBE_ENDPOINT")
        .expect("set the explicit unauthenticated OpenCode probe endpoint");
    let health: serde_json::Value =
        serde_json::from_slice(&get(&endpoint, "/global/health")).expect("health response is JSON");
    assert_eq!(health["healthy"], true);
    assert_eq!(health["version"], "1.14.48");

    let schema: serde_json::Value =
        serde_json::from_slice(&get(&endpoint, "/doc")).expect("schema response is JSON");
    assert_eq!(schema["openapi"], "3.1.0");
    for path in [
        "/global/health",
        "/provider",
        "/session",
        "/session/{sessionID}/prompt_async",
        "/event",
        "/session/{sessionID}/abort",
    ] {
        assert!(schema["paths"].get(path).is_some(), "missing {path}");
    }
}

fn get(endpoint: &str, path: &str) -> Vec<u8> {
    let mut easy = Easy::new();
    easy.url(&format!("{}{path}", endpoint.trim_end_matches('/')))
        .expect("probe endpoint is valid");
    easy.proxy("").expect("ambient proxy is disabled");
    easy.follow_location(false)
        .expect("redirect following is disabled");
    let mut body = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|chunk| {
                body.extend_from_slice(chunk);
                Ok(chunk.len())
            })
            .expect("response callback installs");
        transfer.perform().expect("probe request succeeds");
    }
    assert_eq!(easy.response_code().expect("status is available"), 200);
    body
}
