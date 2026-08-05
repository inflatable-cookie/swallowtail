use super::super::prepared::access_policy;
use crate::failure::failure;
use crate::local_server::protocol::{
    InteractiveSessionRecord, RestFailureKind, RestReply, decode_interactive_session, decode_rest,
};
use std::future::Future;
use swallowtail_core::{
    Capability, CapabilityConstraint, FailureClassification, FailureKind, FailureOrigin,
    FailureRecovery, InstanceOwnership, PreflightPlan,
};
use swallowtail_runtime::{
    HostServices, OpenSessionRequest, RequestId, ResumeSessionRequest, RuntimeFailure, ScopeId,
    validate_session_plan_agreement,
};

pub(super) fn validate_open(
    driver: &super::super::super::KimiLocalServerDriver,
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_common(
        driver,
        plan,
        request.plan_agreement(),
        request.options(),
        services,
    )?;
    if request.working_resource().is_none() {
        return Err(unsupported("resource-free session"));
    }
    validate_deadline(request.deadline(), services)
}

pub(super) fn validate_projected_open(
    driver: &super::super::super::KimiLocalServerDriver,
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    validate_driver_agreement(driver, request.plan_agreement(), request.options())?;
    validate_detachment(driver, plan)?;
    validate_services(services)?;
    if request.working_resource().is_none() {
        return Err(unsupported("resource-free session"));
    }
    validate_deadline(request.deadline(), services)
}

pub(super) fn validate_resume(
    driver: &super::super::super::KimiLocalServerDriver,
    plan: &PreflightPlan,
    request: &ResumeSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_common(
        driver,
        plan,
        request.plan_agreement(),
        request.options(),
        services,
    )?;
    if !request.resume_binding().matches_attachment(
        plan,
        request.working_resource(),
        request.access_policy(),
    ) {
        return Err(binding_failure());
    }
    validate_deadline(request.deadline(), services)
}

fn validate_common(
    driver: &super::super::super::KimiLocalServerDriver,
    plan: &PreflightPlan,
    agreement: &swallowtail_runtime::SessionPlanAgreement,
    options: &swallowtail_runtime::SessionOptions,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    validate_session_plan_agreement(plan, agreement)?;
    validate_driver_agreement(driver, agreement, options)?;
    validate_detachment(driver, plan)?;
    validate_services(services)
}

fn validate_detachment(
    driver: &super::super::super::KimiLocalServerDriver,
    plan: &PreflightPlan,
) -> Result<(), RuntimeFailure> {
    let configured = driver.configuration()?.active_turn_detachment();
    let requirement = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == Capability::ActiveOperationDetachment);
    if !configured && requirement.is_none() {
        return Ok(());
    }
    let valid = configured
        && plan.ownership() == InstanceOwnership::ExternalAttached
        && requirement.is_some_and(|requirement| {
            requirement.constraints().collect::<Vec<_>>()
                == [&CapabilityConstraint::OperationDetachmentScope(
                    swallowtail_core::OperationDetachmentScope::ActiveTurn,
                )]
        })
        && plan
            .requirements()
            .capabilities()
            .any(|required| required.capability() == Capability::ProviderDurableRetention)
        && plan.requirements().extension_namespaces().next().is_none();
    if valid {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.kimi.local_server.detachment_plan_mismatch",
            "Kimi local-server detachment does not match the immutable session plan",
        ))
    }
}

fn validate_driver_agreement(
    driver: &super::super::super::KimiLocalServerDriver,
    agreement: &swallowtail_runtime::SessionPlanAgreement,
    options: &swallowtail_runtime::SessionOptions,
) -> Result<(), RuntimeFailure> {
    let expected = access_policy(driver.configuration()?.permission_mode());
    if agreement.access_policy() != &expected
        || agreement.provider_state_policy()
            != Some(swallowtail_core::SessionProviderStatePolicy::DurableProviderSessionPreserved)
        || agreement.harness_configuration_posture()
            != Some(swallowtail_core::HarnessConfigurationPosture::Ambient)
        || options.developer_instructions().is_some()
        || options.tools().len() != 0
    {
        return Err(binding_failure());
    }
    Ok(())
}

