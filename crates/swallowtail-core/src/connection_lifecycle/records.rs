use super::descriptor::RouteTopology;
use super::identity::{AddableRouteId, ConfigFieldId, CredentialFieldId, InstanceLabel};
use crate::access::AccessStatus;
use crate::diagnostic::{ValueRequired, required_text};
use crate::identity::AdapterIdentity;
use crate::model::{ModelId, ProviderId};
use crate::runtime_identity::{
    ConfigFieldRef, ConfiguredInstanceId, CredentialRef, IntegrationFamilyId,
};
use std::collections::BTreeMap;
use std::fmt;

/// Host preference for whether an admitted instance is enabled.
///
/// Enablement is independent of access-status dimensions and of 047
/// `Ready` / `NotReady`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InstanceEnablement {
    /// The host wants this instance included in ordinary use.
    Enabled,
    /// The host wants this instance retained but not ordinarily used.
    Disabled,
}

/// Disclosure state for one authenticated-subject field.
#[derive(Clone, Eq, PartialEq)]
pub enum SubjectDisclosure {
    /// The provider did not disclose this field.
    Absent,
    /// The provider disclosed this field; the value stays redacted.
    Redacted,
    /// The consumer revealed the disclosed value for presentation.
    Revealed(String),
}

impl SubjectDisclosure {
    #[must_use]
    /// Returns the revealed text, when the consumer has unredacted it.
    pub fn revealed_text(&self) -> Option<&str> {
        match self {
            Self::Revealed(value) => Some(value.as_str()),
            Self::Absent | Self::Redacted => None,
        }
    }

    #[must_use]
    /// Reports whether this field currently exposes revealed text.
    pub const fn is_revealed(&self) -> bool {
        matches!(self, Self::Revealed(_))
    }

    #[must_use]
    /// Collapses revealed text to redacted. Absent stays absent.
    pub const fn without_revealed_text(&self) -> Self {
        match self {
            Self::Absent => Self::Absent,
            Self::Redacted | Self::Revealed(_) => Self::Redacted,
        }
    }
}

impl fmt::Debug for SubjectDisclosure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Absent => formatter.write_str("Absent"),
            Self::Redacted => formatter.write_str("Redacted"),
            Self::Revealed(_) => formatter
                .debug_tuple("Revealed")
                .field(&"<redacted>")
                .finish(),
        }
    }
}

/// Provider-disclosed authenticated subject: email, login, or plan.
///
/// The observation defaults to redacted. It is never a configured-instance
/// id, 047 selection field, diagnostic, or routing key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthenticatedSubjectObservation {
    email: SubjectDisclosure,
    login: SubjectDisclosure,
    plan: SubjectDisclosure,
}

impl AuthenticatedSubjectObservation {
    /// Creates a fully redacted subject observation.
    #[must_use]
    pub const fn redacted() -> Self {
        Self {
            email: SubjectDisclosure::Redacted,
            login: SubjectDisclosure::Redacted,
            plan: SubjectDisclosure::Redacted,
        }
    }

    /// Creates an observation where the provider disclosed no subject fields.
    #[must_use]
    pub const fn undisclosed() -> Self {
        Self {
            email: SubjectDisclosure::Absent,
            login: SubjectDisclosure::Absent,
            plan: SubjectDisclosure::Absent,
        }
    }

    #[must_use]
    /// Marks email as disclosed by the provider and still redacted.
    pub fn with_email_disclosed(mut self) -> Self {
        self.email = SubjectDisclosure::Redacted;
        self
    }

    #[must_use]
    /// Marks login as disclosed by the provider and still redacted.
    pub fn with_login_disclosed(mut self) -> Self {
        self.login = SubjectDisclosure::Redacted;
        self
    }

    #[must_use]
    /// Marks plan as disclosed by the provider and still redacted.
    pub fn with_plan_disclosed(mut self) -> Self {
        self.plan = SubjectDisclosure::Redacted;
        self
    }

    #[must_use]
    /// Marks email as not disclosed by the provider.
    pub fn with_email_absent(mut self) -> Self {
        self.email = SubjectDisclosure::Absent;
        self
    }

    #[must_use]
    /// Marks login as not disclosed by the provider.
    pub fn with_login_absent(mut self) -> Self {
        self.login = SubjectDisclosure::Absent;
        self
    }

