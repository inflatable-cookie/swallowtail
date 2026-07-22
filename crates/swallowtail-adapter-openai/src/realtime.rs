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
    MediaFormat, OperationShape, PreflightPlan, RealtimeMediaConfig, TransportFamilyId,
};
use swallowtail_runtime::{HostServices, OpenRealtimeMediaSessionRequest, RuntimeFailure};

pub(crate) const DRIVER_ID: &str = "swallowtail.openai.realtime";
pub(crate) const MODEL_ID: &str = "gpt-realtime-2.1";
pub(crate) const REALTIME_PATH: &str = "/v1/realtime";

#[derive(Clone, Default)]
pub struct OpenAiRealtimeDriver;

impl OpenAiRealtimeDriver {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    fn validate(
        plan: &PreflightPlan,
        request: &OpenRealtimeMediaSessionRequest,
        services: &HostServices,
    ) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(rejected("plan belongs to a different driver"));
        }
        if plan.credential_mechanism() != &CredentialMechanism::ApiKey
            || plan.credential_reference().is_none()
            || plan.endpoint_audience().as_str() != crate::ENDPOINT_AUDIENCE
        {
            return Err(rejected("public API-key access is not exact"));
        }
        if plan
            .provider_id()
            .is_none_or(|provider| provider.as_str() != crate::INTEGRATION_FAMILY)
            || plan
                .model_id()
                .is_none_or(|model| model.as_str() != MODEL_ID)
            || plan.model_route_id().is_none()
            || plan.requirements().realtime_media().is_none()
        {
            return Err(rejected("model route is not the exact Realtime route"));
        }
        let required = [
            Capability::StreamingEvents,
            Capability::UsageReporting,
            Capability::Interruption,
            Capability::RealtimeMedia,
        ];
        if required.iter().any(|capability| {
            !plan
                .requirements()
                .capabilities()
                .any(|requirement| requirement.capability() == *capability)
        }) {
            return Err(rejected("capability requirements are incomplete"));
        }
        let cancellation = CapabilityConstraint::CancellationScope(
            swallowtail_core::CancellationScope::ActiveResponse,
        );
        let interruption = plan
            .requirements()
            .capabilities()
            .find(|requirement| requirement.capability() == Capability::Interruption)
            .expect("required interruption exists");
        if interruption.constraints().collect::<Vec<_>>() != [&cancellation] {
            return Err(rejected("cancellation scope is not ActiveResponse"));
        }
        if request.config() != &realtime_config()
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
                "swallowtail.openai.realtime_deadline_elapsed",
                "OpenAI Realtime deadline elapsed before provider work",
            ));
        }
        Ok(())
    }
}

#[must_use]
pub fn openai_realtime_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        swallowtail_core::IntegrationFamilyId::new(crate::INTEGRATION_FAMILY)
            .expect("static family id is valid"),
        TransportFamilyId::new("realtime-websocket").expect("static transport id is valid"),
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

pub(crate) fn realtime_config() -> RealtimeMediaConfig {
    let format = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(24_000).expect("sample rate is nonzero"),
        NonZeroU16::new(1).expect("channel count is nonzero"),
    );
    RealtimeMediaConfig::new(
        format,
        format,
        NonZeroU64::new(32_768).expect("chunk bound is nonzero"),
        NonZeroU32::new(2).expect("turn bound is nonzero"),
    )
}

fn rejected(reason: &'static str) -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_preflight_rejected",
        match reason {
            "plan belongs to a different driver" => {
                "OpenAI Realtime plan belongs to a different driver"
            }
            "public API-key access is not exact" => {
                "OpenAI Realtime requires the exact public API-key access boundary"
            }
            "model route is not the exact Realtime route" => {
                "OpenAI Realtime requires the exact gpt-realtime-2.1 route"
            }
            "capability requirements are incomplete" => {
                "OpenAI Realtime capability requirements are incomplete"
            }
            "cancellation scope is not ActiveResponse" => {
                "OpenAI Realtime requires active-response cancellation"
            }
            "media format or bounds differ from preflight" => {
                "OpenAI Realtime media format or bounds differ from preflight"
            }
            _ => "OpenAI Realtime required host services are unavailable",
        },
    )
}
