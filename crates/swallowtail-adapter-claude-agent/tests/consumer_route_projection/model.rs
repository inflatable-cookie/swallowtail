use crate::fixtures::{AGENT_HOST, agent_session};
use crate::naming::{rows, semantic_id, source};
use crate::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use swallowtail_adapter_claude_agent::ClaudeAgentProjectionOpenFailure;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    CleanupOutcome, ConsumerRouteProjectionSourceKind, ConsumerRouteValueKind, RequestId,
};

const PREPARED: &str = "claude-agent.projection.prepared";
const ACTIVE: &str = "claude-agent.projection.active";

#[test]
fn matching_model_survives_open_and_publishes_only_the_projected_active_row() {
    let fixture = FixtureHost::new(Scenario::Success, "0.61.0");
    let outcome = block_on(agent_session(None, false).open_session_with_projection(
        source(PREPARED),
        source(ACTIVE),
        fixture.cleanup_request(),
        fixture.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
    ))
    .map_err(|failure| failure.failure().diagnostic().code())
    .expect("matching model opens");
    let options = outcome
        .session()
        .negotiated_model_options()
        .expect("exact model options survive");
    assert_eq!(options.current_value(), "claude-sonnet-4-6");
    assert_eq!(
        options
            .options()
            .map(|option| (option.value(), option.display_name()))
            .collect::<Vec<_>>(),
        [
            ("default", Some("Default")),
            ("claude-sonnet-4-6", Some("claude-sonnet-4-6")),
        ]
    );
    assert!(rows(outcome.contribution()).contains("feature.negotiated-model-options-observation"));
    let model = outcome
        .contribution()
        .active_session_rows()
        .find(|row| semantic_id(row.identity()) == "feature.negotiated-model-options-observation")
        .expect("model observation row exists");
    assert_eq!(model.source().id().as_str(), ACTIVE);
    assert_eq!(
        model.source().kind(),
        ConsumerRouteProjectionSourceKind::ActiveSessionObservation
    );
    assert!(model.state_support().observed());
    assert!(!model.state_support().provider_effective());
    assert!(!model.state_support().requested());
    assert!(model.mutation_authority().source().is_none());
    let value = model.control_value().expect("observation descriptor");
    assert_eq!(value.kind(), ConsumerRouteValueKind::Observation);
    assert_eq!(
        value.omission(),
        swallowtail_runtime::ConsumerRouteOmissionSemantics::NotSelectable
    );
    let (session, _) = outcome.into_parts();
    assert_eq!(
        block_on(session.close(
            fixture.cleanup_request(),
            fixture.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        )),
        CleanupOutcome::Clean
    );
    assert_eq!(fixture.credential_releases(), 1);
    assert_eq!(fixture.resource_releases(), 1);
}

#[test]
fn prepared_contribution_omits_the_observation_row() {
    let contribution = agent_session(None, false)
        .consumer_route_projection_contribution(source("claude-agent.projection.prepared-only"))
        .expect("prepared session contributes");
    assert!(!rows(&contribution).contains("feature.negotiated-model-options-observation"));
    assert!(!rows(&contribution).contains("feature.model-catalogue"));
}

#[test]
fn invalid_model_is_ignored_by_preserved_open_and_closes_projected_open() {
    for scenario in [
        Scenario::ModelMalformed,
        Scenario::ModelDuplicate,
        Scenario::ModelUnadvertised,
        Scenario::ModelUnbounded,
    ] {
        let preserved = FixtureHost::new(scenario, "0.61.0");
        let handle = block_on(agent_session(None, false).open_session(
            preserved.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        ))
        .expect("preserved open stays compatible");
        assert!(handle.negotiated_model_options().is_none());
        assert_eq!(
            block_on(handle.close(
                preserved.cleanup_request(),
                preserved.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
            )),
            CleanupOutcome::Clean
        );
        assert_eq!(preserved.credential_releases(), 1);
        assert_eq!(preserved.resource_releases(), 1);

        let projected = FixtureHost::new(scenario, "0.61.0");
        let failure = block_on(agent_session(None, false).open_session_with_projection(
            source(PREPARED),
            source(ACTIVE),
            projected.cleanup_request(),
            projected.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        ))
        .err()
        .expect("projected open rejects invalid model evidence");
        assert!(matches!(
            failure,
            ClaudeAgentProjectionOpenFailure::Runtime(_)
        ));
        assert_eq!(
            failure.failure().diagnostic().code(),
            "swallowtail.negotiated_model_options.invalid"
        );
        assert!(failure.rejected_contribution().is_none());
        assert_eq!(projected.credential_releases(), 1);
        assert_eq!(projected.resource_releases(), 1);
    }
}

