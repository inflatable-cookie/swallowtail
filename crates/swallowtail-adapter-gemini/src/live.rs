mod access;
mod handle;
mod lifecycle;
mod pump;
mod session;
mod worker;

use crate::failure::failure;
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, AudioEncoding, Capability, CapabilityConstraint,
    CredentialMechanism, DriverDescriptor, DriverRole, ExecutionLayer, HostServiceKind,
    MediaFormat, OperationShape, PlannedConnectionRolloverPolicy, PreflightPlan,
    RealtimeMediaConfig, TransportFamilyId,
};
use swallowtail_runtime::{
    HostServices, OpenRealtimeMediaSessionRequest, RuntimeFailure,
    validate_planned_connection_rollover_plan,
};

pub(crate) const DRIVER_ID: &str = "swallowtail.gemini.live";
pub(crate) const MODEL_ID: &str = "gemini-3.1-flash-live-preview";
pub(crate) const MODEL_RESOURCE: &str = "models/gemini-3.1-flash-live-preview";
pub(crate) const LIVE_PATH: &str =
    "/ws/google.ai.generativelanguage.v1beta.GenerativeService.BidiGenerateContent";
pub(crate) const AUDIENCE: &str = "generativelanguage.googleapis.com";
const ACCESS_PROFILE: &str = "gemini.authorization-api-key.project";
const PROTOCOL_FACADE: &str =
    "google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent";
const INSTANCE_POLICY: &str = "gemini-live-preview-authorization-key-manual-audio";

#[derive(Clone, Default)]
pub struct GeminiLiveDriver;

impl GeminiLiveDriver {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    fn validate(
        plan: &PreflightPlan,
        request: &OpenRealtimeMediaSessionRequest,
        services: &HostServices,
    ) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID
            || plan.protocol_facade_id().as_str() != PROTOCOL_FACADE
            || plan.instance_policy_id().as_str() != INSTANCE_POLICY
        {
            return Err(rejected("driver or preview facade is not exact"));
        }
        if plan.access_profile_id().as_str() != ACCESS_PROFILE
            || plan.credential_mechanism() != &CredentialMechanism::ApiKey
            || plan.credential_reference().is_none()
            || plan.endpoint_audience().as_str() != AUDIENCE
        {
            return Err(rejected("authorization API-key access is not exact"));
        }
        if plan
            .provider_id()
            .is_none_or(|provider| provider.as_str() != "gemini")
            || plan
                .model_id()
                .is_none_or(|model| model.as_str() != MODEL_ID)
            || plan.model_route_id().is_none()
        {
            return Err(rejected("preview model route is not exact"));
        }
        validate_capabilities(plan)?;
        validate_planned_connection_rollover_plan(plan, request.planned_connection_rollover())?;
        if request.planned_connection_rollover()
            != PlannedConnectionRolloverPolicy::Bounded(NonZeroU32::new(1).unwrap())
        {
            return Err(rejected("rollover bound is not exactly one"));
        }
        if request.config() != &live_config()
            || plan
                .requirements()
                .realtime_media()
                .map(|media| media.config())
                != Some(request.config())
        {
            return Err(rejected("media format or bounds differ from preflight"));
        }
        if services.task().is_none()
            || services.blocking_work().is_none()
            || services.time().is_none()
            || services.network().is_none()
            || services.credential().is_none()
        {
            return Err(rejected("required host services are unavailable"));
        }
        if request
            .deadline()
            .is_some_and(|deadline| services.time().expect("validated").now() >= deadline.instant())
        {
            return Err(failure(
                "swallowtail.gemini.live_deadline_elapsed",
                "Gemini Live deadline elapsed before provider work",
            ));
        }
        Ok(())
    }
}

fn validate_capabilities(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    for capability in [
        Capability::StreamingEvents,
        Capability::UsageReporting,
        Capability::Interruption,
        Capability::RealtimeMedia,
        Capability::PlannedConnectionRollover,
    ] {
        if !plan
            .requirements()
            .capabilities()
            .any(|required| required.capability() == capability)
        {
            return Err(rejected("capability requirements are incomplete"));
        }
    }
    let cancellation = CapabilityConstraint::CancellationScope(
        swallowtail_core::CancellationScope::ActiveResponse,
    );
    let interruption = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == Capability::Interruption)
        .expect("required interruption exists");
    if interruption.constraints().collect::<Vec<_>>() != [&cancellation] {
        return Err(rejected("cancellation scope is not ActiveResponse"));
    }
    Ok(())
}

#[must_use]
pub fn gemini_live_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION")).expect("package version is valid"),
        ),
        swallowtail_core::IntegrationFamilyId::new("gemini").expect("family id is valid"),
        TransportFamilyId::new("gemini-live-raw-websocket").expect("transport id is valid"),
    )
    .with_roles([DriverRole::RealtimeMediaSession])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(
        DriverRole::RealtimeMediaSession,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ],
    )
}

pub(crate) fn live_config() -> RealtimeMediaConfig {
    let input = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(16_000).unwrap(),
        NonZeroU16::new(1).unwrap(),
    );
    let output = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(24_000).unwrap(),
        NonZeroU16::new(1).unwrap(),
    );
    RealtimeMediaConfig::new(
        input,
        output,
        NonZeroU64::new(32_768).unwrap(),
        NonZeroU32::new(2).unwrap(),
    )
}

fn rejected(reason: &'static str) -> RuntimeFailure {
    failure(
        "swallowtail.gemini.live_preflight_rejected",
        match reason {
            "driver or preview facade is not exact" => {
                "Gemini Live requires the exact preview driver, facade, and instance policy"
            }
            "authorization API-key access is not exact" => {
                "Gemini Live requires the exact project authorization API-key access profile"
            }
            "preview model route is not exact" => {
                "Gemini Live requires the exact Gemini 3.1 Flash Live preview route"
            }
            "rollover bound is not exactly one" => {
                "Gemini Live requires exactly one planned connection rollover"
            }
            "media format or bounds differ from preflight" => {
                "Gemini Live media format or bounds differ from preflight"
            }
            "capability requirements are incomplete" => {
                "Gemini Live capability requirements are incomplete"
            }
            "cancellation scope is not ActiveResponse" => {
                "Gemini Live requires active-response cancellation"
            }
            _ => "Gemini Live required host services are unavailable",
        },
    )
}
