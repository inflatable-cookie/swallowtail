use super::route_ledger::{LOCAL, observed_tuples};
use super::*;
use crate::fixture::session_input;
use std::collections::BTreeMap;
use swallowtail_adapter_kimi::{
    KimiLocalServerPermissionMode, KimiLocalServerRunInput, KimiLocalServerSessionConfiguration,
    KimiModelSelection,
};
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, ReasoningMode};
use swallowtail_runtime::{ConsumerRouteProjectionSourceId, OperationContent};
use swallowtail_testkit::{
    ConsumerRouteLedgerClaim, assert_consumer_route_ledger_emitted_by_facade,
};

const ROUTE: &str = "kimi-code.local-server";
const CATALOGUE: &str = "catalogue";
const RUN: &str = "run.owned";
const SESSION: &str = "session.manual";
const DETACHED: &str = "session.detached";
const ARCHIVE: &str = "archive";
const RESTORE: &str = "restore";

#[test]
fn local_server_ledger_matches_each_prepared_facade() {
    let server = FixtureServer::start_with_version("0.29.2");
    let host = FixtureHost::new(&server);
    let execution_host = value(ExecutionHostId::new, "kimi.ledger.local.host");
    let prepared = prepare_attached_for_version(
        execution_host.clone(),
        "0.29.2",
        host.services(execution_host.clone(), false),
    );
    let catalogue = prepared
        .prepare_catalogue(KimiLocalServerCatalogueInput::new(value(
            RequestId::new,
            "ledger-catalogue",
        )))
        .expect("catalogue prepares")
        .consumer_route_projection_contribution(source("kimi.ledger.local.catalogue"))
        .expect("catalogue projects");
    let session = prepared
        .prepare_session(
            session_input(
                "ledger-session",
                KimiLocalServerSessionConfiguration::new(KimiLocalServerPermissionMode::Manual),
            )
            .with_reasoning(value(ReasoningMode::new, "high")),
        )
        .expect("session prepares")
        .consumer_route_projection_contribution(source("kimi.ledger.local.session"))
        .expect("session projects");
    let detached = prepared
        .prepare_session(session_input(
            "ledger-detached",
            KimiLocalServerSessionConfiguration::new(KimiLocalServerPermissionMode::Auto)
                .with_active_turn_detachment(),
        ))
        .expect("detached session prepares")
        .consumer_route_projection_contribution(source("kimi.ledger.local.detached"))
        .expect("detached session projects");
    let management_binding = binding(&prepared);
    let archive = prepared
        .prepare_archive_session(KimiLocalServerSessionManagementInput::new(
            value(RequestId::new, "ledger-archive"),
            management_binding.clone(),
        ))
        .expect("archive prepares")
        .consumer_route_projection_contribution(source("kimi.ledger.local.archive"))
        .expect("archive projects");
    let restore = prepared
        .prepare_restore_session(KimiLocalServerSessionManagementInput::new(
            value(RequestId::new, "ledger-restore"),
            management_binding,
        ))
        .expect("restore prepares")
        .consumer_route_projection_contribution(source("kimi.ledger.local.restore"))
        .expect("restore projects");

    let owned_services = host.services(execution_host.clone(), true);
    let owned = block_on(start_kimi_local_server_owned(
        KimiLocalServerOwnedInput::new(
            attached_input_for_version(execution_host, "0.29.2"),
            value(InstanceTargetRef::new, "fixture.kimi.executable"),
        ),
        probe(),
        owned_services,
    ))
    .expect("owned Kimi starts");
    let run = owned
        .prepared()
        .prepare_run(
            KimiLocalServerRunInput::new(
                value(RequestId::new, "ledger-run"),
                KimiModelSelection::new(
                    value(ModelRouteId::new, "fixture.kimi.ledger.run"),
                    value(ModelRouteRevision::new, "1"),
                    value(ModelId::new, "kimi-k2.5"),
                ),
                OperationContent::new("projection fixture").expect("content"),
                value(WorkingResourceRef::new, "fixture.kimi.workspace"),
                Deadline::at(MonotonicInstant::from_ticks(100)),
                KimiLocalServerSessionConfiguration::new(KimiLocalServerPermissionMode::Manual),
            )
            .with_reasoning(value(ReasoningMode::new, "high"))
            .accept_managed_recovery()
            .with_one_stream_reattachment(),
        )
        .expect("run prepares")
        .consumer_route_projection_contribution(source("kimi.ledger.local.run"))
        .expect("run projects");
    assert_eq!(block_on(owned.close()), CleanupOutcome::Clean);

    let observed = BTreeMap::from([
        (CATALOGUE, observed_tuples(ROUTE, &catalogue)),
        (RUN, observed_tuples(ROUTE, &run)),
        (SESSION, observed_tuples(ROUTE, &session)),
        (DETACHED, observed_tuples(ROUTE, &detached)),
        (ARCHIVE, observed_tuples(ROUTE, &archive)),
        (RESTORE, observed_tuples(ROUTE, &restore)),
    ]);
    assert_consumer_route_ledger_emitted_by_facade(
        &observed,
        LOCAL
            .iter()
            .map(|(shape, semantic, emitted)| ConsumerRouteLedgerClaim {
                identity: (
                    ROUTE.to_owned(),
                    (*shape).to_owned(),
                    (*semantic).to_owned(),
                ),
                emitted_by: if *emitted {
                    local_emitters(shape, semantic)
                } else {
                    &[]
                },
            }),
    );
}

fn source(value: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(value).expect("source")
}

fn local_emitters(shape: &str, semantic: &str) -> &'static [&'static str] {
    match semantic {
        "feature.model-catalogue" => &[CATALOGUE],
        "feature.structured-run"
        | "feature.provider-managed-recovery"
        | "feature.stream-reattachment"
        | "feature.owned-runtime-lifecycle"
        | "control.managed-recovery"
        | "control.stream-reattachment" => &[RUN],
        "feature.interactive-session" => &[SESSION, DETACHED],
        "feature.permission-exchange" | "feature.question-exchange" => &[SESSION],
        "feature.resume-session" => &[SESSION, DETACHED],
        "control.active-turn-detachment" => &[DETACHED],
        "feature.provider-session-archive" => &[ARCHIVE],
        "feature.provider-session-restore" => &[RESTORE],
        "feature.prepared-facade" => &[CATALOGUE, RUN, SESSION, DETACHED, ARCHIVE, RESTORE],
        "feature.reasoning-selection" => &[RUN, SESSION],
        "feature.streaming-events"
        | "feature.cancellation-or-interruption"
        | "feature.working-resource"
        | "feature.persistent-session-posture"
        | "feature.activity-observation" => &[RUN, SESSION, DETACHED],
        "control.reasoning-selection" => match shape {
            "structured-run" => &[RUN],
            "interactive-session" => &[SESSION],
            other => panic!("unexpected reasoning shape {other}"),
        },
        "control.model-selection"
        | "control.permission-mode"
        | "control.provider-profile"
        | "control.disabled-tools" => match shape {
            "structured-run" => &[RUN],
            "interactive-session" => &[SESSION, DETACHED],
            other => panic!("unexpected control shape {other}"),
        },
        other => panic!("unmapped local-server ledger semantic {other}"),
    }
}
