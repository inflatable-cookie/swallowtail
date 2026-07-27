use super::support::lifecycle_binding;
use super::*;

#[test]
fn rejected_and_lost_responses_preserve_effect_truth() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    for (mode, truth) in [
        (
            AppServerMode::LifecycleReject,
            ProviderSessionEffectTruth::FailedBeforeEffect,
        ),
        (
            AppServerMode::LifecycleDisconnect,
            ProviderSessionEffectTruth::UnconfirmedAfterEffect,
        ),
    ] {
        let operation = prepared_app
            .prepare_archive_session(CodexSessionManagementInput::new(
                RequestId::new(format!("failure-{truth:?}")).unwrap(),
                lifecycle_binding(&prepared_app, "0.145.0"),
            ))
            .expect("archive prepares");
        let (process, state) = ScriptedAppServer::new(mode);
        let outcome = block_on(operation.execute(crate::support::host_services(process)))
            .expect("attempt returns typed effect truth");
        assert_eq!(outcome.effect().truth(), truth);
        assert!(state.waited());
    }
}

#[test]
fn response_notification_and_cleanup_disagreement_stay_visible() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    for (mode, truth, code) in [
        (
            AppServerMode::LifecycleMalformed,
            ProviderSessionEffectTruth::UnconfirmedAfterEffect,
            "swallowtail.codex.lifecycle.malformed_response",
        ),
        (
            AppServerMode::LifecycleWrongNotification,
            ProviderSessionEffectTruth::Applied,
            "swallowtail.codex.lifecycle.notification_disagreement",
        ),
        (
            AppServerMode::LifecycleCleanupFailure,
            ProviderSessionEffectTruth::Applied,
            "swallowtail.codex.app_server.close_failed",
        ),
    ] {
        let operation = prepared_app
            .prepare_archive_session(CodexSessionManagementInput::new(
                RequestId::new(format!("disagreement-{code}")).unwrap(),
                lifecycle_binding(&prepared_app, "0.145.0"),
            ))
            .expect("archive prepares");
        let (process, state) = ScriptedAppServer::new(mode);
        let outcome = block_on(operation.execute(crate::support::host_services(process)))
            .expect("attempt resolves with typed evidence");
        assert_eq!(outcome.effect().truth(), truth);
        assert_eq!(
            outcome
                .diagnostic()
                .map(swallowtail_core::SafeDiagnostic::code),
            Some(code)
        );
        assert!(state.waited());
    }
}
