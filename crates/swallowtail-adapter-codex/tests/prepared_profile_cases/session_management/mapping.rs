use super::support::lifecycle_binding;
use super::*;

#[test]
fn prepared_actions_share_exact_codex_lifecycle_mapping() {
    for (version, action) in [
        ("0.80.0", "archive"),
        ("0.92.0", "restore"),
        ("0.140.0", "delete"),
        ("0.145.0", "delete"),
    ] {
        let prepared_app = prepared(
            CodexPreparedDriver::AppServer,
            version,
            &RecordingHostServices::default(),
            false,
        );
        let input = CodexSessionManagementInput::new(
            RequestId::new(format!("{action}-{version}")).unwrap(),
            lifecycle_binding(&prepared_app, version),
        );
        let (process, state) = ScriptedAppServer::new(AppServerMode::LifecycleSuccess);
        let outcome = match action {
            "archive" => block_on(
                prepared_app
                    .prepare_archive_session(input)
                    .expect("archive prepares")
                    .execute(crate::support::host_services(process)),
            ),
            "restore" => block_on(
                prepared_app
                    .prepare_restore_session(input)
                    .expect("restore prepares")
                    .execute(crate::support::host_services(process)),
            ),
            "delete" => block_on(
                prepared_app
                    .prepare_delete_session(input)
                    .expect("delete prepares")
                    .execute(crate::support::host_services(process)),
            ),
            _ => unreachable!(),
        }
        .expect("lifecycle action executes");
        assert_eq!(
            outcome.effect().truth(),
            ProviderSessionEffectTruth::Applied
        );
        if action == "delete" {
            assert_eq!(
                outcome.effect().confirmed_deletion_strength(),
                Some(ProviderSessionDeletionStrength::ProviderHardDeleted)
            );
            assert_eq!(
                outcome.effect().affected_scope(),
                Some(ProviderSessionAffectedScope::ProviderDefinedDescendants)
            );
        } else {
            assert_eq!(
                outcome.effect().affected_scope(),
                Some(ProviderSessionAffectedScope::TargetOnly)
            );
        }
        let method = match action {
            "archive" => "thread/archive",
            "restore" => "thread/unarchive",
            "delete" => "thread/delete",
            _ => unreachable!(),
        };
        assert!(state.methods().contains(&method.to_owned()));
        assert!(state.waited());
    }
}

#[test]
fn lifecycle_preparation_stops_unsupported_and_unverified_routes() {
    let legacy = prepared(
        CodexPreparedDriver::AppServer,
        "0.80.0",
        &RecordingHostServices::default(),
        false,
    );
    assert!(
        legacy
            .prepare_restore_session(CodexSessionManagementInput::new(
                RequestId::new("legacy-restore").unwrap(),
                lifecycle_binding(&legacy, "0.80.0"),
            ))
            .is_err()
    );

    let pre_delete = prepared(
        CodexPreparedDriver::AppServer,
        "0.139.0",
        &RecordingHostServices::default(),
        false,
    );
    assert!(
        pre_delete
            .prepare_delete_session(CodexSessionManagementInput::new(
                RequestId::new("pre-delete").unwrap(),
                lifecycle_binding(&pre_delete, "0.139.0"),
            ))
            .is_err()
    );

    let newer = prepared(
        CodexPreparedDriver::AppServer,
        "0.146.0",
        &RecordingHostServices::default(),
        false,
    );
    let input = CodexSessionManagementInput::new(
        RequestId::new("newer-archive").unwrap(),
        lifecycle_binding(&newer, "0.146.0"),
    );
    assert!(newer.prepare_archive_session(input.clone()).is_err());
    assert!(
        newer
            .prepare_archive_session(input.allow_unverified_newer())
            .is_ok()
    );
}
