use serde_json::Value;
use std::sync::Arc;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, ExtensionNamespace, PreflightPlan,
    ProviderApprovalPolicy, ProviderRequestHandling, ProviderRequestPolicy, ResourceAccess,
    ResourceRepresentation, SessionAccessPolicy,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, ResourceLease, RuntimeFailure, ScopeId, WorkingResourceRef,
    WorkingResourceService, validate_session_access_plan, validate_session_resource_lease,
};

const APPROVAL_EXTENSION: &str = "codex.app-server/provider-request/approval-v1";
const USER_INPUT_EXTENSION: &str = "codex.app-server/provider-request/user-input-v1";

#[must_use]
pub fn codex_approval_request_extension() -> ExtensionNamespace {
    ExtensionNamespace::new(APPROVAL_EXTENSION).expect("static extension namespace is valid")
}

#[must_use]
pub fn codex_user_input_request_extension() -> ExtensionNamespace {
    ExtensionNamespace::new(USER_INPUT_EXTENSION).expect("static extension namespace is valid")
}

#[must_use]
pub fn codex_provider_request_extensions() -> [ExtensionNamespace; 2] {
    [
        codex_approval_request_extension(),
        codex_user_input_request_extension(),
    ]
}

pub(crate) fn provider_request_namespace(method: &str) -> Option<ExtensionNamespace> {
    match method {
        "item/commandExecution/requestApproval"
        | "item/fileChange/requestApproval"
        | "item/permissions/requestApproval" => Some(codex_approval_request_extension()),
        "item/tool/requestUserInput" => Some(codex_user_input_request_extension()),
        _ => None,
    }
}

#[must_use]
pub fn codex_bounded_workspace_access_policy() -> SessionAccessPolicy {
    SessionAccessPolicy::bounded_workspace(codex_provider_request_extensions())
}

#[must_use]
pub fn codex_bounded_workspace_capability() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::WorkingResource,
        [
            CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
            CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
        ],
    )
}

pub(crate) struct CodexSessionAccess {
    working_resource: WorkingResourceRef,
    writable_root: Option<String>,
    provider_requests: ProviderRequestPolicy,
    resource_service: Option<Arc<dyn WorkingResourceService>>,
    lease: Option<ResourceLease>,
}

impl CodexSessionAccess {
    pub(crate) async fn prepare(
        plan: &PreflightPlan,
        policy: &SessionAccessPolicy,
        working_resource: &WorkingResourceRef,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, RuntimeFailure> {
        validate_session_access_plan(plan, policy)?;
        if policy.approval_policy() != ProviderApprovalPolicy::Never {
            return Err(unsupported("provider approval policy"));
        }
        if policy.external_network() != swallowtail_core::ExternalNetworkPolicy::Denied
            || policy.external_search() != swallowtail_core::ExternalSearchPolicy::Disabled
        {
            return Err(unsupported("provider network or external search"));
        }
        validate_provider_requests(policy.provider_requests())?;

        if policy.resource_access() == ResourceAccess::Read {
            return Ok(Self {
                working_resource: working_resource.clone(),
                writable_root: None,
                provider_requests: policy.provider_requests().clone(),
                resource_service: None,
                lease: None,
            });
        }

        services.require_execution_host(plan.execution_host_id())?;
        let service = services.working_resource().cloned().ok_or_else(|| {
            failure(
                "swallowtail.codex.app_server.working_resource_service_missing",
                "Codex writable session requires a preflight-bound working-resource service",
            )
        })?;
        let lease = service
            .resolve(
                scope,
                working_resource.clone(),
                ResourceAccess::ReadWrite,
                ResourceRepresentation::Filesystem,
            )
            .await?;
        if let Err(error) = validate_session_resource_lease(policy, working_resource, &lease) {
            let _ = service.release(lease).await;
            return Err(error);
        }
        let writable_root = lease
            .filesystem()
            .expect("validated filesystem lease exposes a root")
            .as_driver_value()
            .to_owned();
        Ok(Self {
            working_resource: lease.reference().clone(),
            writable_root: Some(writable_root),
            provider_requests: policy.provider_requests().clone(),
            resource_service: Some(service),
            lease: Some(lease),
        })
    }

    pub(crate) fn apply_thread(&self, params: &mut Value) {
        let object = params
            .as_object_mut()
            .expect("static thread parameters are an object");
        object.insert(
            "approvalPolicy".to_owned(),
            Value::String("never".to_owned()),
        );
        match &self.writable_root {
            None => {
                object.insert("sandbox".to_owned(), Value::String("read-only".to_owned()));
            }
            Some(root) => {
                object.insert(
                    "sandbox".to_owned(),
                    Value::String("workspace-write".to_owned()),
                );
                object.insert("cwd".to_owned(), Value::String(root.clone()));
                object.insert(
                    "runtimeWorkspaceRoots".to_owned(),
                    Value::Array(vec![Value::String(root.clone())]),
                );
            }
        }
    }

    pub(crate) fn turn_sandbox_policy(&self) -> Option<Value> {
        self.writable_root.as_ref().map(|root| {
            serde_json::json!({
                "type": "workspaceWrite",
                "writableRoots": [root],
                "networkAccess": false,
                "excludeSlashTmp": true,
                "excludeTmpdirEnvVar": true
            })
        })
    }

    pub(crate) fn requires_experimental_api(&self) -> bool {
        self.provider_requests
            .handling_for(&codex_user_input_request_extension())
            == ProviderRequestHandling::ObserveAndStop
    }

    pub(crate) const fn working_resource(&self) -> &WorkingResourceRef {
        &self.working_resource
    }

    pub(crate) fn provider_requests(&self) -> ProviderRequestPolicy {
        self.provider_requests.clone()
    }

    pub(crate) async fn release(mut self) -> CleanupOutcome {
        match (self.resource_service.take(), self.lease.take()) {
            (Some(service), Some(lease)) => service.release(lease).await,
            _ => CleanupOutcome::NotApplicable,
        }
    }
}

fn validate_provider_requests(policy: &ProviderRequestPolicy) -> Result<(), RuntimeFailure> {
    let supported = codex_provider_request_extensions();
    for namespace in policy.observed_extensions() {
        if !supported.iter().any(|value| value == namespace) {
            return Err(unsupported("provider request extension"));
        }
    }
    Ok(())
}

fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.session_access_unsupported",
        format!("Codex app-server does not support the requested {feature}"),
    )
}

fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(code, message))
}
