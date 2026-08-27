use crate::support::FixtureHost;
use swallowtail_adapter_cline::ClineAcpDriver;
use swallowtail_core::{HarnessMode, ResourceAccess};
use swallowtail_runtime::{OpenSessionRequest, RequestId, SessionOptions, SessionPlanAgreement};

pub(crate) fn driver() -> ClineAcpDriver {
    ClineAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("cline.fixture.isolated")
            .expect("valid environment"),
    )
}

pub(crate) fn plan_open_request(
    id: &str,
    resource: swallowtail_runtime::WorkingResourceRef,
) -> OpenSessionRequest {
    OpenSessionRequest::new(
        RequestId::new(id).expect("valid request"),
        resource,
        None,
        SessionPlanAgreement::explicit(
            swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
            Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
            Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
        ),
    )
    .with_options(SessionOptions::default().with_harness_mode(HarnessMode::Plan))
}

pub(crate) fn wire_methods(host: &FixtureHost) -> Vec<String> {
    host.writes()
        .iter()
        .filter_map(|message| {
            message
                .get("method")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })
        .collect()
}

pub(crate) fn config_sets(host: &FixtureHost) -> Vec<serde_json::Value> {
    host.writes()
        .into_iter()
        .filter(|message| {
            message.get("method").and_then(serde_json::Value::as_str)
                == Some("session/set_config_option")
        })
        .collect()
}
