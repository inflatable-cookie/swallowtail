use super::fixture::PreparedFixture;
use crate::http_support::StreamFixture;
use futures_executor::block_on;
use std::sync::atomic::Ordering;
use std::time::Duration;
use swallowtail_adapter_opencode::{
    OpenCodePreparedDelete, OpenCodeSessionManagementInput, OpenCodeSessionProfileInput,
};
use swallowtail_core::{
    ProviderSessionAffectedScope, ProviderSessionDeletionStrength, ProviderSessionEffectTruth,
};
use swallowtail_runtime::{CleanupOutcome, RequestId};

const DELETION_CORPUS: &str = include_str!("../fixtures/opencode-v1.14.48-v1.18.10/deletion.json");

#[test]
fn every_qualified_delete_segment_and_latest_point_execute_exactly_once() {
    let corpus: serde_json::Value =
        serde_json::from_str(DELETION_CORPUS).expect("deletion corpus parses");
    let mut points: Vec<&str> = corpus["segments"]
        .as_array()
        .expect("segments")
        .iter()
        .map(|segment| segment["minimum"].as_str().expect("minimum"))
        .collect();
    points.push(
        corpus["latest_qualified"]
            .as_str()
            .expect("latest qualified"),
    );

    let fixtures: Vec<_> = points
        .iter()
        .map(|version| {
            PreparedFixture::new(
                &format!("opencode.delete.range.{}", version.replace('.', "-")),
                version,
            )
        })
        .collect();
    for (version, fixture) in points.into_iter().zip(&fixtures) {
        let delete = prepared_delete(fixture, &format!("range-{version}"), false);
        let outcome = block_on(delete.execute(fixture.services())).expect("delete executes");
        assert_eq!(
            outcome.effect().truth(),
            ProviderSessionEffectTruth::Applied,
            "{version}: {:?}",
            fixture.server.requests()
        );
        assert_eq!(
            outcome.effect().confirmed_deletion_strength(),
            Some(ProviderSessionDeletionStrength::ProviderDataDeleted)
        );
        assert_eq!(
            outcome.effect().affected_scope(),
            Some(ProviderSessionAffectedScope::ProviderDefinedDescendants)
        );
        assert_eq!(
            fixture
                .server
                .requests()
                .iter()
                .filter(|request| request.starts_with("DELETE "))
                .count(),
            1
        );
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 3);
    }
}

#[test]
fn local_remote_and_explicit_unverified_newer_execution_preserve_truth() {
    let cases = [
        ("opencode.delete.host.local", "1.18.10", false),
        (
            "opencode.delete.host.remote-authoritative",
            "1.18.10",
            false,
        ),
        ("opencode.delete.host.newer", "1.18.29", true),
    ];
    let fixtures: Vec<_> = cases
        .iter()
        .map(|(host_id, version, _)| PreparedFixture::new(host_id, version))
        .collect();
    for ((host_id, _, allow_unverified_newer), fixture) in cases.into_iter().zip(&fixtures) {
        let delete = prepared_delete(fixture, host_id, allow_unverified_newer);
        assert_eq!(
            delete.plan().preflight().execution_host_id(),
            &fixture.host_id
        );
        let outcome = block_on(delete.execute(fixture.services())).expect("delete executes");
        assert_eq!(
            outcome.effect().truth(),
            ProviderSessionEffectTruth::Applied
        );
        assert_eq!(
            outcome.effect().confirmed_deletion_strength(),
            Some(ProviderSessionDeletionStrength::ProviderDataDeleted)
        );
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 3);
    }
}

#[test]
fn rejection_malformed_server_and_disconnect_cases_keep_exact_effect_truth() {
    let cases = [
        (
            "missing",
            StreamFixture::DeleteMissing,
            ProviderSessionEffectTruth::FailedBeforeEffect,
            "swallowtail.opencode.lifecycle.delete_rejected",
        ),
        (
            "unauthorized",
            StreamFixture::DeleteUnauthorized,
            ProviderSessionEffectTruth::FailedBeforeEffect,
            "swallowtail.opencode.lifecycle.delete_rejected",
        ),
        (
            "malformed",
            StreamFixture::DeleteMalformedSuccess,
            ProviderSessionEffectTruth::UnconfirmedAfterEffect,
            "swallowtail.opencode.lifecycle.delete_unconfirmed",
        ),
        (
            "server",
            StreamFixture::DeleteServerError,
            ProviderSessionEffectTruth::UnconfirmedAfterEffect,
            "swallowtail.opencode.lifecycle.delete_unconfirmed",
        ),
        (
            "disconnect",
            StreamFixture::DeleteDisconnect,
            ProviderSessionEffectTruth::UnconfirmedAfterEffect,
            "swallowtail.opencode.transport_failed",
        ),
    ];
    let fixtures: Vec<_> = cases
        .iter()
        .map(|(suffix, scenario, _, _)| {
            PreparedFixture::new_with_fixture(
                &format!("opencode.delete.failure.{suffix}"),
                "1.18.4",
                *scenario,
            )
        })
        .collect();
    for ((suffix, _, expected, diagnostic), fixture) in cases.into_iter().zip(&fixtures) {
        let delete = prepared_delete(fixture, suffix, false);
        let outcome = block_on(delete.execute(fixture.services())).expect("attempt resolves");
        assert_eq!(outcome.effect().truth(), expected);
        assert_eq!(outcome.effect().confirmed_deletion_strength(), None);
        assert_eq!(outcome.diagnostic().expect("diagnostic").code(), diagnostic);
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 3);
        let debug = format!("{outcome:?}");
        for private in [
            "private missing-target detail",
            "private authorization detail",
            "private server detail",
        ] {
            assert!(!debug.contains(private));
        }
    }
}

