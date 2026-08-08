#![deny(missing_docs)]

use crate::{
    DiscoveryDriver, InteractiveSessionDriver, ModelCatalogDriver,
    ProviderRecoveredResourceCleanupDriver, ProviderRunReconciliationDriver,
    ProviderSessionCatalogueDriver, ProviderSessionHistoryDriver, ProviderSessionImportDriver,
    ProviderSessionManagementDriver, ProviderSessionReconciliationDriver,
    RealtimeMediaSessionDriver, ServingInstanceDriver, StructuredRunDriver,
};
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use swallowtail_core::{DriverDescriptor, DriverRole, SafeDiagnostic};

/// Failure produced when a registration supplies a role absent from its descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistrationFailure {
    role: DriverRole,
    diagnostic: SafeDiagnostic,
}

impl RegistrationFailure {
    fn undeclared(role: DriverRole) -> Self {
        Self {
            role,
            diagnostic: SafeDiagnostic::new(
                "swallowtail.registration_role_undeclared",
                format!("Driver descriptor does not declare {role:?}"),
            ),
        }
    }

    #[must_use]
    /// Returns the undeclared role that was rejected.
    pub const fn role(&self) -> DriverRole {
        self.role
    }

    #[must_use]
    /// Returns the redacted diagnostic suitable for consumer display or logs.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for RegistrationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for RegistrationFailure {}

/// Runtime binding between one driver descriptor and its implemented roles.
///
/// Each `with_*` method verifies that the descriptor declared the role. The
/// registration does not select a provider, infer a fallback, or manufacture
/// capability support.
#[derive(Clone)]
pub struct DriverRegistration {
    descriptor: DriverDescriptor,
    discovery: Option<Arc<dyn DiscoveryDriver>>,
    model_catalog: Option<Arc<dyn ModelCatalogDriver>>,
    structured_run: Option<Arc<dyn StructuredRunDriver>>,
    interactive_session: Option<Arc<dyn InteractiveSessionDriver>>,
    realtime_media_session: Option<Arc<dyn RealtimeMediaSessionDriver>>,
    serving_instance: Option<Arc<dyn ServingInstanceDriver>>,
    provider_session_management: Option<Arc<dyn ProviderSessionManagementDriver>>,
    provider_session_catalogue: Option<Arc<dyn ProviderSessionCatalogueDriver>>,
    provider_session_import: Option<Arc<dyn ProviderSessionImportDriver>>,
    provider_session_reconciliation: Option<Arc<dyn ProviderSessionReconciliationDriver>>,
    provider_session_history: Option<Arc<dyn ProviderSessionHistoryDriver>>,
    provider_run_reconciliation: Option<Arc<dyn ProviderRunReconciliationDriver>>,
    provider_recovered_resource_cleanup: Option<Arc<dyn ProviderRecoveredResourceCleanupDriver>>,
}

impl DriverRegistration {
    /// Creates an empty role binding for the supplied descriptor.
    #[must_use]
    pub const fn new(descriptor: DriverDescriptor) -> Self {
        Self {
            descriptor,
            discovery: None,
            model_catalog: None,
            structured_run: None,
            interactive_session: None,
            realtime_media_session: None,
            serving_instance: None,
            provider_session_management: None,
            provider_session_catalogue: None,
            provider_session_import: None,
            provider_session_reconciliation: None,
            provider_session_history: None,
            provider_run_reconciliation: None,
            provider_recovered_resource_cleanup: None,
        }
    }

    /// Registers the declared discovery role.
    pub fn with_discovery(
        mut self,
        role: Arc<dyn DiscoveryDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::Discovery)?;
        self.discovery = Some(role);
        Ok(self)
    }

