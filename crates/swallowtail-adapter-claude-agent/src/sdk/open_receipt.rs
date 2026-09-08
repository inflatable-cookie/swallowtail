//! Structured failed-open evidence for the prepared Claude Agent SDK route.
//!
//! One failed open is useful evidence without being a support claim: the
//! consumer reads the exact bounded rejection stage, the sidecar's subcode,
//! the provider-readiness truth, and the observed cleanup disposition from
//! typed fields — never by parsing the safe diagnostic message. Nothing here
//! carries paths, endpoints, bearer material, credentials, environment
//! values, provider content, or sidecar stderr. Every field is an
//! observation; absent evidence stays absent.

use crate::sdk::guardian::CleanupReport;
use crate::sdk::wire::ClaudeAgentSdkFailureCode;
use swallowtail_runtime::{CleanupOutcome, ProcessTreeCompletion, RuntimeFailure};

/// Where in the prepared-route open pipeline a failure was observed.
///
/// The stage is decided at the exact failure site, never inferred from a
/// message or a code after the fact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaudeAgentSdkOpenStage {
    /// Refused before the sidecar was contacted: request validation, host
    /// reservation, process spawn, or lease acquisition.
    Admission,
    /// The registered-tool bridge refused or failed its bounded admission
    /// before the sidecar was contacted.
    RegisteredAdmission,
    /// The sidecar rejected the open command. The receipt carries the exact
    /// bounded subcode when one was reported.
    SidecarRejected,
    /// The sidecar answered `open`, but its evidence failed route validation.
    ReadinessValidation,
    /// The sidecar reached provider readiness; the registered courier or its
    /// bridge failed afterwards.
    RegisteredReadiness,
    /// The open exchange with the sidecar ended without a usable answer:
    /// disconnect, terminal record, or protocol violation.
    SidecarExchange,
    /// The caller's deadline ended the open.
    Deadline,
}

/// The bounded sidecar rejection subcode carried by one rejected open.
///
/// This is the sidecar's own fixed failure vocabulary for the open route —
/// the same labels the wire carries — never free-form detail.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaudeAgentSdkOpenSubcode {
    /// The sidecar already had an open session.
    AlreadyOpen,
    /// The open command or one of its parameters was malformed.
    InvalidCommand,
    /// A required provisioned environment input was missing.
    MissingEnvironment,
    /// The Node runtime is below the sidecar's supported floor.
    NodeRuntimeUnsupported,
    /// The admitted tool set was rejected as malformed or unadmittable.
    ToolsInvalid,
    /// The requested permission mode was malformed.
    PermissionModeInvalid,
    /// The declared MCP server list was malformed.
    McpServersInvalid,
    /// An MCP tool name was not declared by any declared server.
    McpServerUndeclared,
    /// A required MCP server failed to connect.
    McpServerFailed,
    /// An MCP server requires authentication, which the route does not
    /// support.
    McpServerNeedsAuth,
    /// The MCP server status evidence was missing or malformed.
    McpStatusInvalid,
    /// The selected-skill bundle was malformed.
    SelectedSkillInvalid,
    /// The selected-skill bundle digest did not match its declared identity.
    SelectedSkillDigestMismatch,
    /// The selected-skill bundle exceeded a declared bound.
    SelectedSkillLimitExceeded,
    /// A selected-skill required reference was invalid.
    SelectedSkillReferenceInvalid,
    /// The selected-skill payload was not text.
    SelectedSkillPayloadNotText,
    /// The provisioned SDK module could not be loaded.
    SdkUnavailable,
    /// The provisioned SDK module did not expose its expected entry point.
    SdkExportMissing,
    /// The loaded SDK version did not match the bound package axis.
    SdkVersionMismatch,
    /// The loaded SDK identity could not be verified.
    SdkIdentityUnverifiable,
    /// The native binary manifest was unavailable.
    NativeManifestUnavailable,
    /// The native binary version did not match the bound axis.
    NativeVersionMismatch,
    /// The SDK query object could not be constructed.
    ConstructionFailed,
    /// The SDK initialization exchange failed or timed out.
    InitializationFailed,
    /// The runtime capability list overflowed its bound.
    CapabilitiesOverflow,
    /// The runtime capability list was malformed.
    CapabilitiesInvalid,
    /// The sidecar could not observe an effective model.
    ModelMissing,
    /// The effective model was absent from the supported-model list.
    SupportedModelRejected,
    /// The requested effort could not be confirmed.
    EffortUnconfirmed,
    /// The account is not the verified first-party provenance the route
    /// requires.
    AccountNotFirstParty,
    /// The account evidence could not be read.
    AccountUnavailable,
    /// The sidecar's native child was not running after construction.
    NativeChildUnavailable,
    /// The resume message boundary was rejected.
    ResumeBoundaryInvalid,
    /// Resume was requested without session persistence.
    ResumePersistenceDisabled,
    /// The resumed init did not report the leased working directory.
    ResumeCwdMismatch,
    /// The resumed init did not report first-party provenance.
    ResumeAccountMismatch,
    /// The provider session could not be identified for resume.
    ResumeSessionUnknown,
    /// A fixed sidecar label outside the open vocabulary above. The text is
    /// one of the sidecar's own bounded failure labels, never provider
    /// content or arbitrary detail.
    Other(&'static str),
}

