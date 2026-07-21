use crate::failure::{failure, unsupported};
use crate::managed::{IdleReason, ManagedEvent, ManagedEventKind, Request};
use crate::managed_transport::ManagedCurlTransport;
use crate::managed_transport::{ManagedResponse, ManagedStreamItem, ManagedSubscription};
use futures_core::Stream;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::future::{Future, poll_fn};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CancellationScope, Capability,
    CapabilityConstraint, CredentialMechanism, DriverDescriptor, DriverRole, ExecutionLayer,
    ExternalNetworkPolicy, ExternalSearchPolicy, HostServiceKind, IntegrationFamilyId,
    OperationShape, OwnedRemoteResourceKind, PreflightPlan, ProviderAgentBinding,
    ProviderRequestRef, RunRef, SafeDiagnostic, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxCallbackStream, BoxEventStream, BoxFuture, CallbackAbandonment, CallbackExchange,
    CallbackFailureKind, CallbackId, CallbackPayload, CallbackRequest, CallbackResponder,
    CallbackResponse, CallbackResult, CancellationAcknowledgement, CancellationControl,
    CleanupOutcome, CredentialLease, Deadline, DeadlineObservation, EndpointRef, HostServices,
    JoinedTask, OperationContent, ProviderCancellationOutcome, ProviderExecutionPolicy,
    ProviderObservation, ProviderRecoveryPolicy, ProviderRetentionPolicy, RateLimitKind,
    RateLimitObservation, RemoteResourceDeletionOutcome, RequestId, RunHandle, RuntimeEvent,
    RuntimeEventKind, RuntimeFailure, RuntimeRunId, ScopeId, StreamReattachmentPolicy,
    StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TerminalStatus,
    runtime_event_channel, terminal_outcome_channel,
};

const DRIVER_ID: &str = "swallowtail.anthropic.managed-agent";
const EVENT_CAPACITY: usize = 64;

#[derive(Clone, Default)]
pub struct AnthropicManagedAgentDriver {
    transport: ManagedCurlTransport,
}

impl AnthropicManagedAgentDriver {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn validate_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.anthropic.managed.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::ApiKey
            || plan.credential_reference().is_none()
            || plan.endpoint_audience().as_str() != crate::managed::ENDPOINT_AUDIENCE
        {
            return Err(failure(
                "swallowtail.anthropic.managed.access_binding_rejected",
                "Anthropic Managed Agents requires the exact first-party API-key boundary",
            ));
        }
        Ok(())
    }
}

#[must_use]
pub fn anthropic_managed_agent_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("anthropic").expect("static family id is valid"),
        TransportFamilyId::new("https-sse-managed-agent").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::StructuredRun])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_required_host_services(
        DriverRole::StructuredRun,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ],
    )
}

fn require_services(services: &HostServices) -> Result<(), RuntimeFailure> {
    if services.task().is_none()
        || services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        Err(failure(
            "swallowtail.anthropic.managed.host_service_missing",
            "Anthropic Managed Agents requires task, blocking-work, time, network, and credential services",
        ))
    } else {
        Ok(())
    }
}

fn operation_scope(id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("anthropic-managed:run:{id}")).map_err(|_| {
        failure(
            "swallowtail.anthropic.managed.scope_invalid",
            "Anthropic Managed Agents operation scope was invalid",
        )
    })
}

fn requires(plan: &PreflightPlan, capability: Capability) -> bool {
    plan.requirements()
        .capabilities()
        .any(|requirement| requirement.capability() == capability)
}

include!("managed_driver/access.rs");
include!("managed_driver/validation.rs");
include!("managed_driver/callback.rs");
include!("managed_driver/handle.rs");
include!("managed_driver/provision.rs");
include!("managed_driver/observations.rs");
include!("managed_driver/run.rs");
include!("managed_driver/pump.rs");