    #[must_use]
    /// Marks plan as not disclosed by the provider.
    pub fn with_plan_absent(mut self) -> Self {
        self.plan = SubjectDisclosure::Absent;
        self
    }

    #[must_use]
    /// Returns a copy with revealed text collapsed. Absent stays absent.
    pub const fn without_revealed_text(&self) -> Self {
        Self {
            email: self.email.without_revealed_text(),
            login: self.login.without_revealed_text(),
            plan: self.plan.without_revealed_text(),
        }
    }

    /// Reveals email text for consumer presentation.
    pub fn reveal_email(mut self, value: impl Into<String>) -> Result<Self, ValueRequired> {
        self.email = SubjectDisclosure::Revealed(required_text("subject email", value)?);
        Ok(self)
    }

    /// Reveals login text for consumer presentation.
    pub fn reveal_login(mut self, value: impl Into<String>) -> Result<Self, ValueRequired> {
        self.login = SubjectDisclosure::Revealed(required_text("subject login", value)?);
        Ok(self)
    }

    /// Reveals plan text for consumer presentation.
    pub fn reveal_plan(mut self, value: impl Into<String>) -> Result<Self, ValueRequired> {
        self.plan = SubjectDisclosure::Revealed(required_text("subject plan", value)?);
        Ok(self)
    }

    #[must_use]
    /// Returns email disclosure. Revealed text is opt-in.
    pub const fn email(&self) -> &SubjectDisclosure {
        &self.email
    }

    #[must_use]
    /// Returns login disclosure. Revealed text is opt-in.
    pub const fn login(&self) -> &SubjectDisclosure {
        &self.login
    }

    #[must_use]
    /// Returns plan disclosure. Revealed text is opt-in.
    pub const fn plan(&self) -> &SubjectDisclosure {
        &self.plan
    }

    #[must_use]
    /// Reports whether every disclosed field is still redacted.
    pub const fn is_redacted(&self) -> bool {
        !self.email.is_revealed() && !self.login.is_revealed() && !self.plan.is_revealed()
    }
}

impl Default for AuthenticatedSubjectObservation {
    fn default() -> Self {
        Self::redacted()
    }
}

/// Presentation overlay bound to one exact catalogue model identity.
///
/// Markers cannot invent a model id. They do not change 047 `Ready` /
/// `NotReady` and they do not copy models across instances.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlayMarker {
    instance_id: ConfiguredInstanceId,
    provider_id: ProviderId,
    model_id: ModelId,
    hidden: bool,
    ordinal: Option<u32>,
    consumer_default: bool,
    favourite: bool,
}

impl OverlayMarker {
    /// Creates an overlay marker for one exact instance, provider, and model.
    ///
    /// `model_id` is a validated [`ModelId`], so an empty model id cannot be
    /// constructed.
    #[must_use]
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        provider_id: ProviderId,
        model_id: ModelId,
    ) -> Self {
        Self {
            instance_id,
            provider_id,
            model_id,
            hidden: false,
            ordinal: None,
            consumer_default: false,
            favourite: false,
        }
    }

    #[must_use]
    /// Marks the model hidden in the consumer presentation overlay.
    pub const fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    #[must_use]
    /// Sets a consumer-owned ordinal. `None` leaves provider order untouched.
    pub const fn with_ordinal(mut self, ordinal: Option<u32>) -> Self {
        self.ordinal = ordinal;
        self
    }

    #[must_use]
    /// Marks this model as the consumer default for the instance.
    pub const fn with_consumer_default(mut self, consumer_default: bool) -> Self {
        self.consumer_default = consumer_default;
        self
    }

    #[must_use]
    /// Marks this model as a consumer favourite.
    pub const fn with_favourite(mut self, favourite: bool) -> Self {
        self.favourite = favourite;
        self
    }

    #[must_use]
    /// Returns the configured instance this overlay belongs to.
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        &self.instance_id
    }

    #[must_use]
    /// Returns the provider identity from the catalogue result.
    pub const fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    #[must_use]
    /// Returns the exact catalogue model identity.
    pub const fn model_id(&self) -> &ModelId {
        &self.model_id
    }

    #[must_use]
    /// Returns whether the consumer hid this model.
    pub const fn hidden(&self) -> bool {
        self.hidden
    }

    #[must_use]
    /// Returns the consumer ordinal, when one was set.
    pub const fn ordinal(&self) -> Option<u32> {
        self.ordinal
    }

    #[must_use]
    /// Returns whether this is the consumer-default model for the instance.
    pub const fn consumer_default(&self) -> bool {
        self.consumer_default
    }

    #[must_use]
    /// Returns whether the consumer marked this model as a favourite.
    pub const fn favourite(&self) -> bool {
        self.favourite
    }
}