impl ClaudeAgentSdkOpenSubcode {
    /// Total mapping from the wire's closed failure vocabulary.
    pub(crate) const fn from_wire(code: ClaudeAgentSdkFailureCode) -> Self {
        use ClaudeAgentSdkFailureCode as Wire;
        match code {
            Wire::AlreadyOpen => Self::AlreadyOpen,
            Wire::InvalidCommand => Self::InvalidCommand,
            Wire::MissingEnvironment => Self::MissingEnvironment,
            Wire::NodeRuntimeUnsupported => Self::NodeRuntimeUnsupported,
            Wire::ToolsInvalid => Self::ToolsInvalid,
            Wire::PermissionModeInvalid => Self::PermissionModeInvalid,
            Wire::McpServersInvalid => Self::McpServersInvalid,
            Wire::McpServerUndeclared => Self::McpServerUndeclared,
            Wire::McpServerFailed => Self::McpServerFailed,
            Wire::McpServerNeedsAuth => Self::McpServerNeedsAuth,
            Wire::McpStatusInvalid => Self::McpStatusInvalid,
            Wire::SelectedSkillInvalid => Self::SelectedSkillInvalid,
            Wire::SelectedSkillDigestMismatch => Self::SelectedSkillDigestMismatch,
            Wire::SelectedSkillLimitExceeded => Self::SelectedSkillLimitExceeded,
            Wire::SelectedSkillReferenceInvalid => Self::SelectedSkillReferenceInvalid,
            Wire::SelectedSkillPayloadNotText => Self::SelectedSkillPayloadNotText,
            Wire::SdkUnavailable => Self::SdkUnavailable,
            Wire::SdkExportMissing => Self::SdkExportMissing,
            Wire::SdkVersionMismatch => Self::SdkVersionMismatch,
            Wire::SdkIdentityUnverifiable => Self::SdkIdentityUnverifiable,
            Wire::NativeManifestUnavailable => Self::NativeManifestUnavailable,
            Wire::NativeVersionMismatch => Self::NativeVersionMismatch,
            Wire::ConstructionFailed => Self::ConstructionFailed,
            Wire::InitializationFailed => Self::InitializationFailed,
            Wire::CapabilitiesOverflow => Self::CapabilitiesOverflow,
            Wire::CapabilitiesInvalid => Self::CapabilitiesInvalid,
            Wire::ModelMissing => Self::ModelMissing,
            Wire::SupportedModelRejected => Self::SupportedModelRejected,
            Wire::EffortUnconfirmed => Self::EffortUnconfirmed,
            Wire::AccountNotFirstParty => Self::AccountNotFirstParty,
            Wire::AccountUnavailable => Self::AccountUnavailable,
            Wire::NativeChildUnavailable => Self::NativeChildUnavailable,
            Wire::ResumeBoundaryInvalid => Self::ResumeBoundaryInvalid,
            Wire::ResumePersistenceDisabled => Self::ResumePersistenceDisabled,
            Wire::ResumeCwdMismatch => Self::ResumeCwdMismatch,
            Wire::ResumeAccountMismatch => Self::ResumeAccountMismatch,
            Wire::ResumeSessionUnknown => Self::ResumeSessionUnknown,
            other => Self::Other(other.as_str()),
        }
    }