    /// Registers the declared structured-run role.
    pub fn with_structured_run(
        mut self,
        role: Arc<dyn StructuredRunDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::StructuredRun)?;
        self.structured_run = Some(role);
        Ok(self)
    }

    /// Registers the declared model-catalogue role.
    pub fn with_model_catalog(
        mut self,
        role: Arc<dyn ModelCatalogDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::ModelCatalog)?;
        self.model_catalog = Some(role);
        Ok(self)
    }

    /// Registers the declared interactive-session role.
    pub fn with_interactive_session(
        mut self,
        role: Arc<dyn InteractiveSessionDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::InteractiveSession)?;
        self.interactive_session = Some(role);
        Ok(self)
    }

    /// Registers the declared serving-instance lifecycle role.
    pub fn with_serving_instance(
        mut self,
        role: Arc<dyn ServingInstanceDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::ServingInstanceLifecycle)?;
        self.serving_instance = Some(role);
        Ok(self)
    }

    /// Registers the declared realtime-media session role.
    pub fn with_realtime_media_session(
        mut self,
        role: Arc<dyn RealtimeMediaSessionDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::RealtimeMediaSession)?;
        self.realtime_media_session = Some(role);
        Ok(self)
    }

    /// Registers the declared inactive provider-session management role.
    pub fn with_provider_session_management(
        mut self,
        role: Arc<dyn ProviderSessionManagementDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::ProviderSessionManagement)?;
        self.provider_session_management = Some(role);
        Ok(self)
    }

    /// Registers the declared provider-session catalogue role.
    pub fn with_provider_session_catalogue(
        mut self,
        role: Arc<dyn ProviderSessionCatalogueDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::ProviderSessionCatalogue)?;
        self.provider_session_catalogue = Some(role);
        Ok(self)
    }

    /// Registers the declared provider-session import role.
    pub fn with_provider_session_import(
        mut self,
        role: Arc<dyn ProviderSessionImportDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::ProviderSessionImport)?;
        self.provider_session_import = Some(role);
        Ok(self)
    }

    /// Registers the declared provider-session reconciliation role.
    pub fn with_provider_session_reconciliation(
        mut self,
        role: Arc<dyn ProviderSessionReconciliationDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::ProviderSessionReconciliation)?;
        self.provider_session_reconciliation = Some(role);
        Ok(self)
    }

    /// Registers the declared provider-session history role.
    pub fn with_provider_session_history(
        mut self,
        role: Arc<dyn ProviderSessionHistoryDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::ProviderSessionHistory)?;
        self.provider_session_history = Some(role);
        Ok(self)
    }

    /// Registers the declared provider-run reconciliation role.
    pub fn with_provider_run_reconciliation(
        mut self,
        role: Arc<dyn ProviderRunReconciliationDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::ProviderRunReconciliation)?;
        self.provider_run_reconciliation = Some(role);
        Ok(self)
    }

    /// Registers the declared recovered-resource cleanup role.
    pub fn with_provider_recovered_resource_cleanup(
        mut self,
        role: Arc<dyn ProviderRecoveredResourceCleanupDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::ProviderRecoveredResourceCleanup)?;
        self.provider_recovered_resource_cleanup = Some(role);
        Ok(self)
    }

    /// Returns the descriptor whose declarations bound this registration.
    #[must_use]
    pub const fn descriptor(&self) -> &DriverDescriptor {
        &self.descriptor
    }

    /// Returns the discovery role when registered.
    #[must_use]
    pub fn discovery(&self) -> Option<&Arc<dyn DiscoveryDriver>> {
        self.discovery.as_ref()
    }

    /// Returns the structured-run role when registered.
    #[must_use]
    pub fn structured_run(&self) -> Option<&Arc<dyn StructuredRunDriver>> {
        self.structured_run.as_ref()
    }

    /// Returns the model-catalogue role when registered.
    #[must_use]
    pub fn model_catalog(&self) -> Option<&Arc<dyn ModelCatalogDriver>> {
        self.model_catalog.as_ref()
    }

    /// Returns the interactive-session role when registered.
    #[must_use]
    pub fn interactive_session(&self) -> Option<&Arc<dyn InteractiveSessionDriver>> {
        self.interactive_session.as_ref()
    }

    /// Returns the serving-instance role when registered.
    #[must_use]
    pub fn serving_instance(&self) -> Option<&Arc<dyn ServingInstanceDriver>> {
        self.serving_instance.as_ref()
    }

    /// Returns the realtime-media session role when registered.
    #[must_use]
    pub fn realtime_media_session(&self) -> Option<&Arc<dyn RealtimeMediaSessionDriver>> {
        self.realtime_media_session.as_ref()
    }

    /// Returns the provider-session management role when registered.
    #[must_use]
    pub fn provider_session_management(&self) -> Option<&Arc<dyn ProviderSessionManagementDriver>> {
        self.provider_session_management.as_ref()
    }

    /// Returns the provider-session catalogue role when registered.
    #[must_use]
    pub fn provider_session_catalogue(&self) -> Option<&Arc<dyn ProviderSessionCatalogueDriver>> {
        self.provider_session_catalogue.as_ref()
    }

    /// Returns the provider-session import role when registered.
    #[must_use]
    pub fn provider_session_import(&self) -> Option<&Arc<dyn ProviderSessionImportDriver>> {
        self.provider_session_import.as_ref()
    }

    /// Returns the provider-session reconciliation role when registered.
    #[must_use]
    pub fn provider_session_reconciliation(
        &self,
    ) -> Option<&Arc<dyn ProviderSessionReconciliationDriver>> {
        self.provider_session_reconciliation.as_ref()
    }

    /// Returns the provider-session history role when registered.
    #[must_use]
    pub fn provider_session_history(&self) -> Option<&Arc<dyn ProviderSessionHistoryDriver>> {
        self.provider_session_history.as_ref()
    }

    /// Returns the provider-run reconciliation role when registered.
    #[must_use]
    pub fn provider_run_reconciliation(&self) -> Option<&Arc<dyn ProviderRunReconciliationDriver>> {
        self.provider_run_reconciliation.as_ref()
    }

    /// Returns the recovered-resource cleanup role when registered.
    #[must_use]
    pub fn provider_recovered_resource_cleanup(
        &self,
    ) -> Option<&Arc<dyn ProviderRecoveredResourceCleanupDriver>> {
        self.provider_recovered_resource_cleanup.as_ref()
    }

    fn require_declared(&self, role: DriverRole) -> Result<(), RegistrationFailure> {
        if self.descriptor.supports_role(role) {
            Ok(())
        } else {
            Err(RegistrationFailure::undeclared(role))
        }
    }
}
