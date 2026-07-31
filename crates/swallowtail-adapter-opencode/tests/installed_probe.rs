use curl::easy::Easy;
use serde_json::Value;
use swallowtail_adapter_opencode::{opencode_http_claim, opencode_server_binding};
use swallowtail_core::InterfaceCompatibilityAssessment;

#[test]
#[ignore = "requires SWALLOWTAIL_OPENCODE_PROBE_ENDPOINT and an operator-started OpenCode server"]
fn installed_opencode_health_and_schema_match_the_frozen_subset() {
    let endpoint = std::env::var("SWALLOWTAIL_OPENCODE_PROBE_ENDPOINT")
        .expect("set the explicit unauthenticated OpenCode probe endpoint");
    let health: serde_json::Value =
        serde_json::from_slice(&get(&endpoint, "/global/health")).expect("health response is JSON");
    classify_health(&health).unwrap_or_else(|message| panic!("{message}"));

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

fn classify_health(health: &Value) -> Result<InterfaceCompatibilityAssessment, &'static str> {
    if health.get("healthy") != Some(&Value::Bool(true)) {
        return Err("OpenCode server did not report healthy status");
    }
    let version = health
        .get("version")
        .and_then(Value::as_str)
        .ok_or("OpenCode server did not report a string version")?;
    let binding =
        opencode_server_binding(version).ok_or("OpenCode server reported a malformed version")?;
    let assessment = opencode_http_claim().assess(binding.version());
    if assessment.is_permitted() {
        Ok(assessment)
    } else {
        Err("OpenCode server version is incompatible with this driver")
    }
}

#[test]
fn current_qualified_server_health_is_accepted() {
    let assessment = classify_health(&serde_json::json!({
        "healthy": true,
        "version": "1.18.10"
    }))
    .expect("current qualified health is accepted");
    assert!(matches!(
        assessment,
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
}

#[test]
fn later_stable_server_health_is_visibly_unverified() {
    let assessment = classify_health(&serde_json::json!({
        "healthy": true,
        "version": "1.18.11"
    }))
    .expect("later stable health is permitted");
    let InterfaceCompatibilityAssessment::UnverifiedNewer(unverified) = assessment else {
        panic!("later stable server must remain visibly unverified");
    };
    assert_eq!(unverified.version().as_str(), "1.18.11");
    assert_eq!(unverified.latest_qualified().as_str(), "1.18.10");
}

#[test]
fn incompatible_and_malformed_server_health_is_rejected() {
    for version in ["1.14.47", "1.15.8", "1.18.11-rc.1"] {
        assert!(
            classify_health(&serde_json::json!({
                "healthy": true,
                "version": version
            }))
            .is_err(),
            "{version} passed"
        );
    }
    for health in [
        serde_json::json!({"healthy": true, "version": "current"}),
        serde_json::json!({"healthy": true, "version": "1.18.10\n"}),
        serde_json::json!({"healthy": true, "version": 1_018_010}),
        serde_json::json!({"healthy": true}),
    ] {
        assert!(classify_health(&health).is_err());
    }
}

#[test]
fn unhealthy_server_is_rejected_before_compatibility_acceptance() {
    assert!(
        classify_health(&serde_json::json!({
            "healthy": false,
            "version": "1.18.10"
        }))
        .is_err()
    );
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