/// Store record for one admitted configured instance.
///
/// The record holds opaque references, enablement, and an optional label. It
/// never carries secret bytes, paths, or URLs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedInstanceRecord {
    id: ConfiguredInstanceId,
    family: IntegrationFamilyId,
    route_id: AddableRouteId,
    driver: AdapterIdentity,
    topology: RouteTopology,
    credential_refs: BTreeMap<CredentialFieldId, CredentialRef>,
    config_refs: BTreeMap<ConfigFieldId, ConfigFieldRef>,
    enablement: InstanceEnablement,
    label: Option<InstanceLabel>,
    access_status: Option<AccessStatus>,
}

impl AdmittedInstanceRecord {
    /// Creates an enabled instance record without labels, refs, or access status.
    #[must_use]
    pub fn new(
        id: ConfiguredInstanceId,
        family: IntegrationFamilyId,
        route_id: AddableRouteId,
        driver: AdapterIdentity,
        topology: RouteTopology,
    ) -> Self {
        Self {
            id,
            family,
            route_id,
            driver,
            topology,
            credential_refs: BTreeMap::new(),
            config_refs: BTreeMap::new(),
            enablement: InstanceEnablement::Enabled,
            label: None,
            access_status: None,
        }
    }

    #[must_use]
    /// Replaces stored credential references. Values stay opaque.
    pub fn with_credential_refs(
        mut self,
        refs: impl IntoIterator<Item = (CredentialFieldId, CredentialRef)>,
    ) -> Self {
        self.credential_refs = refs.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces stored config-field references. Values stay opaque.
    pub fn with_config_refs(
        mut self,
        refs: impl IntoIterator<Item = (ConfigFieldId, ConfigFieldRef)>,
    ) -> Self {
        self.config_refs = refs.into_iter().collect();
        self
    }

    #[must_use]
    /// Sets host enablement without changing access-status dimensions.
    pub const fn with_enablement(mut self, enablement: InstanceEnablement) -> Self {
        self.enablement = enablement;
        self
    }

    #[must_use]
    /// Sets an optional host-owned instance label.
    pub fn with_label(mut self, label: InstanceLabel) -> Self {
        self.label = Some(label);
        self
    }

    #[must_use]
    /// Stores observed access dimensions without deriving enablement from them.
    pub fn with_access_status(mut self, status: AccessStatus) -> Self {
        self.access_status = Some(status);
        self
    }

    #[must_use]
    /// Returns the configured-instance identity.
    pub const fn id(&self) -> &ConfiguredInstanceId {
        &self.id
    }

    #[must_use]
    /// Returns the integration family. Several instances may share a family.
    pub const fn family(&self) -> &IntegrationFamilyId {
        &self.family
    }

    #[must_use]
    /// Returns the addable route this instance was admitted from.
    pub const fn route_id(&self) -> &AddableRouteId {
        &self.route_id
    }

    #[must_use]
    /// Returns the driver identity for this instance.
    pub const fn driver(&self) -> &AdapterIdentity {
        &self.driver
    }

    #[must_use]
    /// Returns hosted, installed, or local-runtime topology.
    pub const fn topology(&self) -> RouteTopology {
        self.topology
    }

    /// Iterates credential references in stable field-id order.
    pub fn credential_refs(
        &self,
    ) -> impl ExactSizeIterator<Item = (&CredentialFieldId, &CredentialRef)> {
        self.credential_refs.iter()
    }

    /// Iterates config-field references in stable field-id order.
    pub fn config_refs(&self) -> impl ExactSizeIterator<Item = (&ConfigFieldId, &ConfigFieldRef)> {
        self.config_refs.iter()
    }

    #[must_use]
    /// Returns host enablement. This is not 047 selection readiness.
    pub const fn enablement(&self) -> InstanceEnablement {
        self.enablement
    }

    #[must_use]
    /// Returns the optional host-owned label.
    pub const fn label(&self) -> Option<&InstanceLabel> {
        self.label.as_ref()
    }

    #[must_use]
    /// Returns observed access dimensions, when stored.
    pub const fn access_status(&self) -> Option<&AccessStatus> {
        self.access_status.as_ref()
    }
}
