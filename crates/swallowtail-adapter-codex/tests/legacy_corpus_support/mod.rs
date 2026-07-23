#![allow(dead_code)] // Each integration-test binary uses a different helper subset.

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const EXEC_RELEASES: &str = include_str!("../fixtures/compatibility/legacy-exec-releases.json");
pub const APP_SERVER_RELEASES: &str =
    include_str!("../fixtures/compatibility/legacy-app-server-releases.json");

pub fn assert_release_evidence(release: &Value) {
    assert_eq!(
        release["tag_commit"]
            .as_str()
            .expect("tag commit is text")
            .len(),
        40
    );
    assert_eq!(release["npm_package"], "@openai/codex");
    let artifact_version = release["npm_artifact_version"]
        .as_str()
        .expect("npm artifact version is text");
    assert!(
        artifact_version.starts_with(
            release["version"]
                .as_str()
                .expect("release version is text")
        ),
        "artifact version must preserve the release version"
    );
    assert!(
        release["npm_integrity"]
            .as_str()
            .expect("npm integrity is text")
            .starts_with("sha512-")
    );
    assert_eq!(
        release["npm_shasum"]
            .as_str()
            .expect("npm shasum is text")
            .len(),
        40
    );
    for key in [
        "cli_source_sha256",
        "events_source_sha256",
        "cli_main_source_sha256",
    ] {
        if let Some(value) = release.get(key) {
            assert_sha256(value);
        }
    }
}

pub fn assert_no_private_fields(value: &Value) {
    const PRIVATE_FIELDS: &[&str] = &[
        "access_token",
        "api_key",
        "authorization",
        "content",
        "credential",
        "cwd",
        "home",
        "prompt",
        "refresh_token",
    ];
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                assert!(
                    !PRIVATE_FIELDS.contains(&key.as_str()),
                    "private field {key} is forbidden"
                );
                assert_no_private_fields(value);
            }
        }
        Value::Array(values) => {
            for value in values {
                assert_no_private_fields(value);
            }
        }
        _ => {}
    }
}

pub fn releases_by_version(corpus: &Value) -> BTreeMap<&str, &Value> {
    corpus["releases"]
        .as_array()
        .expect("releases are an array")
        .iter()
        .map(|release| {
            (
                release["version"].as_str().expect("version is text"),
                release,
            )
        })
        .collect()
}

pub fn release_versions(corpus: &Value) -> Vec<&str> {
    corpus["releases"]
        .as_array()
        .expect("releases are an array")
        .iter()
        .map(|release| release["version"].as_str().expect("version is text"))
        .collect()
}

pub fn segments_by_id(corpus: &Value) -> BTreeMap<&str, &Value> {
    corpus["segments"]
        .as_array()
        .expect("segments are an array")
        .iter()
        .map(|segment| (segment["id"].as_str().expect("segment id is text"), segment))
        .collect()
}

pub fn assert_sha256(value: &Value) {
    let value = value.as_str().expect("digest is text");
    assert_eq!(value.len(), 64);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

pub fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen corpus JSON is valid")
}

pub fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|value| value.as_str().expect("array value is text"))
        .collect()
}

pub fn string_set(value: &Value) -> BTreeSet<&str> {
    strings(value).into_iter().collect()
}
