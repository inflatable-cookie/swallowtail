use serde_json::{Value, json};

const INPUT_CALLBACK: &str =
    include_str!("fixtures/opencode-v1.14.48-v1.18.10/input-callback-corpus.json");
const COMPATIBILITY: &str = include_str!("fixtures/opencode-v1.14.48-v1.18.10/compatibility.json");

fn version_tuple(version: &str) -> (u32, u32, u32) {
    let mut parts = version.split('.').map(|part| {
        part.parse::<u32>()
            .expect("qualified fixture versions are numeric")
    });
    (
        parts.next().expect("major exists"),
        parts.next().expect("minor exists"),
        parts.next().expect("patch exists"),
    )
}

#[test]
fn every_qualified_release_maps_to_one_input_callback_surface() {
    let corpus: Value = serde_json::from_str(INPUT_CALLBACK).expect("fixture parses");
    let compatibility: Value = serde_json::from_str(COMPATIBILITY).expect("fixture parses");
    let segments = corpus["surface_segments"]
        .as_array()
        .expect("segments are an array");
    let releases = compatibility["releases"]
        .as_array()
        .expect("releases are an array");

    assert_eq!(releases.len(), 59);
    assert_eq!(corpus["qualified_release_count"], 59);
    for release in releases {
        let version = version_tuple(
            release["version"]
                .as_str()
                .expect("release version is a string"),
        );
        let matches = segments
            .iter()
            .filter(|segment| {
                version >= version_tuple(segment["minimum"].as_str().expect("minimum is a string"))
                    && version
                        <= version_tuple(segment["maximum"].as_str().expect("maximum is a string"))
            })
            .count();
        assert_eq!(matches, 1, "release must map to one surface");
    }
}

#[test]
fn file_part_is_bounded_inline_input_without_client_path_authority() {
    let corpus: Value = serde_json::from_str(INPUT_CALLBACK).expect("fixture parses");
    let attachment = &corpus["attachment"];

    assert_eq!(attachment["maximum_count"], 1);
    assert_eq!(attachment["accepted_media"], json!(["image/png"]));
    assert_eq!(attachment["part"]["type"], "file");
    assert_eq!(attachment["part"]["mime"], "image/png");
    assert!(
        attachment["part"]["url"]
            .as_str()
            .expect("URL is a string")
            .starts_with("data:image/png;base64,")
    );
    assert!(attachment["part"].get("path").is_none());
}

#[test]
fn permission_and_question_corpora_expose_only_one_shot_responses() {
    let corpus: Value = serde_json::from_str(INPUT_CALLBACK).expect("fixture parses");

    assert_eq!(corpus["permission"]["namespace"], "opencode/permission");
    assert_eq!(
        corpus["permission"]["allowed_replies"],
        json!(["once", "reject"])
    );
    assert_eq!(corpus["permission"]["once_body"]["reply"], "once");
    assert_eq!(corpus["permission"]["persistent_reply"], "unsupported");
    assert_eq!(corpus["question"]["namespace"], "opencode/question");
    assert_eq!(
        corpus["question"]["reply_body"]["answers"],
        json!([["Safe"]])
    );
    assert_eq!(corpus["cancellation"]["pending_responses"], "abandoned");
    assert!(
        corpus["response_failures"]
            .as_array()
            .expect("failures are an array")
            .contains(&json!("duplicate_response"))
    );
}