    /// The bounded sidecar label this subcode carries.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AlreadyOpen => "already_open",
            Self::InvalidCommand => "invalid_command",
            Self::MissingEnvironment => "missing_environment",
            Self::NodeRuntimeUnsupported => "node_runtime_unsupported",
            Self::ToolsInvalid => "tools_invalid",
            Self::PermissionModeInvalid => "permission_mode_invalid",
            Self::McpServersInvalid => "mcp_servers_invalid",
            Self::McpServerUndeclared => "mcp_server_undeclared",
            Self::McpServerFailed => "mcp_server_failed",
            Self::McpServerNeedsAuth => "mcp_server_needs_auth",
            Self::McpStatusInvalid => "mcp_status_invalid",
            Self::SelectedSkillInvalid => "selected_skill_invalid",
            Self::SelectedSkillDigestMismatch => "selected_skill_digest_mismatch",
            Self::SelectedSkillLimitExceeded => "selected_skill_limit_exceeded",
            Self::SelectedSkillReferenceInvalid => "selected_skill_reference_invalid",
            Self::SelectedSkillPayloadNotText => "selected_skill_payload_not_text",
            Self::SdkUnavailable => "sdk_unavailable",
            Self::SdkExportMissing => "sdk_export_missing",
            Self::SdkVersionMismatch => "sdk_version_mismatch",
            Self::SdkIdentityUnverifiable => "sdk_identity_unverifiable",
            Self::NativeManifestUnavailable => "native_manifest_unavailable",
            Self::NativeVersionMismatch => "native_version_mismatch",
            Self::ConstructionFailed => "construction_failed",
            Self::InitializationFailed => "initialization_failed",
            Self::CapabilitiesOverflow => "capabilities_overflow",
            Self::CapabilitiesInvalid => "capabilities_invalid",
            Self::ModelMissing => "model_missing",
            Self::SupportedModelRejected => "supported_model_rejected",
            Self::EffortUnconfirmed => "effort_unconfirmed",
            Self::AccountNotFirstParty => "account_not_first_party",
            Self::AccountUnavailable => "account_unavailable",
            Self::NativeChildUnavailable => "native_child_unavailable",
            Self::ResumeBoundaryInvalid => "resume_boundary_invalid",
            Self::ResumePersistenceDisabled => "resume_persistence_disabled",
            Self::ResumeCwdMismatch => "resume_cwd_mismatch",
            Self::ResumeAccountMismatch => "resume_account_mismatch",
            Self::ResumeSessionUnknown => "resume_session_unknown",
            Self::Other(label) => label,
        }
    }
}

/// Whether the failed open owed cleanup, and whether that cleanup was
/// observed.
///
/// The three states are exhaustive and mutually exclusive; the receipt never
/// reports a stronger posture than the host actually observed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaudeAgentSdkOpenCleanupDisposition {
    /// Nothing had been acquired when the open failed — the failure happened
    /// before the guard owned any credential, resource, process, pump, or
    /// lease — so no cleanup was owed.
    NotAcquired,
    /// The ordered cleanup continuation completed inside the caller's bound,
    /// and the staged observations below carry what it saw.
    Confirmed,
    /// Termination was requested, but the continuation did not finish inside
    /// the caller's bound and the guard still owns it. Staged observations
    /// stay absent rather than being invented.
    Unconfirmed,
}

/// The observed cleanup disposition of one failed open.
///
/// Every staged field is what the ordered cleanup continuation actually
/// observed; a disposition of [`NotAcquired`](ClaudeAgentSdkOpenCleanupDisposition::NotAcquired)
/// or [`Unconfirmed`](ClaudeAgentSdkOpenCleanupDisposition::Unconfirmed)
/// leaves them absent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSdkFailedOpenCleanup {
    disposition: ClaudeAgentSdkOpenCleanupDisposition,
    resource: Option<CleanupOutcome>,
    credential: Option<CleanupOutcome>,
    survivor_posture: Option<ProcessTreeCompletion>,
    registered_lease: Option<CleanupOutcome>,
}

impl ClaudeAgentSdkFailedOpenCleanup {
    pub(crate) const fn not_acquired() -> Self {
        Self {
            disposition: ClaudeAgentSdkOpenCleanupDisposition::NotAcquired,
            resource: None,
            credential: None,
            survivor_posture: None,
            registered_lease: None,
        }
    }

