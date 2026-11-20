use super::{AttachedModelTag, ModelManifestDigest};

#[test]
fn model_tags_are_bounded_safe_values() {
    assert_eq!(
        AttachedModelTag::new("qwen3:8b")
            .expect("tag is valid")
            .as_str(),
        "qwen3:8b"
    );
    assert!(AttachedModelTag::new(" ").is_err());
    assert!(AttachedModelTag::new("model\nname").is_err());
    assert!(AttachedModelTag::new("x".repeat(257)).is_err());
}

#[test]
fn manifest_digest_accepts_only_sha256_evidence() {
    let digest = format!("sha256:{}", "a".repeat(64));
    assert_eq!(
        ModelManifestDigest::new(&digest)
            .expect("digest is valid")
            .as_str(),
        digest
    );
    assert!(ModelManifestDigest::new("/private/model/manifest").is_err());
    assert!(ModelManifestDigest::new("sha256:short").is_err());
}

#[test]
fn manifest_digest_equality_is_case_consistent() {
    let lowercase = ModelManifestDigest::new(format!("sha256:{}", "ab".repeat(32)))
        .expect("lowercase digest is valid");
    let uppercase = ModelManifestDigest::new(format!("sha256:{}", "AB".repeat(32)))
        .expect("uppercase digest is valid");
    assert_eq!(lowercase, uppercase);
    assert_eq!(uppercase.as_str(), lowercase.as_str());
    assert!(
        uppercase
            .as_str()
            .bytes()
            .all(|byte| !byte.is_ascii_uppercase())
    );
}
