use super::super::access::SessionAccess;
use super::super::session::{CursorState, KimiInteractiveSession, SessionCancellation};
use super::validation::{binding_failure, protocol_failure};
use crate::local_server::protocol::InteractiveSessionRecord;
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    ConfiguredInstance, PreflightPlan, ProviderSessionBindingOrigin, SessionRef,
};
use swallowtail_runtime::{
    HostServices, PreparedAccessEvidence, ProviderSessionManagementBinding, RequestId,
    RuntimeFailure, RuntimeSessionId, SessionResumeBinding,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn build_handle(
    driver: &super::super::super::KimiLocalServerDriver,
    plan: &PreflightPlan,
    request_id: RequestId,
    resource: swallowtail_runtime::WorkingResourceRef,
    options: swallowtail_runtime::SessionOptions,
    access_policy: swallowtail_core::SessionAccessPolicy,
    record: InteractiveSessionRecord,
    access: SessionAccess,
    services: HostServices,
    management: Option<(ConfiguredInstance, PreparedAccessEvidence)>,
    origin: ProviderSessionBindingOrigin,
) -> Result<KimiInteractiveSession, RuntimeFailure> {
    let provider_ref = SessionRef::new(&record.id).map_err(|_| protocol_failure())?;
    let resume = SessionResumeBinding::new(
        provider_ref.clone(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id()
            .expect("validated model route")
            .clone(),
        plan.model_id().expect("validated model").clone(),
        resource.clone(),
        access_policy,
    );
    let management = management
        .map(|(instance, evidence)| {
            ProviderSessionManagementBinding::from_bound_session(
                provider_ref.clone(),
                &crate::kimi_local_server_descriptor(),
                &instance,
                evidence,
                Some(resource),
                origin,
            )
            .map_err(|_| binding_failure())
        })
        .transpose()?;
    let active = Arc::new(Mutex::new(None));
    let runtime_id = RuntimeSessionId::new(format!("kimi-local:{}", request_id.as_str()))
        .map_err(|_| protocol_failure())?;
    Ok(KimiInteractiveSession {
        request_id,
        runtime_id,
        provider_ref,
        provider_session_id: record.id,
        resume,
        management,
        model_id: plan.model_id().expect("validated model").clone(),
        options,
        configuration: driver.configuration()?.clone(),
        cursor: Arc::new(Mutex::new(CursorState {
            seq: record.last_seq,
            epoch: None,
        })),
        access: Some(access),
        services,
        transport: driver.transport.clone(),
        active: Arc::clone(&active),
        cancellation: SessionCancellation::new(active),
    })
}