fn validate_services(services: &HostServices) -> Result<(), RuntimeFailure> {
    for required in [
        swallowtail_core::HostServiceKind::Task,
        swallowtail_core::HostServiceKind::BlockingWork,
        swallowtail_core::HostServiceKind::Time,
        swallowtail_core::HostServiceKind::Network,
        swallowtail_core::HostServiceKind::Credential,
        swallowtail_core::HostServiceKind::WorkingResource,
    ] {
        if !services.available_kinds().contains(&required) {
            return Err(unsupported("required host service"));
        }
    }
    Ok(())
}

pub(super) fn require_interactive_session(
    response: crate::local_server::transport::Response,
) -> Result<InteractiveSessionRecord, RuntimeFailure> {
    if response.status != 200 {
        return match decode_rest(response.status, &response.body) {
            Ok(RestReply::Failure(kind)) => Err(provider_rejected(kind)),
            _ => Err(protocol_failure()),
        };
    }
    decode_interactive_session(&response.body)
}

pub(super) async fn before_deadline<T, F>(
    work: F,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
) -> Result<T, RuntimeFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    let Some(deadline) = deadline else {
        return work.await;
    };
    let mut work = Box::pin(work);
    let mut timer = services
        .time()
        .expect("validated time service")
        .wait_until(deadline);
    std::future::poll_fn(|context| {
        if let std::task::Poll::Ready(result) = work.as_mut().poll(context) {
            return std::task::Poll::Ready(result);
        }
        if timer.as_mut().poll(context).is_ready() {
            return std::task::Poll::Ready(Err(timeout()));
        }
        std::task::Poll::Pending
    })
    .await
}

fn validate_deadline(
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if deadline
        .is_some_and(|deadline| services.time().expect("validated").now() >= deadline.instant())
    {
        Err(timeout())
    } else {
        Ok(())
    }
}

pub(super) fn scope(kind: &str, request: &RequestId) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("kimi-local:{kind}:{}", request.as_str())).map_err(|_| protocol_failure())
}

fn unsupported(feature: &'static str) -> RuntimeFailure {
    crate::failure::unsupported(feature)
}

pub(super) fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.session_protocol_failed",
        "Kimi local-server session protocol failed",
    )
}

pub(super) fn binding_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.session_binding_mismatch",
        "Kimi local-server session binding does not match preflight",
    )
}

fn provider_rejected(kind: RestFailureKind) -> RuntimeFailure {
    let (failure_kind, recovery) = match kind {
        RestFailureKind::Validation => (
            FailureKind::InvalidRequest,
            FailureRecovery::InputChangeRequired,
        ),
        RestFailureKind::Unauthorized => (
            FailureKind::AuthenticationRejected,
            FailureRecovery::ReauthenticationRequired,
        ),
        RestFailureKind::Missing => (
            FailureKind::ResourceNotFound,
            FailureRecovery::ConfigurationChangeRequired,
        ),
        RestFailureKind::Busy => (
            FailureKind::ProviderUnavailable,
            FailureRecovery::RetryMaySucceed,
        ),
        RestFailureKind::Server => (
            FailureKind::ProviderUnavailable,
            FailureRecovery::RetryMaySucceed,
        ),
    };
    failure(
        "swallowtail.kimi.local_server.session_rejected",
        "Kimi local server rejected the session request",
    )
    .with_failure_classification(FailureClassification::new(
        FailureOrigin::Harness,
        failure_kind,
        recovery,
    ))
}

fn timeout() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.session_timed_out",
        "Kimi local-server session request timed out",
    )
}

#[cfg(test)]
mod failure_classification_tests {
    use super::*;

    #[test]
    fn typed_rest_failure_preserves_harness_authentication_meaning() {
        let failure = provider_rejected(RestFailureKind::Unauthorized);

        assert_eq!(
            failure.diagnostic().failure_classification().origin(),
            FailureOrigin::Harness
        );
        assert_eq!(
            failure.diagnostic().failure_classification().kind(),
            FailureKind::AuthenticationRejected
        );
    }
}