#[test]
fn denied_endpoint_and_expired_request_stop_before_delete() {
    let denied = PreparedFixture::new("opencode.delete.denied", "1.18.4");
    let delete = prepared_delete(&denied, "denied", false);
    let failure = block_on(delete.execute(denied.services_with_denied_network()))
        .expect_err("unapproved endpoint rejects");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_network.endpoint_not_approved"
    );
    assert!(
        !denied
            .server
            .requests()
            .iter()
            .any(|request| request.starts_with("DELETE "))
    );

    let expired = PreparedFixture::new("opencode.delete.expired", "1.18.4");
    let prepared = expired.prepared();
    let binding = opened_binding(&expired, &prepared, "expired");
    let delete = prepared
        .prepare_delete_session(
            OpenCodeSessionManagementInput::new(
                RequestId::new("delete-expired").expect("request id"),
                binding,
            )
            .with_deadline(expired.deadline_after(Duration::ZERO)),
        )
        .expect("delete prepares");
    let outcome = block_on(delete.execute(expired.services())).expect("expired attempt resolves");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::FailedBeforeEffect
    );
    assert!(
        !expired
            .server
            .requests()
            .iter()
            .any(|request| request.starts_with("DELETE "))
    );
    assert_eq!(expired.releases.load(Ordering::SeqCst), 2);
}

#[test]
fn deadline_after_dispatch_is_joined_unconfirmed_and_releases_access() {
    let fixture = PreparedFixture::new_with_fixture(
        "opencode.delete.deadline",
        "1.18.4",
        StreamFixture::DeleteGated,
    );
    let prepared = fixture.prepared();
    let binding = opened_binding(&fixture, &prepared, "deadline");
    let delete = prepared
        .prepare_delete_session(
            OpenCodeSessionManagementInput::new(
                RequestId::new("delete-deadline").expect("request id"),
                binding,
            )
            .with_deadline(fixture.deadline_after(Duration::from_secs(1))),
        )
        .expect("delete prepares");
    let deadline = fixture.arm_manual_deadline();
    let response_gate = fixture.server.delete_response_gate();
    let controller = std::thread::spawn(move || {
        response_gate.wait_for_dispatch();
        deadline.fire_and_wait_for_observation();
        response_gate.release();
    });
    let outcome = block_on(delete.execute(fixture.services())).expect("deadline attempt resolves");
    controller.join().expect("deadline controller joins");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::UnconfirmedAfterEffect
    );
    assert_eq!(outcome.effect().confirmed_deletion_strength(), None);
    assert!(
        fixture
            .server
            .requests()
            .iter()
            .any(|request| request.starts_with("DELETE "))
    );
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 3);
}

fn prepared_delete(
    fixture: &PreparedFixture,
    suffix: &str,
    allow_unverified_newer: bool,
) -> OpenCodePreparedDelete {
    let prepared = fixture.prepared();
    let binding = opened_binding(fixture, &prepared, suffix);
    let mut input = OpenCodeSessionManagementInput::new(
        RequestId::new(format!("delete-{suffix}")).expect("request id"),
        binding,
    );
    if allow_unverified_newer {
        input = input.allow_unverified_newer();
    }
    prepared
        .prepare_delete_session(input)
        .expect("delete prepares")
}

fn opened_binding(
    fixture: &PreparedFixture,
    prepared: &swallowtail_adapter_opencode::OpenCodePreparedIntegration,
    suffix: &str,
) -> swallowtail_runtime::ProviderSessionManagementBinding {
    let session = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new(format!("open-{suffix}")).expect("request id"),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .expect("session prepares");
    let handle = block_on(session.open_session(fixture.services()))
        .unwrap_or_else(|error| panic!("session {suffix} opens: {error:?}"));
    let binding = handle
        .management_binding()
        .expect("management binding")
        .clone();
    assert_eq!(
        block_on(fixture.close_session(handle)),
        CleanupOutcome::Clean
    );
    binding
}
