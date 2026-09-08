//! Provider-free failed-open receipt proofs for the `claude-agent.sdk`
//! prepared route.
//!
//! Card 144: one failed open is useful evidence without being a support
//! claim. The exact bounded rejection stage, the sidecar subcode, the
//! provider-readiness truth, and the observed cleanup disposition must be
//! readable from typed fields — never by parsing a diagnostic message.
//!
//! Frozen Card 132 tuple (Research 296): SDK `0.3.259`, native `2.1.259`,
//! Node `22.23.2`, sidecar source tag
//! `swallowtail-claude-agent-sdk-sidecar@0.4.4`, carrier
//! `swallowtail-claude-agent-sdk-registered-tool-mcp-v1`,
//! `private-loopback-http` plus `mediated-stdio-proxy`, MCP `2025-11-25`,
//! model `claude-sonnet-5`, default permission, persistence false, strict
//! MCP configuration, empty setting sources, and omitted `allowedTools`.

use crate::host_id;
use crate::sdk_support::{
    CleanupEvent, SdkFixtureHost, SdkScenario, Stall, cleanup_request, prepared_session,
};
use futures_executor::block_on;
use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkOpenCleanupDisposition, ClaudeAgentSdkOpenRejection, ClaudeAgentSdkOpenStage,
    ClaudeAgentSdkOpenSubcode,
};
use swallowtail_runtime::{CleanupOutcome, InteractiveSessionHandle, ProcessTreeCompletion};

/// Opens against the named scenario and returns the structured rejection.
fn rejected_open(scenario: SdkScenario) -> ClaudeAgentSdkOpenRejection {
    let host = host_id("claude-agent-sdk.fixture.open-receipt");
    let fixture = SdkFixtureHost::new(scenario);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let Err(rejection) = block_on(prepared.open_route_session_with_receipt(services)) else {
        panic!("the {scenario:?} scenario must fail the open");
    };
    rejection
}

#[test]
fn a_construction_rejection_reports_its_typed_receipt() {
    let rejection = rejected_open(SdkScenario::OpenRejected);
    assert_eq!(
        rejection.failure().diagnostic().code(),
        "swallowtail.claude-agent.sdk.open_rejected"
    );
    let receipt = rejection.receipt();
    assert_eq!(
        receipt.route_code(),
        "swallowtail.claude-agent.sdk.open_rejected"
    );
    assert_eq!(receipt.stage(), ClaudeAgentSdkOpenStage::SidecarRejected);
    assert_eq!(
        receipt.sidecar_code(),
        Some(ClaudeAgentSdkOpenSubcode::ConstructionFailed)
    );
    assert!(!receipt.provider_readiness_reached());
    let cleanup = receipt.cleanup();
    assert_eq!(
        cleanup.disposition(),
        ClaudeAgentSdkOpenCleanupDisposition::Confirmed
    );
    // The fixture working-resource service releases with no applicable
    // action by design; the credential lease release is Clean.
    assert_eq!(cleanup.resource(), Some(&CleanupOutcome::NotApplicable));
    assert_eq!(cleanup.credential(), Some(&CleanupOutcome::Clean));
    assert_eq!(
        cleanup.survivor_posture(),
        Some(ProcessTreeCompletion::RootOnly)
    );
    assert!(cleanup.registered_lease().is_none());
}

/// The failure the ordinary open surface returns keeps its exact stable code
/// alongside the receipt surface.
#[test]
fn the_receipt_surface_preserves_the_ordinary_failure_code() {
    let host = host_id("claude-agent-sdk.fixture.receipt-parity");
    let fixture = SdkFixtureHost::new(SdkScenario::OpenRejected);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let ordinary = block_on(prepared.open_session(services));
    let host = host_id("claude-agent-sdk.fixture.receipt-parity-2");
    let fixture = SdkFixtureHost::new(SdkScenario::OpenRejected);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let receipted = block_on(prepared.open_route_session_with_receipt(services));
    let ordinary_code = ordinary
        .err()
        .unwrap_or_else(|| panic!("the ordinary open must fail"))
        .diagnostic()
        .code()
        .to_owned();
    let receipted_code = receipted
        .err()
        .unwrap_or_else(|| panic!("the receipted open must fail"))
        .into_failure()
        .diagnostic()
        .code()
        .to_owned();
    assert_eq!(ordinary_code, receipted_code);
}