    pub(crate) const fn unconfirmed() -> Self {
        Self {
            disposition: ClaudeAgentSdkOpenCleanupDisposition::Unconfirmed,
            resource: None,
            credential: None,
            survivor_posture: None,
            registered_lease: None,
        }
    }

    pub(crate) fn from_report(report: &CleanupReport) -> Self {
        Self {
            disposition: ClaudeAgentSdkOpenCleanupDisposition::Confirmed,
            resource: Some(report.resource.clone()),
            credential: Some(report.credential.clone()),
            // Root truth is only readable once the pump that records it was
            // joined; the same rule the session-close decision applies.
            survivor_posture: if report.pump_joined {
                report.root_exit
            } else {
                None
            },
            registered_lease: report.registered.clone(),
        }
    }

    /// The cleanup disposition: whether cleanup was owed at all, and whether
    /// the ordered continuation was observed to completion.
    #[must_use]
    pub const fn disposition(&self) -> ClaudeAgentSdkOpenCleanupDisposition {
        self.disposition
    }

    /// Working-resource lease release observation, present only when the
    /// ordered cleanup completed.
    #[must_use]
    pub const fn resource(&self) -> Option<&CleanupOutcome> {
        self.resource.as_ref()
    }

    /// Credential lease release observation, present only when the ordered
    /// cleanup completed.
    #[must_use]
    pub const fn credential(&self) -> Option<&CleanupOutcome> {
        self.credential.as_ref()
    }

    /// The owned process tree's survivor posture, observed after the protocol
    /// pump joined: `RootOnly` means only the root exit was seen and owned
    /// descendants (such as a provider-spawned registered courier) are
    /// unattested; `OwnedTreeEmpty` means the host observed no member
    /// remaining. `None` means the posture was not observed.
    #[must_use]
    pub const fn survivor_posture(&self) -> Option<ProcessTreeCompletion> {
        self.survivor_posture
    }

    /// The registered-tool bridge lease close observation, present only when
    /// the ordered cleanup completed and a registered lease had been opened.
    /// `Clean` means bridge admission was frozen, every outstanding courier
    /// call joined, and the listener and registry slot released; a `Failed`
    /// outcome names the bounded teardown truth.
    #[must_use]
    pub fn registered_lease(&self) -> Option<&CleanupOutcome> {
        self.registered_lease.as_ref()
    }
}

/// Observation-derived structured evidence for one failed prepared-route
/// open.
///
/// The receipt is additive: the returned [`RuntimeFailure`](swallowtail_runtime::RuntimeFailure)
/// keeps its exact stable route code and message. The `route_code` field is
/// the underlying open failure's route code; it differs from the returned
/// diagnostic's code only when the deadline or an unconfirmed cleanup
/// replaced the reported error, which the cleanup disposition records.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSdkFailedOpenReceipt {
    route_code: String,
    stage: ClaudeAgentSdkOpenStage,
    sidecar_code: Option<ClaudeAgentSdkOpenSubcode>,
    provider_readiness_reached: bool,
    cleanup: ClaudeAgentSdkFailedOpenCleanup,
}

impl ClaudeAgentSdkFailedOpenReceipt {
    /// The stable route code of the underlying open failure.
    #[must_use]
    pub fn route_code(&self) -> &str {
        &self.route_code
    }

    /// Where in the open pipeline the failure was observed.
    #[must_use]
    pub const fn stage(&self) -> ClaudeAgentSdkOpenStage {
        self.stage
    }

    /// The exact bounded sidecar subcode, present only for a sidecar
    /// rejection.
    #[must_use]
    pub const fn sidecar_code(&self) -> Option<ClaudeAgentSdkOpenSubcode> {
        self.sidecar_code
    }

    /// Whether the sidecar reached provider readiness before the failure.
    #[must_use]
    pub const fn provider_readiness_reached(&self) -> bool {
        self.provider_readiness_reached
    }

    /// The observed cleanup disposition.
    #[must_use]
    pub const fn cleanup(&self) -> &ClaudeAgentSdkFailedOpenCleanup {
        &self.cleanup
    }
}

/// One failed prepared-route open: the exact failure the ordinary open
/// surface returns, plus its structured receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSdkOpenRejection {
    failure: RuntimeFailure,
    receipt: ClaudeAgentSdkFailedOpenReceipt,
}

