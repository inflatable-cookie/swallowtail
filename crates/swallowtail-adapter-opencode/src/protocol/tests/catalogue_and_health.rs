    #[test]
    fn catalogue_keeps_provider_and_model_identity_separate() {
        let fixture: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(format!("{ROOT}/http-success.json")).expect("fixture reads"),
        )
        .expect("fixture parses");
        let responses = fixture.as_array().expect("fixture is an array");
        let health = serde_json::to_vec(&responses[0]["response"]["body"]).expect("serializes");
        observe_health(&Response {
            status: 200,
            body: health,
            next_cursor: None,
        })
        .expect("health parses");
        let body = serde_json::to_vec(&responses[1]["response"]["body"]).expect("serializes");
        let models = parse_catalog(&Response {
            status: 200,
            body,
            next_cursor: None,
        })
        .expect("catalogue parses");
        assert_eq!(
            models[0].provider_id().expect("provider").as_str(),
            "anthropic"
        );
        assert_eq!(models[0].id().as_str(), "claude-sonnet");
        assert!(models[0].metadata().is_default());
        assert_eq!(
            models[0]
                .metadata()
                .token_limits()
                .expect("limits")
                .maximum_input_tokens(),
            Some(190_000)
        );
        assert!(
            models[0]
                .metadata()
                .reasoning()
                .expect("reasoning evidence exists")
                .supports(&ReasoningMode::new("high").expect("mode is valid"))
        );
        assert_eq!(
            models[0]
                .metadata()
                .catalog_observations()
                .expect("catalogue observations exist")
                .tool_calling_supported(),
            Some(true)
        );
    }

    #[test]
    fn health_observation_is_exact_classified_and_safe() {
        let response = fixture_response("health-supported.json");
        let observation = observe_health(&response).expect("candidate health is supported");
        assert_eq!(observation.binding().axis().as_str(), "opencode.server");
        assert_eq!(observation.binding().version().as_str(), "1.18.10");
        assert_eq!(
            observation
                .assessment()
                .behavior_revision()
                .expect("qualified observation has behavior")
                .as_str(),
            "opencode.http-sse.surface-18"
        );
        assert!(matches!(
            observation.assessment(),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
        let formatted = format!("{observation:?}");
        assert!(!formatted.contains("endpoint"));
        assert!(!formatted.contains("credential"));
    }

    #[test]
    fn newer_stable_health_is_permitted_but_unverified() {
        let observation =
            observe_health(&fixture_response("health-above.json")).expect("newer version permits");
        let InterfaceCompatibilityAssessment::UnverifiedNewer(unverified) =
            observation.assessment()
        else {
            panic!("newer stable version must remain unverified");
        };
        assert_eq!(unverified.version().as_str(), "1.18.21");
        assert_eq!(unverified.latest_qualified().as_str(), "1.18.20");
        assert_eq!(
            unverified.behavior_revision().as_str(),
            "opencode.http-sse.surface-19"
        );
    }

    #[test]
    fn incompatible_and_invalid_health_envelopes_fail_closed() {
        for fixture in ["health-below.json", "health-prerelease.json"] {
            let error = observe_health(&fixture_response(fixture)).expect_err("version rejects");
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.opencode.version_unsupported"
            );
        }
        let malformed =
            observe_health(&fixture_response("health-malformed.json")).expect_err("malformed");
        assert_eq!(
            malformed.diagnostic().code(),
            "swallowtail.opencode.version_invalid"
        );
        let missing =
            observe_health(&fixture_response("health-missing.json")).expect_err("missing");
        assert_eq!(
            missing.diagnostic().code(),
            "swallowtail.opencode.protocol_invalid"
        );
        let unhealthy =
            observe_health(&fixture_response("health-unhealthy.json")).expect_err("unhealthy");
        assert_eq!(
            unhealthy.diagnostic().code(),
            "swallowtail.opencode.unhealthy"
        );
        let unpublished = observe_health(&Response {
            status: 200,
            body: br#"{"healthy":true,"version":"1.15.8"}"#.to_vec(),
            next_cursor: None,
        })
        .expect_err("unpublished semantic gap rejects");
        assert_eq!(
            unpublished.diagnostic().code(),
            "swallowtail.opencode.version_unsupported"
        );
    }

    #[test]
    fn session_version_must_match_the_exact_health_binding() {
        let expected = opencode_server_binding("1.18.10").expect("expected version is safe");
        let error = parse_session_for_version(
            &fixture_response("session-version-mismatch.json"),
            &expected,
        )
        .expect_err("session drift rejects");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.session_invalid"
        );
    }