#[test]
fn an_initialization_rejection_reports_its_typed_receipt() {
    let rejection = rejected_open(SdkScenario::OpenInitializationRejected);
    assert_eq!(
        rejection.failure().diagnostic().code(),
        "swallowtail.claude-agent.sdk.open_rejected"
    );
    let receipt = rejection.receipt();
    assert_eq!(receipt.stage(), ClaudeAgentSdkOpenStage::SidecarRejected);
    assert_eq!(
        receipt.sidecar_code(),
        Some(ClaudeAgentSdkOpenSubcode::InitializationFailed)
    );
    assert!(!receipt.provider_readiness_reached());
    assert_eq!(
        receipt.cleanup().disposition(),
        ClaudeAgentSdkOpenCleanupDisposition::Confirmed
    );
}

#[test]
fn an_account_rejection_reports_its_typed_receipt() {
    let rejection = rejected_open(SdkScenario::OpenAccountRejected);
    let receipt = rejection.receipt();
    assert_eq!(receipt.stage(), ClaudeAgentSdkOpenStage::SidecarRejected);
    assert_eq!(
        receipt.sidecar_code(),
        Some(ClaudeAgentSdkOpenSubcode::AccountUnavailable)
    );
    assert!(!receipt.provider_readiness_reached());
}

#[test]
fn an_mcp_status_rejection_reports_its_typed_receipt() {
    let rejection = rejected_open(SdkScenario::McpRequiredFail);
    let receipt = rejection.receipt();
    assert_eq!(receipt.stage(), ClaudeAgentSdkOpenStage::SidecarRejected);
    assert_eq!(
        receipt.sidecar_code(),
        Some(ClaudeAgentSdkOpenSubcode::McpServerFailed)
    );
    assert!(!receipt.provider_readiness_reached());
    assert_eq!(
        receipt.cleanup().disposition(),
        ClaudeAgentSdkOpenCleanupDisposition::Confirmed
    );
}

#[test]
fn an_account_readiness_validation_failure_names_its_stage() {
    // The sidecar answers open successfully with a non-first-party account;
    // the route rejects the evidence itself. That is readiness validation,
    // not a sidecar rejection, so no sidecar subcode exists.
    let rejection = rejected_open(SdkScenario::AccountNotFirstParty);
    let receipt = rejection.receipt();
    assert_eq!(
        receipt.stage(),
        ClaudeAgentSdkOpenStage::ReadinessValidation
    );
    assert_eq!(receipt.sidecar_code(), None);
    assert!(!receipt.provider_readiness_reached());
    assert_eq!(
        receipt.cleanup().disposition(),
        ClaudeAgentSdkOpenCleanupDisposition::Confirmed
    );
}

#[test]
fn a_deadline_open_reports_the_deadline_stage_and_unconfirmed_cleanup() {
    // The sidecar never answers open; the immediate fixture clock fires the
    // caller's deadline, so the guard cannot finish its ordered cleanup
    // inside the same bound. The receipt says so instead of inventing
    // posture.
    let host = host_id("claude-agent-sdk.fixture.open-receipt-deadline");
    let fixture = SdkFixtureHost::new(SdkScenario::OpenHold).with_immediate_time();
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let Err(rejection) = block_on(prepared.open_route_session_with_receipt(services)) else {
        panic!("an unbounded open must fail on the host deadline");
    };
    assert_eq!(
        rejection.failure().diagnostic().code(),
        "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed"
    );
    let receipt = rejection.receipt();
    assert_eq!(receipt.stage(), ClaudeAgentSdkOpenStage::Deadline);
    assert_eq!(receipt.sidecar_code(), None);
    assert!(!receipt.provider_readiness_reached());
    let cleanup = receipt.cleanup();
    assert_eq!(
        cleanup.disposition(),
        ClaudeAgentSdkOpenCleanupDisposition::Unconfirmed
    );
    assert_eq!(cleanup.survivor_posture(), None);
    assert_eq!(cleanup.resource(), None);
    assert_eq!(cleanup.credential(), None);
    // The guard still owns the termination request even though this future
    // returned.
    fixture.wait_for_cleanup(CleanupEvent::ProcessWait);
    fixture.release_process_hold();
    fixture.wait_for_cleanup(CleanupEvent::CredentialRelease);
    fixture.reaper().shutdown();
}

