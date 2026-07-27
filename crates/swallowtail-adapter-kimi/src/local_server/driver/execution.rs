use super::KimiLocalServerDriver;
use super::control::{cancelled_or_expired, wait_after_dispatch, wait_before_dispatch};
use super::response::{action_path, classify_response, outcome, with_cleanup};
use crate::failure::failure;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_core::{
    CredentialMechanism, ExtensionNamespace, ProviderSessionAffectedScope,
    ProviderSessionManagementEffect,
};
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, EndpointRef, HostServices, ImmediateCancellation,
    ProviderSessionManagementAgreement, ProviderSessionManagementOutcome,
    ProviderSessionManagementPlan, RequestId, RuntimeFailure, ScopeId,
};

use crate::local_server::transport::Request;

const DRIVER_ID: &str = "swallowtail.kimi.local-server";

impl KimiLocalServerDriver {
    pub(super) async fn execute(
        &self,
        plan: ProviderSessionManagementPlan,
        agreement: &ProviderSessionManagementAgreement,
        cancellation: &ImmediateCancellation,
        request_id: &RequestId,
        services: HostServices,
    ) -> Result<ProviderSessionManagementOutcome, RuntimeFailure> {
        validate_plan(&plan)?;
        let action = agreement.action();
        if cancelled_or_expired(agreement, cancellation, &services)? {
            return Ok(outcome(
                agreement,
                ProviderSessionManagementEffect::failed_before_effect(action),
            ));
        }
        if agreement.binding().working_resource().is_none() {
            return Err(failure(
                "swallowtail.kimi.local_server.state_root_missing",
                "Kimi local-server lifecycle requires its bound state-root identity",
            ));
        }

        let scope = operation_scope(request_id)?;
        let endpoint_ref =
            EndpointRef::from_instance_target(plan.preflight().instance_target_ref());
        let audience = plan.preflight().endpoint_audience().clone();
        let network = services.network().ok_or_else(|| {
            failure(
                "swallowtail.kimi.local_server.network_service_missing",
                "Kimi local-server requires a network-policy service",
            )
        })?;
        let (grant, interrupted) = wait_before_dispatch(
            network.authorize(scope.clone(), endpoint_ref.clone(), audience.clone()),
            agreement,
            cancellation,
            &services,
        )
        .await?;
        if interrupted {
            return Ok(outcome(
                agreement,
                ProviderSessionManagementEffect::failed_before_effect(action),
            ));
        }
        let grant = grant?;
        if grant.scope() != &scope
            || grant.endpoint() != &endpoint_ref
            || grant.audience() != &audience
        {
            return Err(failure(
                "swallowtail.kimi.local_server.network_grant_mismatch",
                "Kimi local-server network grant does not match its immutable plan",
            ));
        }

        let credential_service = services.credential().ok_or_else(|| {
            failure(
                "swallowtail.kimi.local_server.credential_service_missing",
                "Kimi local-server requires a credential service",
            )
        })?;
        let reference = plan
            .preflight()
            .credential_reference()
            .expect("validated credential reference")
            .clone();
        let (lease, interrupted) = wait_before_dispatch(
            credential_service.acquire(scope.clone(), reference.clone(), audience.clone()),
            agreement,
            cancellation,
            &services,
        )
        .await?;
        let lease = lease?;
        if interrupted {
            let cleanup = credential_service.release(lease).await;
            return Ok(with_cleanup(
                outcome(
                    agreement,
                    ProviderSessionManagementEffect::failed_before_effect(action),
                ),
                cleanup,
            ));
        }
        let bearer = match secret_bytes(&lease, &scope, &reference, &audience) {
            Ok(bearer) => bearer,
            Err(error) => {
                let cleanup = credential_service.release(lease).await;
                return if matches!(
                    cleanup,
                    CleanupOutcome::Clean | CleanupOutcome::NotApplicable
                ) {
                    Err(error)
                } else {
                    Err(cleanup_failure())
                };
            }
        };

        let result = if cancelled_or_expired(agreement, cancellation, &services)? {
            outcome(
                agreement,
                ProviderSessionManagementEffect::failed_before_effect(action),
            )
        } else {
            let provider_session_id = agreement
                .binding()
                .provider_session_ref()
                .as_provider_value();
            let path = action_path(provider_session_id, action)?;
            let transport_cancelled = Arc::new(AtomicBool::new(false));
            let (response, interrupted) = wait_after_dispatch(
                self.transport.request(
                    scope,
                    grant.authorized().as_driver_value().to_owned(),
                    Request::post(path),
                    Some(bearer),
                    &services,
                    Arc::clone(&transport_cancelled),
                ),
                agreement,
                cancellation,
                &services,
                transport_cancelled,
            )
            .await?;
            if interrupted {
                outcome(
                    agreement,
                    ProviderSessionManagementEffect::unconfirmed_after_effect(action),
                )
            } else {
                classify_response(agreement, response, provider_session_id)
            }
        };
        let cleanup = credential_service.release(lease).await;
        Ok(with_cleanup(result, cleanup))
    }
}

fn validate_plan(plan: &ProviderSessionManagementPlan) -> Result<(), RuntimeFailure> {
    let preflight = plan.preflight();
    if preflight.driver_identity().id().as_str() != DRIVER_ID {
        return Err(failure(
            "swallowtail.kimi.local_server.plan_driver_mismatch",
            "Kimi local-server plan is bound to a different driver",
        ));
    }
    let required = CredentialMechanism::ProviderSpecific(
        ExtensionNamespace::new("kimi-code/local-server-bearer")
            .expect("static credential namespace is valid"),
    );
    if preflight.credential_mechanism() != &required || preflight.credential_reference().is_none() {
        return Err(failure(
            "swallowtail.kimi.local_server.credential_profile_rejected",
            "Kimi local-server requires its opaque local bearer credential",
        ));
    }
    if plan.agreement().affected_scope() != ProviderSessionAffectedScope::TargetOnly {
        return Err(failure(
            "swallowtail.kimi.local_server.affected_scope_mismatch",
            "Kimi local-server lifecycle actions affect only the selected session",
        ));
    }
    Ok(())
}

fn operation_scope(request_id: &RequestId) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!(
        "kimi-local-server-management-{}",
        request_id.as_str()
    ))
    .map_err(|_| {
        failure(
            "swallowtail.kimi.local_server.scope_invalid",
            "Kimi local-server operation scope is invalid",
        )
    })
}

fn secret_bytes(
    lease: &CredentialLease,
    scope: &ScopeId,
    reference: &swallowtail_runtime::CredentialRef,
    audience: &swallowtail_core::EndpointAudience,
) -> Result<Vec<u8>, RuntimeFailure> {
    match lease {
        CredentialLease::Secret(secret)
            if secret.scope() == scope
                && secret.reference() == reference
                && secret.audience() == audience =>
        {
            Ok(secret.expose_secret().to_vec())
        }
        CredentialLease::Secret(_) | CredentialLease::Delegated(_) => Err(failure(
            "swallowtail.kimi.local_server.credential_lease_rejected",
            "Kimi local-server requires a matching secret bearer lease",
        )),
    }
}

fn cleanup_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.cleanup_failed",
        "Kimi local-server cleanup could not be joined",
    )
}