#[test]
fn missing_model_entry_fails_both_opens_through_confirmation() {
    let preserved = FixtureHost::new(Scenario::ModelEntryMissing, "0.61.0");
    let preserved_failure =
        match block_on(agent_session(None, false).open_session(
            preserved.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        )) {
            Ok(_) => panic!("preserved open should reject a missing model entry"),
            Err(failure) => failure,
        };
    let projected = FixtureHost::new(Scenario::ModelEntryMissing, "0.61.0");
    let projected_failure = match block_on(agent_session(None, false).open_session_with_projection(
        source(PREPARED),
        source(ACTIVE),
        projected.cleanup_request(),
        projected.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
    )) {
        Ok(_) => panic!("projected open should reject a missing model entry"),
        Err(failure) => failure,
    };
    assert_eq!(
        preserved_failure.diagnostic().code(),
        "swallowtail.claude_agent.acp.config_option_missing"
    );
    assert_eq!(
        preserved_failure.diagnostic().code(),
        projected_failure.failure().diagnostic().code()
    );
    assert!(matches!(
        projected_failure,
        ClaudeAgentProjectionOpenFailure::Runtime(_)
    ));
    assert!(projected_failure.rejected_contribution().is_none());
    assert_eq!(
        preserved.credential_releases(),
        projected.credential_releases()
    );
    assert_eq!(preserved.resource_releases(), projected.resource_releases());
    assert_eq!(preserved.credential_releases(), 1);
    assert_eq!(preserved.resource_releases(), 1);
}

#[test]
fn rejected_reasoning_omits_the_observation_row() {
    let fixture = FixtureHost::new(Scenario::ReasoningMismatchAdvertised, "0.61.0");
    let failure = block_on(
        agent_session(Some("low"), false).open_session_with_projection(
            source(PREPARED),
            source(ACTIVE),
            fixture.cleanup_request(),
            fixture.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        ),
    )
    .err()
    .expect("exact mismatch rejects");
    assert!(
        !rows(
            failure
                .rejected_contribution()
                .expect("rejected contribution exists")
        )
        .contains("feature.negotiated-model-options-observation")
    );
    assert_eq!(fixture.credential_releases(), 1);
    assert_eq!(fixture.resource_releases(), 1);
}

#[test]
fn load_and_resume_omit_negotiated_model_options() {
    let host_id = ExecutionHostId::new(AGENT_HOST).expect("host is valid");
    let session = agent_session(None, false);
    let opened = FixtureHost::new(Scenario::Success, "0.61.0");
    let handle = block_on(session.open_session(opened.services(host_id.clone())))
        .expect("open supplies a resume binding");
    let binding = handle
        .resume_binding()
        .expect("open session returns resume binding")
        .clone();
    assert!(handle.negotiated_model_options().is_some());
    assert_eq!(
        block_on(handle.close(opened.cleanup_request(), opened.services(host_id.clone()))),
        CleanupOutcome::Clean
    );

    let load_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let loaded = block_on(
        session
            .load_session(
                RequestId::new("claude-agent.projection.load").expect("request is valid"),
                binding.clone(),
                load_host.services(host_id.clone()),
            )
            .expect("load prepares"),
    )
    .expect("session loads");
    let (_, loaded_handle) = loaded.into_parts();
    assert!(loaded_handle.negotiated_model_options().is_none());
    assert_eq!(
        block_on(loaded_handle.close(
            load_host.cleanup_request(),
            load_host.services(host_id.clone()),
        )),
        CleanupOutcome::Clean
    );

    let resume_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let resumed = block_on(
        session
            .resume_session(
                RequestId::new("claude-agent.projection.resume").expect("request is valid"),
                binding,
                resume_host.services(host_id.clone()),
            )
            .expect("resume prepares"),
    )
    .expect("session resumes");
    assert!(resumed.negotiated_model_options().is_none());
    assert_eq!(
        block_on(resumed.close(resume_host.cleanup_request(), resume_host.services(host_id))),
        CleanupOutcome::Clean
    );
}