/// A failure refused before anything was acquired owes no cleanup, and the
/// receipt must not report unconfirmed cleanup for it.
#[test]
fn an_open_refused_after_its_deadline_owes_no_cleanup() {
    let host = host_id("claude-agent-sdk.fixture.open-receipt-elapsed");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    // The clock is already past the open deadline, so the route refuses the
    // open before arming its guard: nothing is acquired, nothing is owed.
    fixture.advance_time();
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let Err(rejection) = block_on(prepared.open_route_session_with_receipt(services)) else {
        panic!("an elapsed open deadline must refuse the open");
    };
    assert_eq!(
        rejection.failure().diagnostic().code(),
        "swallowtail.claude-agent.sdk.open_deadline_elapsed"
    );
    let receipt = rejection.receipt();
    // No replacement happened, so the receipt's route code is the returned
    // failure's own code.
    assert_eq!(
        receipt.route_code(),
        "swallowtail.claude-agent.sdk.open_deadline_elapsed"
    );
    assert_eq!(receipt.stage(), ClaudeAgentSdkOpenStage::Deadline);
    assert_eq!(
        receipt.cleanup().disposition(),
        ClaudeAgentSdkOpenCleanupDisposition::NotAcquired
    );
    assert_eq!(receipt.cleanup().resource(), None);
    assert_eq!(receipt.cleanup().credential(), None);
    assert_eq!(receipt.cleanup().survivor_posture(), None);
}

/// When the deadline ends an open whose sidecar never answered, the receipt
/// names the deadline-and-unconfirmed-cleanup truth itself: no rejection
/// arrived, so none may be claimed.
#[test]
fn a_deadline_ended_open_never_claims_an_unarrived_rejection() {
    let host = host_id("claude-agent-sdk.fixture.receipt-replaced");
    let fixture = SdkFixtureHost::new(SdkScenario::OpenRejected)
        .stalling(Stall::ForceStop)
        .with_immediate_time();
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let Err(rejection) = block_on(prepared.open_route_session_with_receipt(services)) else {
        panic!("the stalled open must fail on the host deadline");
    };
    assert_eq!(
        rejection.failure().diagnostic().code(),
        "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed"
    );
    let receipt = rejection.receipt();
    assert_eq!(
        receipt.route_code(),
        "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed"
    );
    assert_eq!(receipt.stage(), ClaudeAgentSdkOpenStage::Deadline);
    assert_eq!(receipt.sidecar_code(), None);
    assert!(!receipt.provider_readiness_reached());
    let cleanup = receipt.cleanup();
    assert_eq!(
        cleanup.disposition(),
        ClaudeAgentSdkOpenCleanupDisposition::Unconfirmed
    );
    assert_eq!(cleanup.resource(), None);
    assert_eq!(cleanup.credential(), None);
    // The guard still owns the termination request.
    fixture.wait_for_cleanup(CleanupEvent::ProcessForceStop);
}

/// A prepared session that survives to close keeps the ordinary surface
/// unchanged.
#[test]
fn a_healthy_open_still_closes_through_the_ordinary_surface() {
    let host = host_id("claude-agent-sdk.fixture.receipt-healthy");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("session opens");
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}
