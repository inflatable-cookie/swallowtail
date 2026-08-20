use std::collections::BTreeMap;
use swallowtail_core::{
    AddableRouteAvailability, AddableRouteId, AdmittedInstanceRecord, ConfigFieldId,
    ConfigFieldRef, ConfiguredInstanceId, CredentialFieldId, CredentialRef, InstanceEnablement,
    InstanceLabel, IntegrationFamilyId,
};

use super::{AddableRouteCatalog, ConnectionLifecycleStore, InstanceAdmissionFailure};

/// Host-owned input for admitting one addable route into the store.
///
/// Admission writes an [`AdmittedInstanceRecord`]. It does not prepare, select
/// a model, or change 047 readiness. Discovery candidates cannot be admitted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstanceAdmissionRequest {
    instance_id: ConfiguredInstanceId,
    family: IntegrationFamilyId,
    route_id: AddableRouteId,
    credential_refs: BTreeMap<CredentialFieldId, CredentialRef>,
    config_refs: BTreeMap<ConfigFieldId, ConfigFieldRef>,
    enablement: InstanceEnablement,
    label: Option<InstanceLabel>,
}

impl InstanceAdmissionRequest {
    /// Creates an enabled admission request without refs or a label.
    #[must_use]
    pub fn new(
        instance_id: ConfiguredInstanceId,
        family: IntegrationFamilyId,
        route_id: AddableRouteId,
    ) -> Self {
        Self {
            instance_id,
            family,
            route_id,
            credential_refs: BTreeMap::new(),
            config_refs: BTreeMap::new(),
            enablement: InstanceEnablement::Enabled,
            label: None,
        }
    }

    #[must_use]
    /// Replaces credential references. Values stay opaque.
    pub fn with_credential_refs(
        mut self,
        refs: impl IntoIterator<Item = (CredentialFieldId, CredentialRef)>,
    ) -> Self {
        self.credential_refs = refs.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces config-field references. Values stay host-private.
    pub fn with_config_refs(
        mut self,
        refs: impl IntoIterator<Item = (ConfigFieldId, ConfigFieldRef)>,
    ) -> Self {
        self.config_refs = refs.into_iter().collect();
        self
    }

    #[must_use]
    /// Sets host enablement without changing 047 readiness.
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
    /// Returns the configured-instance identity to admit.
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        &self.instance_id
    }

    #[must_use]
    /// Returns the integration family. Several instances may share a family.
    pub const fn family(&self) -> &IntegrationFamilyId {
        &self.family
    }

    #[must_use]
    /// Returns the addable route to admit from the catalog.
    pub const fn route_id(&self) -> &AddableRouteId {
        &self.route_id
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
}

/// Admits one addable route plus host-owned configuration through the store.
///
/// The route must already be in `catalog` and available. Unavailable and
/// unsupported observations stay distinct. A discovered candidate is not an
/// addable-route row and cannot be admitted.
pub fn admit_instance(
    catalog: &AddableRouteCatalog,
    store: &dyn ConnectionLifecycleStore,
    request: InstanceAdmissionRequest,
) -> Result<AdmittedInstanceRecord, InstanceAdmissionFailure> {
    let descriptor = catalog
        .get(request.route_id())
        .ok_or_else(InstanceAdmissionFailure::route_absent)?;
    match descriptor.availability() {
        AddableRouteAvailability::Available => {}
        AddableRouteAvailability::Unavailable(_) => {
            return Err(InstanceAdmissionFailure::route_unavailable());
        }
        AddableRouteAvailability::Unsupported => {
            return Err(InstanceAdmissionFailure::route_unsupported());
        }
    }
    if request
        .credential_refs()
        .any(|(id, _)| descriptor.credential_field(id).is_none())
    {
        return Err(InstanceAdmissionFailure::unknown_credential_field());
    }
    if request
        .config_refs()
        .any(|(id, _)| descriptor.config_field(id).is_none())
    {
        return Err(InstanceAdmissionFailure::unknown_config_field());
    }

    let mut record = AdmittedInstanceRecord::new(
        request.instance_id.clone(),
        request.family.clone(),
        descriptor.id().clone(),
        descriptor.driver().clone(),
        descriptor.topology(),
    )
    .with_credential_refs(request.credential_refs)
    .with_config_refs(request.config_refs)
    .with_enablement(request.enablement);
    if let Some(label) = request.label {
        record = record.with_label(label);
    }

    store
        .put_instance(record.clone())
        .map_err(InstanceAdmissionFailure::from_store)?;
    Ok(record)
}
