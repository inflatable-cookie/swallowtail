use crate::{
    DiscoveryDriver, InteractiveSessionDriver, ModelCatalogDriver, RealtimeMediaSessionDriver,
    ServingInstanceDriver, StructuredRunDriver,
};
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use swallowtail_core::{DriverDescriptor, DriverRole, SafeDiagnostic};

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
    pub const fn role(&self) -> DriverRole {
        self.role
    }

    #[must_use]
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

#[derive(Clone)]
pub struct DriverRegistration {
    descriptor: DriverDescriptor,
    discovery: Option<Arc<dyn DiscoveryDriver>>,
    model_catalog: Option<Arc<dyn ModelCatalogDriver>>,
    structured_run: Option<Arc<dyn StructuredRunDriver>>,
    interactive_session: Option<Arc<dyn InteractiveSessionDriver>>,
    realtime_media_session: Option<Arc<dyn RealtimeMediaSessionDriver>>,
    serving_instance: Option<Arc<dyn ServingInstanceDriver>>,
}

impl DriverRegistration {
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
        }
    }

    pub fn with_discovery(
        mut self,
        role: Arc<dyn DiscoveryDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::Discovery)?;
        self.discovery = Some(role);
        Ok(self)
    }

    pub fn with_structured_run(
        mut self,
        role: Arc<dyn StructuredRunDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::StructuredRun)?;
        self.structured_run = Some(role);
        Ok(self)
    }

    pub fn with_model_catalog(
        mut self,
        role: Arc<dyn ModelCatalogDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::ModelCatalog)?;
        self.model_catalog = Some(role);
        Ok(self)
    }

    pub fn with_interactive_session(
        mut self,
        role: Arc<dyn InteractiveSessionDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::InteractiveSession)?;
        self.interactive_session = Some(role);
        Ok(self)
    }

    pub fn with_serving_instance(
        mut self,
        role: Arc<dyn ServingInstanceDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::ServingInstanceLifecycle)?;
        self.serving_instance = Some(role);
        Ok(self)
    }

    pub fn with_realtime_media_session(
        mut self,
        role: Arc<dyn RealtimeMediaSessionDriver>,
    ) -> Result<Self, RegistrationFailure> {
        self.require_declared(DriverRole::RealtimeMediaSession)?;
        self.realtime_media_session = Some(role);
        Ok(self)
    }

    #[must_use]
    pub const fn descriptor(&self) -> &DriverDescriptor {
        &self.descriptor
    }

    #[must_use]
    pub fn discovery(&self) -> Option<&Arc<dyn DiscoveryDriver>> {
        self.discovery.as_ref()
    }

    #[must_use]
    pub fn structured_run(&self) -> Option<&Arc<dyn StructuredRunDriver>> {
        self.structured_run.as_ref()
    }

    #[must_use]
    pub fn model_catalog(&self) -> Option<&Arc<dyn ModelCatalogDriver>> {
        self.model_catalog.as_ref()
    }

    #[must_use]
    pub fn interactive_session(&self) -> Option<&Arc<dyn InteractiveSessionDriver>> {
        self.interactive_session.as_ref()
    }

    #[must_use]
    pub fn serving_instance(&self) -> Option<&Arc<dyn ServingInstanceDriver>> {
        self.serving_instance.as_ref()
    }

    #[must_use]
    pub fn realtime_media_session(&self) -> Option<&Arc<dyn RealtimeMediaSessionDriver>> {
        self.realtime_media_session.as_ref()
    }

    fn require_declared(&self, role: DriverRole) -> Result<(), RegistrationFailure> {
        if self.descriptor.supports_role(role) {
            Ok(())
        } else {
            Err(RegistrationFailure::undeclared(role))
        }
    }
}