impl ClaudeAgentSdkOpenRejection {
    pub(crate) fn new(failure: RuntimeFailure, receipt: ClaudeAgentSdkFailedOpenReceipt) -> Self {
        Self { failure, receipt }
    }

    /// The failure, byte-identical to what the ordinary open surface returns.
    #[must_use]
    pub fn failure(&self) -> &RuntimeFailure {
        &self.failure
    }

    /// Consumes the rejection into its failure.
    #[must_use]
    pub fn into_failure(self) -> RuntimeFailure {
        self.failure
    }

    /// The structured failed-open receipt.
    #[must_use]
    pub const fn receipt(&self) -> &ClaudeAgentSdkFailedOpenReceipt {
        &self.receipt
    }
}

/// A classified open failure, produced at the exact failure site.
///
/// `route_code` is pinned from the failure's own diagnostic at construction,
/// so a deadline or unconfirmed-cleanup replacement of the returned error
/// never overwrites the documented underlying route code.
pub(crate) struct OpenFailure {
    route_code: String,
    error: RuntimeFailure,
    stage: ClaudeAgentSdkOpenStage,
    sidecar_code: Option<ClaudeAgentSdkFailureCode>,
    provider_readiness_reached: bool,
}

impl OpenFailure {
    fn new(
        error: RuntimeFailure,
        stage: ClaudeAgentSdkOpenStage,
        sidecar_code: Option<ClaudeAgentSdkFailureCode>,
    ) -> Self {
        Self {
            route_code: error.diagnostic().code().to_owned(),
            error,
            stage,
            sidecar_code,
            provider_readiness_reached: false,
        }
    }

    pub(crate) fn admission(error: RuntimeFailure) -> Self {
        Self::new(error, ClaudeAgentSdkOpenStage::Admission, None)
    }

    pub(crate) fn registered_admission(error: RuntimeFailure) -> Self {
        Self::new(error, ClaudeAgentSdkOpenStage::RegisteredAdmission, None)
    }

    pub(crate) fn sidecar_exchange(error: RuntimeFailure) -> Self {
        Self::new(error, ClaudeAgentSdkOpenStage::SidecarExchange, None)
    }

    pub(crate) fn readiness_validation(error: RuntimeFailure) -> Self {
        Self::new(error, ClaudeAgentSdkOpenStage::ReadinessValidation, None)
    }

    pub(crate) fn deadline(provider_readiness_reached: bool) -> Self {
        Self::new(
            crate::sdk::driver::open_deadline_elapsed(),
            ClaudeAgentSdkOpenStage::Deadline,
            None,
        )
        .with_readiness(provider_readiness_reached)
    }

    /// The open ended on the deadline and its cleanup could not be confirmed
    /// inside the same bound.
    pub(crate) fn deadline_unconfirmed_cleanup(provider_readiness_reached: bool) -> Self {
        Self::new(
            crate::sdk::driver::open_cleanup_unconfirmed(),
            ClaudeAgentSdkOpenStage::Deadline,
            None,
        )
        .with_readiness(provider_readiness_reached)
    }

    pub(crate) fn sidecar_rejection(
        error: RuntimeFailure,
        sidecar_code: ClaudeAgentSdkFailureCode,
    ) -> Self {
        Self::new(
            error,
            ClaudeAgentSdkOpenStage::SidecarRejected,
            Some(sidecar_code),
        )
    }

    /// A failure observed after the sidecar reached provider readiness.
    pub(crate) fn after_readiness(error: RuntimeFailure) -> Self {
        Self::new(error, ClaudeAgentSdkOpenStage::RegisteredReadiness, None).with_readiness(true)
    }

    fn with_readiness(mut self, provider_readiness_reached: bool) -> Self {
        self.provider_readiness_reached = provider_readiness_reached;
        self
    }

    /// The same failure facts under a replacement error (deadline or
    /// unconfirmed cleanup). The underlying route code and every staged
    /// observation survive the replacement untouched.
    pub(crate) fn with_replaced_error(&self, error: RuntimeFailure) -> Self {
        Self {
            route_code: self.route_code.clone(),
            error,
            stage: self.stage,
            sidecar_code: self.sidecar_code,
            provider_readiness_reached: self.provider_readiness_reached,
        }
    }

    fn route_code(&self) -> &str {
        &self.route_code
    }
}

