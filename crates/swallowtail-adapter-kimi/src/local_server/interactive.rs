mod access;
mod callbacks;
mod driver;
mod prepared;
mod session;
mod websocket;

pub(in crate::local_server) use prepared::{access_policy, validate_revision_options};
pub(in crate::local_server) use session::{KimiInteractiveSession, TurnCancellation};

use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{
    BoxFuture, InteractiveSessionHandle, OperationContent, PreparationFailure, RequestId,
    SessionOptions, WorkingResourceRef,
};

pub type KimiLocalServerPreparedSessionFuture = BoxFuture<
    'static,
    Result<Box<dyn InteractiveSessionHandle>, swallowtail_runtime::RuntimeFailure>,
>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KimiLocalServerPermissionMode {
    Manual,
    Auto,
    Yolo,
}

impl KimiLocalServerPermissionMode {
    const fn as_wire_value(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Auto => "auto",
            Self::Yolo => "yolo",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerSessionConfiguration {
    permission_mode: KimiLocalServerPermissionMode,
    profile: Option<String>,
    disabled_tools: Vec<String>,
}

impl KimiLocalServerSessionConfiguration {
    #[must_use]
    pub const fn new(permission_mode: KimiLocalServerPermissionMode) -> Self {
        Self {
            permission_mode,
            profile: None,
            disabled_tools: Vec::new(),
        }
    }

    pub fn with_profile(mut self, profile: impl Into<String>) -> Result<Self, PreparationFailure> {
        self.profile = Some(required_provider_text("profile", profile)?);
        Ok(self)
    }

    pub fn with_disabled_tools(
        mut self,
        tools: impl IntoIterator<Item = String>,
    ) -> Result<Self, PreparationFailure> {
        let tools: Vec<_> = tools.into_iter().collect();
        if tools.len() > 128
            || tools
                .iter()
                .any(|tool| tool.trim().is_empty() || tool.len() > 256)
        {
            return Err(preparation_error(
                "swallowtail.kimi.local_server.preparation.tool_policy_invalid",
                "Kimi local-server disabled-tool policy is invalid",
            ));
        }
        self.disabled_tools = tools;
        Ok(self)
    }

    #[must_use]
    pub const fn permission_mode(&self) -> KimiLocalServerPermissionMode {
        self.permission_mode
    }

    #[must_use]
    pub fn profile(&self) -> Option<&str> {
        self.profile.as_deref()
    }

    pub fn disabled_tools(&self) -> impl ExactSizeIterator<Item = &str> {
        self.disabled_tools.iter().map(String::as_str)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerSessionInput {
    request_id: RequestId,
    model: crate::KimiModelSelection,
    working_resource: WorkingResourceRef,
    deadline: Option<swallowtail_runtime::Deadline>,
    options: SessionOptions,
    configuration: KimiLocalServerSessionConfiguration,
    allow_unverified_newer: bool,
}

impl KimiLocalServerSessionInput {
    #[must_use]
    pub fn new(
        request_id: RequestId,
        model: crate::KimiModelSelection,
        working_resource: WorkingResourceRef,
        configuration: KimiLocalServerSessionConfiguration,
    ) -> Self {
        Self {
            request_id,
            model,
            working_resource,
            deadline: None,
            options: SessionOptions::default(),
            configuration,
            allow_unverified_newer: false,
        }
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: swallowtail_runtime::Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub fn with_reasoning(mut self, reasoning: ReasoningMode) -> Self {
        self.options = self.options.with_reasoning_mode(reasoning);
        self
    }

    #[must_use]
    pub const fn allow_unverified_newer(mut self) -> Self {
        self.allow_unverified_newer = true;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerPreparedSession {
    evidence: swallowtail_runtime::PreparedOperationEvidence,
    request: swallowtail_runtime::OpenSessionRequest,
    configuration: KimiLocalServerSessionConfiguration,
    management_instance: swallowtail_core::ConfiguredInstance,
}

impl KimiLocalServerPreparedSession {
    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &swallowtail_runtime::OpenSessionRequest {
        &self.request
    }

    #[must_use]
    pub const fn configuration(&self) -> &KimiLocalServerSessionConfiguration {
        &self.configuration
    }

    #[must_use]
    pub fn low_level_driver(&self) -> super::KimiLocalServerDriver {
        super::KimiLocalServerDriver::with_session_configuration(self.configuration.clone())
    }

    pub fn open_session(
        &self,
        services: swallowtail_runtime::HostServices,
    ) -> KimiLocalServerPreparedSessionFuture {
        prepared::open(self, services)
    }

    pub fn resume_session(
        &self,
        request_id: RequestId,
        binding: swallowtail_runtime::SessionResumeBinding,
        services: swallowtail_runtime::HostServices,
    ) -> Result<KimiLocalServerPreparedSessionFuture, PreparationFailure> {
        prepared::resume(self, request_id, binding, services)
    }
}

fn required_provider_text(
    field: &'static str,
    value: impl Into<String>,
) -> Result<String, PreparationFailure> {
    let value = value.into();
    if value.trim().is_empty() || value.len() > 256 {
        Err(preparation_error(
            "swallowtail.kimi.local_server.preparation.provider_option_invalid",
            field,
        ))
    } else {
        Ok(value)
    }
}

fn preparation_error(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        swallowtail_runtime::PreparationStage::Preflight,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}

fn reasoning_wire(options: &SessionOptions) -> Option<&str> {
    options.reasoning_mode().map(ReasoningMode::as_str)
}

fn content_json(content: &OperationContent) -> serde_json::Value {
    serde_json::json!([{"type":"text","text":content.as_str()}])
}
