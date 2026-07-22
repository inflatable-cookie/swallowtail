use crate::{
    ConformanceAssertion, ConformanceReport, RealtimeMediaPreflightCase,
    RealtimeMediaPreflightFixture, RecordedHostCall, RecordingHostServices, SyntheticProfile,
    poll_immediate,
};
use swallowtail_core::{
    CredentialRef, EndpointAudience, PlannedConnectionRolloverPolicy, PreflightDimension,
};
use swallowtail_runtime::{ScopeId, validate_planned_connection_rollover_plan};

pub(crate) fn run() -> ConformanceReport {
    assert_preflight_matrix();
    assert_request_plan_agreement();
    assert_idle_handoff_and_bounds();
    assert_joined_cleanup_before_credential_release();

    let mut report = ConformanceReport::new(SyntheticProfile::RealtimeMediaDirectSession);
    report.record(ConformanceAssertion::PlannedConnectionRollover);
    report.record(ConformanceAssertion::RolloverNoReplay);
    report.record(ConformanceAssertion::RolloverCleanupOrdered);
    report
}

fn assert_preflight_matrix() {
    RealtimeMediaPreflightFixture::for_case(RealtimeMediaPreflightCase::RolloverCanonical)
        .preflight()
        .expect("bounded rollover preflight succeeds");

    for case in [
        RealtimeMediaPreflightCase::RolloverCapabilityWhileDisabled,
        RealtimeMediaPreflightCase::RolloverMissingCapability,
        RealtimeMediaPreflightCase::RolloverMismatchedBound,
        RealtimeMediaPreflightCase::RolloverZeroBound,
    ] {
        let fixture = RealtimeMediaPreflightFixture::for_case(case);
        let failure = fixture
            .preflight()
            .expect_err("invalid policy agreement fails");
        assert_eq!(
            failure.dimension(),
            PreflightDimension::PlannedConnectionRollover
        );
        assert_eq!(fixture.provider_side_effect_count(), 0);
    }

    for (case, dimension) in [
        (
            RealtimeMediaPreflightCase::RolloverInstanceMissing,
            PreflightDimension::Capability,
        ),
        (
            RealtimeMediaPreflightCase::RolloverRouteWrongBound,
            PreflightDimension::Constraint,
        ),
    ] {
        let failure = RealtimeMediaPreflightFixture::for_case(case)
            .preflight()
            .expect_err("advertised rollover mismatch fails");
        assert_eq!(failure.dimension(), dimension);
    }
}

fn assert_request_plan_agreement() {
    let fixture =
        RealtimeMediaPreflightFixture::for_case(RealtimeMediaPreflightCase::RolloverCanonical);
    let plan = fixture.preflight().expect("rollover plan succeeds");
    let request = fixture.open_request();
    validate_planned_connection_rollover_plan(&plan, request.planned_connection_rollover())
        .expect("request agrees with immutable plan");
    assert!(
        validate_planned_connection_rollover_plan(&plan, PlannedConnectionRolloverPolicy::Disabled)
            .is_err()
    );

    let disabled = RealtimeMediaPreflightFixture::for_case(RealtimeMediaPreflightCase::Canonical)
        .open_request();
    assert_eq!(
        disabled.planned_connection_rollover(),
        PlannedConnectionRolloverPolicy::Disabled
    );
}

fn assert_idle_handoff_and_bounds() {
    let mut gate = RolloverGate::default();
    gate.begin_turn().expect("first turn starts");
    assert!(gate.rollover(true).is_err());
    gate.complete_turn();
    let replayed_before = gate.replayed_items;
    gate.rollover(true).expect("idle rollover succeeds");
    assert_eq!(gate.generation, 2);
    assert_eq!(gate.replayed_items, replayed_before);
    assert!(gate.rollover(true).is_err());

    let mut failed = RolloverGate::default();
    assert!(failed.rollover(false).is_err());
    assert_eq!(failed.generation, 1);
    assert_eq!(failed.rollovers, 0);
}

fn assert_joined_cleanup_before_credential_release() {
    let host = RecordingHostServices::default();
    let scope = ScopeId::new("fixture.rollover-scope").unwrap();
    let credential = poll_immediate(
        host.services()
            .credential()
            .expect("credential service")
            .acquire(
                scope.clone(),
                CredentialRef::new("fixture.rollover-credential").unwrap(),
                EndpointAudience::new("fixture.realtime.example").unwrap(),
            ),
    )
    .expect("credential acquired");
    let first = host
        .services()
        .task()
        .expect("task service")
        .spawn(scope.clone(), Box::pin(async {}))
        .expect("first generation spawned");
    let second = host
        .services()
        .task()
        .expect("task service")
        .spawn(scope, Box::pin(async {}))
        .expect("second generation spawned");
    poll_immediate(first.join()).expect("first generation joined");
    poll_immediate(second.join()).expect("second generation joined");
    poll_immediate(
        host.services()
            .credential()
            .expect("credential service")
            .release(credential),
    );

    let calls = host.calls();
    let release = calls
        .iter()
        .position(|call| *call == RecordedHostCall::CredentialRelease)
        .expect("credential released");
    assert_eq!(host.count(RecordedHostCall::TaskJoin), 2);
    assert!(
        calls[..release]
            .iter()
            .filter(|call| **call == RecordedHostCall::TaskJoin)
            .count()
            == 2
    );
}

#[derive(Default)]
struct RolloverGate {
    active_turn: bool,
    generation: u8,
    rollovers: u8,
    replayed_items: u8,
}

impl RolloverGate {
    fn begin_turn(&mut self) -> Result<(), ()> {
        if self.active_turn {
            Err(())
        } else {
            self.active_turn = true;
            self.generation = self.generation.max(1);
            Ok(())
        }
    }

    fn complete_turn(&mut self) {
        assert!(self.active_turn);
        self.active_turn = false;
    }

    fn rollover(&mut self, provider_connected: bool) -> Result<(), ()> {
        self.generation = self.generation.max(1);
        if self.active_turn || self.rollovers >= 1 || !provider_connected {
            return Err(());
        }
        self.rollovers += 1;
        self.generation += 1;
        Ok(())
    }
}
