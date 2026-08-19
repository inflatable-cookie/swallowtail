fn requires_experimental_api(message: &Value) -> bool {
    const EXPERIMENTAL_FIELDS: &[&str] = &[
        "allowProviderModelFallback",
        "collaborationMode",
        "dynamicTools",
        "runtimeWorkspaceRoots",
    ];
    message["params"].as_object().is_some_and(|params| {
        EXPERIMENTAL_FIELDS
            .iter()
            .any(|field| params.contains_key(*field))
    })
}

fn notification_is_structurally_valid(message: &Value) -> bool {
    match message["method"].as_str() {
        Some("item/agentMessage/delta") => message["params"]["delta"].is_string(),
        Some(_) => true,
        None => false,
    }
}

fn assert_exact_evidence(release: &Value) {
    assert_eq!(
        release["tag_commit"]
            .as_str()
            .expect("tag commit is text")
            .len(),
        40
    );
    assert!(
        release["npm_integrity"]
            .as_str()
            .expect("integrity is text")
            .starts_with("sha512-")
    );
    assert_eq!(
        release["npm_shasum"]
            .as_str()
            .expect("npm shasum is text")
            .len(),
        40
    );
    for key in ["help_sha256", "cli_source_sha256", "events_source_sha256"] {
        if !release[key].is_null() {
            assert_sha256(&release[key]);
        }
    }
}

fn assert_unverified_newer(corpus: &Value) {
    let release = &corpus["unverified_newer"];
    assert_eq!(release["version"], "0.148.1");
    assert_eq!(
        release["execution"],
        "permitted-with-explicit-unverified-status"
    );
    assert_eq!(release["guaranteed"], false);
    assert_eq!(release["evidence"], "synthetic-later-stable-classification");
}

fn assert_sha256(value: &Value) {
    let value = value.as_str().expect("digest is text");
    assert_eq!(value.len(), 64);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen corpus JSON is valid")
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|value| value.as_str().expect("array value is text"))
        .collect()
}

fn string_set(value: &Value) -> BTreeSet<&str> {
    strings(value).into_iter().collect()
}
