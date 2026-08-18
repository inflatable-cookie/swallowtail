#[test]
fn exact_version_codec_separates_qualified_unverified_and_incompatible_points() {
    for version in ["0.14.0", "0.18.0", "0.30.0", "0.32.1"] {
        let name = format!("version-{version}.json");
        let body = std::fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/ollama-native-v0.14.0-v0.32.1")
                .join(name),
        )
        .expect("fixture reads");
        let binding = parse_version(&response(200, &body)).expect("version is qualified");
        assert_eq!(binding.version().as_str(), version);
    }
    let newer = parse_version(&response(200, fixture_bytes!("version-newer.json")))
        .expect("stable newer version is preserved for an unverified attempt");
    assert_eq!(newer.version().as_str(), "0.32.15");
    assert!(matches!(
        crate::selection::ollama_runtime_claim().assess(newer.version()),
        swallowtail_core::InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    for body in [
        fixture_bytes!("version-below.json").as_slice(),
        fixture_bytes!("version-above.json").as_slice(),
        fixture_bytes!("version-0.32.10.json").as_slice(),
        fixture_bytes!("version-prerelease.json").as_slice(),
        fixture_bytes!("version-malformed.json").as_slice(),
    ] {
        let error = parse_version(&response(200, body)).expect_err("version fails closed");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.ollama.version_unsupported"
        );
    }
}

#[test]
fn selected_fixture_versions_are_semantic_values() {
    for version in ["0.14.0", "0.18.0", "0.30.0", "0.32.1"] {
        assert!(!InterfaceVersion::new(version).unwrap().as_str().is_empty());
    }
}

#[test]
fn blank_runtime_version_fails_closed_instead_of_panicking() {
    for version in ["", "   ", " \t ", "\n"] {
        let body = format!(r#"{{"version":{version:?}}}"#);
        let error = parse_version(&response(200, body.as_bytes()))
            .expect_err("blank runtime version must fail closed");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.ollama.version_parse_failed"
        );
        assert_eq!(crate::selection::ollama_runtime_binding(version), None);
    }
}