/// Builds one open rejection from its classified failure and the observed
/// cleanup disposition.
pub(crate) fn open_rejection(
    failure: OpenFailure,
    cleanup: ClaudeAgentSdkFailedOpenCleanup,
) -> ClaudeAgentSdkOpenRejection {
    let receipt = ClaudeAgentSdkFailedOpenReceipt {
        route_code: failure.route_code().to_owned(),
        stage: failure.stage,
        sidecar_code: failure
            .sidecar_code
            .map(ClaudeAgentSdkOpenSubcode::from_wire),
        provider_readiness_reached: failure.provider_readiness_reached,
        cleanup,
    };
    ClaudeAgentSdkOpenRejection::new(failure.error, receipt)
}

impl From<OpenFailure> for ClaudeAgentSdkOpenRejection {
    fn from(failure: OpenFailure) -> Self {
        // The `?` conversion is only reachable before the guard arms, so the
        // open has acquired nothing and owes no cleanup.
        open_rejection(failure, ClaudeAgentSdkFailedOpenCleanup::not_acquired())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The deadline and unconfirmed-cleanup replacements rewrite only the
    /// returned error. The receipt's route code, stage, and subcode stay
    /// pinned to the underlying failure at its failure site.
    #[test]
    fn a_replaced_error_keeps_the_underlying_route_code_stage_and_subcode() {
        let failure = OpenFailure::sidecar_rejection(
            crate::sdk::failure::command_rejected(
                "swallowtail.claude-agent.sdk.open_rejected",
                "Claude Agent SDK sidecar rejected its restrictive open",
                ClaudeAgentSdkFailureCode::ConstructionFailed,
            ),
            ClaudeAgentSdkFailureCode::ConstructionFailed,
        );
        assert_eq!(
            failure.route_code(),
            "swallowtail.claude-agent.sdk.open_rejected"
        );

        let unconfirmed =
            failure.with_replaced_error(crate::sdk::driver::open_cleanup_unconfirmed());
        assert_eq!(
            unconfirmed.error.diagnostic().code(),
            "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed"
        );
        assert_eq!(
            unconfirmed.route_code(),
            "swallowtail.claude-agent.sdk.open_rejected",
            "the replacement must not overwrite the underlying route code"
        );
        assert_eq!(unconfirmed.stage, ClaudeAgentSdkOpenStage::SidecarRejected);
        assert_eq!(
            unconfirmed.sidecar_code,
            Some(ClaudeAgentSdkFailureCode::ConstructionFailed)
        );

        let rejection = open_rejection(unconfirmed, ClaudeAgentSdkFailedOpenCleanup::unconfirmed());
        assert_eq!(
            rejection.failure().diagnostic().code(),
            "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed"
        );
        assert_eq!(
            rejection.receipt().route_code(),
            "swallowtail.claude-agent.sdk.open_rejected"
        );
        assert_eq!(
            rejection.receipt().sidecar_code(),
            Some(ClaudeAgentSdkOpenSubcode::ConstructionFailed)
        );
        assert_eq!(
            rejection.receipt().cleanup().disposition(),
            ClaudeAgentSdkOpenCleanupDisposition::Unconfirmed
        );
    }

    /// A deadline that replaces a sidecar rejection keeps the rejection's
    /// route code and subcode in the receipt too.
    #[test]
    fn a_deadline_replacement_keeps_the_underlying_rejection() {
        let failure = OpenFailure::sidecar_rejection(
            crate::sdk::failure::command_rejected(
                "swallowtail.claude-agent.sdk.open_rejected",
                "Claude Agent SDK sidecar rejected its restrictive open",
                ClaudeAgentSdkFailureCode::InitializationFailed,
            ),
            ClaudeAgentSdkFailureCode::InitializationFailed,
        )
        .with_replaced_error(crate::sdk::driver::open_deadline_elapsed());
        let rejection = open_rejection(failure, ClaudeAgentSdkFailedOpenCleanup::not_acquired());
        assert_eq!(
            rejection.failure().diagnostic().code(),
            "swallowtail.claude-agent.sdk.open_deadline_elapsed"
        );
        assert_eq!(
            rejection.receipt().route_code(),
            "swallowtail.claude-agent.sdk.open_rejected"
        );
        assert_eq!(
            rejection.receipt().sidecar_code(),
            Some(ClaudeAgentSdkOpenSubcode::InitializationFailed)
        );
    }
}
