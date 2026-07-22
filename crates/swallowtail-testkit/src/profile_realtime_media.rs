use crate::{
    ConformanceAssertion, ConformanceReport, ProfilePreflightFixture, RealtimeMediaPreflightCase,
    RealtimeMediaPreflightFixture, SyntheticProfile, assert_common_contract,
};
use std::num::NonZeroU64;
use swallowtail_core::{
    DriverRole, ExecutionLayer, HostServiceKind, MediaDirection, OperationShape,
    PreflightDimension, ProviderRequestRef,
};
use swallowtail_runtime::{
    MediaChunk, MediaStreamId, MediaTranscript, ProviderObservation, QuotaObservation, QuotaState,
    RateLimitKind, RateLimitObservation, RealtimeMediaEvent, RealtimeMediaEventKind,
    RealtimeMediaResponseStatus, RealtimeMediaSessionState, RuntimeSessionId, RuntimeTurnId,
    TokenUsage,
};

mod lifecycle;

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::RealtimeMediaDirectSession;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    assert_preflight_matrix();
    let fixture = ProfilePreflightFixture::new(profile);
    let plan = fixture
        .preflight()
        .expect("realtime-media preflight succeeds");
    assert_eq!(
        plan.requirements().driver_role(),
        DriverRole::RealtimeMediaSession
    );
    assert_eq!(
        plan.requirements().execution_layer(),
        ExecutionLayer::DirectModelInference
    );
    assert_eq!(
        plan.requirements().operation_shape(),
        OperationShape::InteractiveSession
    );
    assert_eq!(
        plan.requirements()
            .realtime_media()
            .expect("media is bound")
            .model_id(),
        plan.model_id().expect("model is bound"),
    );
    assert!(
        fixture
            .driver()
            .required_host_services(DriverRole::RealtimeMediaSession)
            .all(|service| service != HostServiceKind::Process)
    );
    report.record(ConformanceAssertion::RealtimeMediaBoundary);

    assert_two_serial_turns();
    report.record(ConformanceAssertion::RealtimeMediaOrdering);
    report.record(ConformanceAssertion::ProviderEvidenceSeparated);

    lifecycle::assert_interruption_ends_session();
    report.record(ConformanceAssertion::RealtimeMediaInterruptionEndsSession);

    lifecycle::assert_joined_cleanup(&plan);
    report.record(ConformanceAssertion::SessionLifecycle);
    report.record(ConformanceAssertion::ConnectionScopedLeaseLifecycle);
    report.record(ConformanceAssertion::HostedEndpointCredentialBinding);
    report.record(ConformanceAssertion::DirectSessionNoResource);
    report
}

fn assert_preflight_matrix() {
    for (case, dimension) in [
        (
            RealtimeMediaPreflightCase::WrongRole,
            PreflightDimension::RealtimeMedia,
        ),
        (
            RealtimeMediaPreflightCase::WrongLayer,
            PreflightDimension::ExecutionLayer,
        ),
        (
            RealtimeMediaPreflightCase::WrongShape,
            PreflightDimension::OperationShape,
        ),
        (
            RealtimeMediaPreflightCase::MissingRoute,
            PreflightDimension::ModelRoute,
        ),
        (
            RealtimeMediaPreflightCase::WrongModel,
            PreflightDimension::ModelRoute,
        ),
        (
            RealtimeMediaPreflightCase::MissingFormat,
            PreflightDimension::Constraint,
        ),
        (
            RealtimeMediaPreflightCase::WrongChunkBound,
            PreflightDimension::Constraint,
        ),
        (
            RealtimeMediaPreflightCase::RejectedAccess,
            PreflightDimension::Access,
        ),
        (
            RealtimeMediaPreflightCase::MissingHostService,
            PreflightDimension::HostService,
        ),
    ] {
        let fixture = RealtimeMediaPreflightFixture::for_case(case);
        let failure = fixture
            .preflight()
            .expect_err("invalid media preflight must fail");
        assert_eq!(
            failure.dimension(),
            dimension,
            "wrong dimension for {case:?}"
        );
        assert_eq!(fixture.provider_side_effect_count(), 0);
    }
    RealtimeMediaPreflightFixture::for_case(RealtimeMediaPreflightCase::Canonical)
        .preflight()
        .expect("canonical media preflight succeeds");
}

fn assert_two_serial_turns() {
    let config = crate::realtime_media_fixture::realtime_media_config();
    let session_id = valid_session("profile-media-session");
    let mut state = RealtimeMediaSessionState::new(session_id.clone(), config.clone());
    let mut event_sequence = 1;
    for turn in 1..=2 {
        let input = valid_stream(&format!("input-{turn}"));
        for sequence in 1..=2 {
            let chunk = MediaChunk::new(
                session_id.clone(),
                input.clone(),
                nonzero(sequence),
                MediaDirection::Input,
                config.input_format(),
                vec![u8::try_from(turn).expect("turn fits"); 4],
                &config,
            )
            .expect("input chunk is valid");
            state.append_input(&chunk).expect("input appends");
        }
        let turn_id = valid_turn(&format!("media-turn-{turn}"));
        state
            .commit_input(turn_id.clone(), input)
            .expect("input commits");
        let output = MediaChunk::new(
            session_id.clone(),
            valid_stream(&format!("output-{turn}")),
            nonzero(1),
            MediaDirection::Output,
            config.output_format(),
            vec![9, 8, 7, 6],
            &config,
        )
        .expect("output chunk is valid");
        for kind in [
            RealtimeMediaEventKind::ResponseStarted,
            RealtimeMediaEventKind::OutputAudio(output),
            RealtimeMediaEventKind::TranscriptDelta(
                MediaTranscript::new("hel").expect("transcript is valid"),
            ),
            RealtimeMediaEventKind::TranscriptCompleted(
                MediaTranscript::new("hello").expect("transcript is valid"),
            ),
            RealtimeMediaEventKind::ProviderObservation(ProviderObservation::Usage(
                TokenUsage::new(Some(4), Some(3)),
            )),
            RealtimeMediaEventKind::ProviderObservation(ProviderObservation::RateLimit(
                RateLimitObservation::new(RateLimitKind::Requests, Some(100), Some(99), Some(60)),
            )),
            RealtimeMediaEventKind::ProviderObservation(ProviderObservation::Quota(
                QuotaObservation::new(QuotaState::Available),
            )),
            RealtimeMediaEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(
                ProviderRequestRef::new(format!("request-{turn}")).expect("request ref is valid"),
            )),
            RealtimeMediaEventKind::ResponseTerminal(RealtimeMediaResponseStatus::Completed),
        ] {
            state
                .record_response_event(&RealtimeMediaEvent::new(
                    nonzero(event_sequence),
                    turn_id.clone(),
                    kind,
                ))
                .expect("response event applies");
            event_sequence += 1;
        }
    }
    assert!(!state.is_reusable());
}

fn nonzero(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).expect("fixture sequence is nonzero")
}

fn valid_session(value: &str) -> RuntimeSessionId {
    RuntimeSessionId::new(value).expect("fixture session id is valid")
}

fn valid_stream(value: &str) -> MediaStreamId {
    MediaStreamId::new(value).expect("fixture stream id is valid")
}

fn valid_turn(value: &str) -> RuntimeTurnId {
    RuntimeTurnId::new(value).expect("fixture turn id is valid")
}
