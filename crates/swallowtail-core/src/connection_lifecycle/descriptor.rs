use super::identity::{
    AddableRouteId, ConfigFieldId, CredentialFieldId, EnvironmentVariableName, FieldLabel,
};
use crate::identity::AdapterIdentity;
use crate::registration::SignInAction;
use std::collections::{BTreeMap, BTreeSet};

/// Presentation topology for one addable route.
///
/// This grouping is hosted, installed, or local-runtime. It is not
/// [`crate::ExecutionLayer`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RouteTopology {
    /// A hosted API or subscription endpoint.
    Hosted,
    /// An installed harness or application on the host.
    Installed,
    /// A local model runtime attached on the host.
    LocalRuntime,
}

/// Why an addable route is currently unavailable.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AddableRouteMissingRequirement {
    /// A required install is missing.
    Install,
    /// A required local runtime is missing.
    Runtime,
    /// A required host service is missing.
    HostService,
}

/// Observed availability of one addable route on the current host.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AddableRouteAvailability {
    /// The route can be admitted on this host.
    Available,
    /// The route is known but a named requirement is missing.
    Unavailable(AddableRouteMissingRequirement),
    /// The adapter will not offer this route on this host.
    Unsupported,
}

/// Whether a credential field holds a secret or public value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CredentialFieldVisibility {
    /// The field value is a host-owned secret. Records store only a reference.
    Secret,
    /// The field value is public configuration, still stored as a reference.
    Public,
}

/// Descriptor for one credential or sign-in field collected at admission.
///
/// The descriptor never carries secret bytes. Optional environment names are
/// names only, not resolved values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialFieldDescriptor {
    id: CredentialFieldId,
    label: FieldLabel,
    visibility: CredentialFieldVisibility,
    environment_name: Option<EnvironmentVariableName>,
}

impl CredentialFieldDescriptor {
    /// Creates a credential-field descriptor without an environment name.
    #[must_use]
    pub const fn new(
        id: CredentialFieldId,
        label: FieldLabel,
        visibility: CredentialFieldVisibility,
    ) -> Self {
        Self {
            id,
            label,
            visibility,
            environment_name: None,
        }
    }

    #[must_use]
    /// Names an environment variable the host may use. The name is not a value.
    pub fn with_environment_name(mut self, name: EnvironmentVariableName) -> Self {
        self.environment_name = Some(name);
        self
    }

    #[must_use]
    /// Returns the stable field identity.
    pub const fn id(&self) -> &CredentialFieldId {
        &self.id
    }

    #[must_use]
    /// Returns the operator-facing field label.
    pub const fn label(&self) -> &FieldLabel {
        &self.label
    }

    #[must_use]
    /// Returns whether the field is secret or public.
    pub const fn visibility(&self) -> CredentialFieldVisibility {
        self.visibility
    }

    #[must_use]
    /// Returns the optional environment-variable name, when advertised.
    pub const fn environment_name(&self) -> Option<&EnvironmentVariableName> {
        self.environment_name.as_ref()
    }
}

/// Kind of host-private configuration described by one field.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConfigFieldKind {
    /// Host-owned binary path. The path itself stays private.
    BinaryPath,
    /// Host-owned API endpoint. The URL itself stays private.
    ApiEndpoint,
    /// Host-owned environment body. The body itself stays private.
    Environment,
}

/// Descriptor for one per-instance configuration field.
///
/// Values stay host-private behind [`crate::ConfigFieldRef`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigFieldDescriptor {
    id: ConfigFieldId,
    label: FieldLabel,
    kind: ConfigFieldKind,
}

impl ConfigFieldDescriptor {
    /// Creates a config-field descriptor without a stored value.
    #[must_use]
    pub const fn new(id: ConfigFieldId, label: FieldLabel, kind: ConfigFieldKind) -> Self {
        Self { id, label, kind }
    }

    #[must_use]
    /// Returns the stable field identity.
    pub const fn id(&self) -> &ConfigFieldId {
        &self.id
    }

    #[must_use]
    /// Returns the operator-facing field label.
    pub const fn label(&self) -> &FieldLabel {
        &self.label
    }

    #[must_use]
    /// Returns the kind of host-private value the field describes.
    pub const fn kind(&self) -> ConfigFieldKind {
        self.kind
    }
}

/// Portable descriptor for one addable connection route.
///
/// Adapters construct these records. Consumers assemble a catalog. This type
/// does not authenticate, persist, or prepare.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddableRouteDescriptor {
    id: AddableRouteId,
    driver: AdapterIdentity,
    topology: RouteTopology,
    availability: AddableRouteAvailability,
    credential_fields: BTreeMap<CredentialFieldId, CredentialFieldDescriptor>,
    config_fields: BTreeMap<ConfigFieldId, ConfigFieldDescriptor>,
    sign_in_actions: BTreeSet<SignInAction>,
}

impl AddableRouteDescriptor {
    /// Creates a descriptor with identity, topology, and availability.
    #[must_use]
    pub fn new(
        id: AddableRouteId,
        driver: AdapterIdentity,
        topology: RouteTopology,
        availability: AddableRouteAvailability,
    ) -> Self {
        Self {
            id,
            driver,
            topology,
            availability,
            credential_fields: BTreeMap::new(),
            config_fields: BTreeMap::new(),
            sign_in_actions: BTreeSet::new(),
        }
    }

    #[must_use]
    /// Replaces credential-field descriptors for this route.
    pub fn with_credential_fields(
        mut self,
        fields: impl IntoIterator<Item = CredentialFieldDescriptor>,
    ) -> Self {
        self.credential_fields = fields
            .into_iter()
            .map(|field| (field.id().clone(), field))
            .collect();
        self
    }

    #[must_use]
    /// Replaces config-field descriptors for this route.
    pub fn with_config_fields(
        mut self,
        fields: impl IntoIterator<Item = ConfigFieldDescriptor>,
    ) -> Self {
        self.config_fields = fields
            .into_iter()
            .map(|field| (field.id().clone(), field))
            .collect();
        self
    }

    #[must_use]
    /// Replaces advertised sign-in actions for this route.
    pub fn with_sign_in_actions(mut self, actions: impl IntoIterator<Item = SignInAction>) -> Self {
        self.sign_in_actions = actions.into_iter().collect();
        self
    }

    #[must_use]
    /// Returns the stable addable-route identity.
    pub const fn id(&self) -> &AddableRouteId {
        &self.id
    }

    #[must_use]
    /// Returns the driver identity that owns this route.
    pub const fn driver(&self) -> &AdapterIdentity {
        &self.driver
    }

    #[must_use]
    /// Returns hosted, installed, or local-runtime topology.
    pub const fn topology(&self) -> RouteTopology {
        self.topology
    }

    #[must_use]
    /// Returns whether the route is available, unavailable, or unsupported.
    pub const fn availability(&self) -> AddableRouteAvailability {
        self.availability
    }

    /// Iterates credential-field descriptors in stable id order.
    pub fn credential_fields(&self) -> impl ExactSizeIterator<Item = &CredentialFieldDescriptor> {
        self.credential_fields.values()
    }

    /// Iterates config-field descriptors in stable id order.
    pub fn config_fields(&self) -> impl ExactSizeIterator<Item = &ConfigFieldDescriptor> {
        self.config_fields.values()
    }

    /// Iterates advertised sign-in actions in stable order.
    pub fn sign_in_actions(&self) -> impl ExactSizeIterator<Item = SignInAction> {
        self.sign_in_actions.iter().copied()
    }
}
