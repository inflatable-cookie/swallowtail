//! Live ACP claim identity versus frozen historical reservations.

use super::support::{FROZEN_0_30_0_0_31_0_RANGE, FROZEN_0_31_1_RELEASE, json};
use swallowtail_adapter_kimi::{kimi_acp_claim, kimi_code_binding};
use swallowtail_core::{
    ExecutionHostId, InstalledExecutableObservation, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId,
};
use swallowtail_runtime::observe_instance_update;

const PRE_A2_LIVE_ACP_CLAIM_ID: &str = "kimi.acp.executable-window-2";
const LIVE_ACP_CLAIM_ID: &str = "kimi.acp.executable-window-5";
const RESERVED_HISTORICAL_ACP_CLAIM_IDS: [&str; 2] = [
    "kimi.acp.executable-window-3",
    "kimi.acp.executable-window-4",
];

#[test]
fn frozen_historical_acp_claim_ids_stay_reserved() {
    let range = json(FROZEN_0_30_0_0_31_0_RANGE);
    let release = json(FROZEN_0_31_1_RELEASE);
    let from_fixtures = [
        range["acp"]["claim"]
            .as_str()
            .expect("0.30.0-0.31.0 ACP claim is text"),
        release["acp"]["claim"]
            .as_str()
            .expect("0.31.1 ACP claim is text"),
    ];
    assert_eq!(
        from_fixtures, RESERVED_HISTORICAL_ACP_CLAIM_IDS,
        "frozen corpora and the reserved set must name the same historical ids"
    );

    let live_claim = kimi_acp_claim();
    let live = live_claim.id().as_str();
    assert_eq!(live, LIVE_ACP_CLAIM_ID);
    assert_ne!(
        live, PRE_A2_LIVE_ACP_CLAIM_ID,
        "live id must not reuse the immediate pre-A2 claim"
    );
    assert!(
        !RESERVED_HISTORICAL_ACP_CLAIM_IDS.contains(&live),
        "live id {live} collides with a frozen historical ACP claim id"
    );
}

#[test]
fn window_2_observation_fails_closed_against_window_5_before_projection() {
    let current = kimi_acp_claim();
    assert_eq!(current.id().as_str(), LIVE_ACP_CLAIM_ID);

    let stale_claim = rebuild_id(&current, PRE_A2_LIVE_ACP_CLAIM_ID);
    let stale = InstalledExecutableObservation::classify(
        ExecutionHostId::new("fixture.host.stale-claim").expect("valid host"),
        kimi_code_binding("0.38.0").expect("qualified binding"),
        &stale_claim,
    )
    .expect("window-2 classifies qualified 0.38.0");
    assert_eq!(stale.claim_id().as_str(), PRE_A2_LIVE_ACP_CLAIM_ID);
    assert!(stale.is_qualified());

    let error = observe_instance_update(&current, Some(stale.clone()))
        .expect_err("window-2 evidence must fail closed against window-5 before projection");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.connection_lifecycle.update_claim_mismatch"
    );

    let reverted = rebuild_id(&current, PRE_A2_LIVE_ACP_CLAIM_ID);
    observe_instance_update(&reverted, Some(stale))
        .expect("mutating the id back to window-2 would accept the stale observation");

    assert_eq!(kimi_acp_claim().id().as_str(), LIVE_ACP_CLAIM_ID);
}

fn rebuild_id(original: &InterfaceCompatibilityClaim, id: &str) -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new(id).expect("fixture claim id is valid"),
        original.axis().clone(),
        original.scheme(),
        original.newer_version_posture(),
        original.milestones().cloned(),
        original.exclusions().cloned(),
    )
    .expect("id-mutated claim stays structurally valid")
}
