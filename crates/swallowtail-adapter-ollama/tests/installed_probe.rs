#[test]
#[ignore = "requires SWALLOWTAIL_OLLAMA_PROBE_ENDPOINT and an operator-started Ollama runtime"]
fn installed_runtime_exposes_qualified_version_and_local_catalogue() {
    let endpoint = std::env::var("SWALLOWTAIL_OLLAMA_PROBE_ENDPOINT")
        .expect("set an explicit operator-approved Ollama endpoint");
    assert!(
        endpoint.starts_with("http://127.0.0.1:") || endpoint.starts_with("http://[::1]:"),
        "the first probe accepts only an explicit loopback endpoint"
    );

    let version = curl(&endpoint, "/api/version");
    let value: serde_json::Value = serde_json::from_slice(&version).expect("version JSON parses");
    let version = value["version"].as_str().expect("version is present");
    assert!(
        swallowtail_adapter_ollama::ollama_runtime_claim()
            .supports(swallowtail_adapter_ollama::ollama_runtime_binding(version).version()),
        "installed runtime version is qualified"
    );

    let tags = curl(&endpoint, "/api/tags");
    let value: serde_json::Value = serde_json::from_slice(&tags).expect("catalogue JSON parses");
    assert!(
        value["models"].is_array(),
        "local catalogue has a models array"
    );
}

fn curl(endpoint: &str, path: &str) -> Vec<u8> {
    let output = std::process::Command::new("curl")
        .args([
            "--fail",
            "--silent",
            "--show-error",
            &format!("{endpoint}{path}"),
        ])
        .output()
        .expect("curl starts");
    assert!(output.status.success(), "Ollama probe request succeeds");
    output.stdout
}
