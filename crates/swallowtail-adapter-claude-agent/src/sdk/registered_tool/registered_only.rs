//! Registered-only Claude Agent SDK session: zero native tools, explicit access.
//!
//! Ordinary native profiles still require at least one admitted SDK tool.
//! This binding is the only public constructor that may admit an empty native
//! set, and it does so only together with a non-empty qualified registered
//! selection and an explicit `Read` or `ReadWrite` working-resource lease.

use super::ClaudeAgentSdkRegisteredToolBinding;
use crate::sdk::profile::{ClaudeAgentSdkPermissionMode, ClaudeAgentSdkSessionProfile};
use swallowtail_core::{Diagnostic, ResourceAccess};
use swallowtail_host_local::LocalHostServices;
use swallowtail_runtime::{PreparationFailure, PreparationStage, RegisteredToolPreparation};

/// Zero-native session bound to one qualified registered-tool selection.
///
/// Native admission stays empty. Working-resource access is the consumer's
/// explicit registered-route choice, never inferred from MCP presence, a
/// carrier spelling, or [`swallowtail_runtime::RegisteredToolEffectPosture`].
#[derive(Clone)]
pub struct ClaudeAgentSdkRegisteredOnlyBinding {
    profile: ClaudeAgentSdkSessionProfile,
    registered: ClaudeAgentSdkRegisteredToolBinding,
}

impl ClaudeAgentSdkRegisteredOnlyBinding {
    /// Qualifies one registered selection and binds it to a zero-native
    /// session with explicit working-resource access.
    ///
    /// The selection must already be non-empty and qualify for the
    /// mediated-stdio courier. An empty native profile without this binding
    /// remains an early typed failure on `new` / `from_names` and at prepare.
    pub fn new(
        preparation: RegisteredToolPreparation,
        resource_access: ResourceAccess,
        permission_mode: ClaudeAgentSdkPermissionMode,
    ) -> Result<Self, PreparationFailure> {
        let registered =
            ClaudeAgentSdkRegisteredToolBinding::qualify(preparation).map_err(|error| {
                PreparationFailure::new(
                    PreparationStage::Preflight,
                    Diagnostic::new(error.diagnostic().clone()),
                )
            })?;
        Ok(Self {
            profile: ClaudeAgentSdkSessionProfile::registered_only(
                resource_access,
                permission_mode,
            ),
            registered,
        })
    }

    /// Binds the host composition that resolves the approved courier path.
    #[must_use]
    pub fn with_host(mut self, host: LocalHostServices) -> Self {
        self.registered = self.registered.with_host(host);
        self
    }

    /// Replaces the opening permission mode on the Copy profile.
    #[must_use]
    pub fn with_permission_mode(mut self, permission_mode: ClaudeAgentSdkPermissionMode) -> Self {
        self.profile = self.profile.with_permission_mode(permission_mode);
        self
    }

    /// Selects one admitted effort level for the opening SDK options.
    #[must_use]
    pub fn with_effort(mut self, effort: crate::sdk::profile::ClaudeAgentSdkEffort) -> Self {
        self.profile = self.profile.with_effort(effort);
        self
    }

    /// Enables or disables provider-owned session persistence.
    #[must_use]
    pub fn with_persist_session(mut self, persist_session: bool) -> Self {
        self.profile = self.profile.with_persist_session(persist_session);
        self
    }

    /// Returns the Copy profile this binding carries: zero native tools and
    /// the explicit working-resource lease.
    #[must_use]
    pub const fn session_profile(&self) -> ClaudeAgentSdkSessionProfile {
        self.profile
    }

    /// Returns the qualified registered-tool selection.
    #[must_use]
    pub const fn registered_tools(&self) -> &ClaudeAgentSdkRegisteredToolBinding {
        &self.registered
    }

    /// Returns the explicit working-resource access this session requires.
    #[must_use]
    pub const fn resource_access(&self) -> ResourceAccess {
        self.profile.resource_access()
    }

    /// Consumes the binding into the qualified registered-tool selection.
    #[must_use]
    pub fn into_registered(self) -> ClaudeAgentSdkRegisteredToolBinding {
        self.registered
    }
}
