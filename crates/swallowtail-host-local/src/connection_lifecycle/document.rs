use super::state::StoreState;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use swallowtail_core::{
    AccessProfileId, AccessStatus, AdapterId, AdapterIdentity, AdapterVersion, AddableRouteId,
    AdmittedInstanceRecord, ConfigFieldId, ConfigFieldRef, ConfiguredInstanceId, CredentialFieldId,
    CredentialRef, CredentialState, EndpointAuthorization, EntitlementState, InstanceEnablement,
    InstanceLabel, IntegrationFamilyId, ModelId, OverlayMarker, ProviderId, RouteTopology,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::ConnectionLifecycleStoreFailure;

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct JsonDocument {
    instances: Vec<JsonInstance>,
    overlay_markers: Vec<JsonOverlayMarker>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonInstance {
    id: String,
    family: String,
    route_id: String,
    driver_id: String,
    driver_version: String,
    topology: String,
    credential_refs: BTreeMap<String, String>,
    config_refs: BTreeMap<String, String>,
    enablement: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_status: Option<JsonAccessStatus>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonAccessStatus {
    profile_id: String,
    credential: String,
    entitlement: String,
    endpoint_authorization: String,
    runtime_readiness: String,
    support_authority: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonOverlayMarker {
    instance_id: String,
    provider_id: String,
    model_id: String,
    hidden: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordinal: Option<u32>,
    consumer_default: bool,
    favourite: bool,
}

impl JsonDocument {
    pub(super) fn from_state(state: &StoreState) -> Self {
        Self {
            instances: state.instances.values().map(json_instance).collect(),
            overlay_markers: state.overlays.values().map(json_overlay).collect(),
        }
    }

    pub(super) fn into_state(self) -> Result<StoreState, ConnectionLifecycleStoreFailure> {
        let mut state = StoreState::default();
        for instance in self.instances {
            state.put_instance(instance_from_json(instance)?);
        }
        for marker in self.overlay_markers {
            state.put_overlay_marker(overlay_from_json(marker)?);
        }
        Ok(state)
    }
}

pub(super) fn refuse_secret_byte_fields(
    value: &Value,
) -> Result<(), ConnectionLifecycleStoreFailure> {
    if contains_secret_byte_field(value) {
        return Err(ConnectionLifecycleStoreFailure::new(
            "swallowtail.connection_lifecycle.json_secret_bytes_refused",
            "JSON-file store refuses to write secret bytes",
        ));
    }
    Ok(())
}

fn contains_secret_byte_field(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            map.keys().any(|key| is_secret_byte_field(key))
                || map.values().any(contains_secret_byte_field)
        }
        Value::Array(items) => items.iter().any(contains_secret_byte_field),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => false,
    }
}

fn is_secret_byte_field(key: &str) -> bool {
    matches!(
        key,
        "secret" | "secret_bytes" | "password" | "token_bytes" | "expose_secret"
    )
}

fn json_instance(record: &AdmittedInstanceRecord) -> JsonInstance {
    JsonInstance {
        id: record.id().as_str().to_owned(),
        family: record.family().as_str().to_owned(),
        route_id: record.route_id().as_str().to_owned(),
        driver_id: record.driver().id().as_str().to_owned(),
        driver_version: record.driver().version().as_str().to_owned(),
        topology: topology_name(record.topology()).to_owned(),
        credential_refs: record
            .credential_refs()
            .map(|(id, reference)| (id.as_str().to_owned(), reference.as_host_value().to_owned()))
            .collect(),
        config_refs: record
            .config_refs()
            .map(|(id, reference)| (id.as_str().to_owned(), reference.as_host_value().to_owned()))
            .collect(),
        enablement: enablement_name(record.enablement()).to_owned(),
        label: record.label().map(|label| label.as_str().to_owned()),
        access_status: record.access_status().map(json_access),
    }
}

fn json_overlay(marker: &OverlayMarker) -> JsonOverlayMarker {
    JsonOverlayMarker {
        instance_id: marker.instance_id().as_str().to_owned(),
        provider_id: marker.provider_id().as_str().to_owned(),
        model_id: marker.model_id().as_str().to_owned(),
        hidden: marker.hidden(),
        ordinal: marker.ordinal(),
        consumer_default: marker.consumer_default(),
        favourite: marker.favourite(),
    }
}

fn json_access(status: &AccessStatus) -> JsonAccessStatus {
    JsonAccessStatus {
        profile_id: status.profile_id().as_str().to_owned(),
        credential: credential_name(status.credential()).to_owned(),
        entitlement: entitlement_name(status.entitlement()).to_owned(),
        endpoint_authorization: endpoint_name(status.endpoint_authorization()).to_owned(),
        runtime_readiness: readiness_name(status.runtime_readiness()).to_owned(),
        support_authority: authority_name(status.support_authority()).to_owned(),
    }
}

fn instance_from_json(
    instance: JsonInstance,
) -> Result<AdmittedInstanceRecord, ConnectionLifecycleStoreFailure> {
    let mut record = AdmittedInstanceRecord::new(
        required_id(
            ConfiguredInstanceId::new,
            "configured instance id",
            instance.id,
        )?,
        required_id(
            IntegrationFamilyId::new,
            "integration family id",
            instance.family,
        )?,
        required_id(AddableRouteId::new, "addable route id", instance.route_id)?,
        AdapterIdentity::new(
            required_id(AdapterId::new, "adapter id", instance.driver_id)?,
            required_id(
                AdapterVersion::new,
                "adapter version",
                instance.driver_version,
            )?,
        ),
        parse_topology(&instance.topology)?,
    )
    .with_enablement(parse_enablement(&instance.enablement)?);
    let mut credential_refs = Vec::new();
    for (id, reference) in instance.credential_refs {
        credential_refs.push((
            required_id(CredentialFieldId::new, "credential field id", id)?,
            required_id(CredentialRef::new, "credential reference", reference)?,
        ));
    }
    let mut config_refs = Vec::new();
    for (id, reference) in instance.config_refs {
        config_refs.push((
            required_id(ConfigFieldId::new, "config field id", id)?,
            required_id(ConfigFieldRef::new, "config field reference", reference)?,
        ));
    }
    record = record
        .with_credential_refs(credential_refs)
        .with_config_refs(config_refs);
    if let Some(label) = instance.label {
        record = record.with_label(required_id(InstanceLabel::new, "instance label", label)?);
    }
    if let Some(status) = instance.access_status {
        record = record.with_access_status(access_from_json(status)?);
    }
    Ok(record)
}

fn overlay_from_json(
    marker: JsonOverlayMarker,
) -> Result<OverlayMarker, ConnectionLifecycleStoreFailure> {
    Ok(OverlayMarker::new(
        required_id(
            ConfiguredInstanceId::new,
            "configured instance id",
            marker.instance_id,
        )?,
        required_id(ProviderId::new, "provider id", marker.provider_id)?,
        required_id(ModelId::new, "model id", marker.model_id)?,
    )
    .with_hidden(marker.hidden)
    .with_ordinal(marker.ordinal)
    .with_consumer_default(marker.consumer_default)
    .with_favourite(marker.favourite))
}

fn access_from_json(
    status: JsonAccessStatus,
) -> Result<AccessStatus, ConnectionLifecycleStoreFailure> {
    Ok(AccessStatus::new(
        required_id(AccessProfileId::new, "access profile id", status.profile_id)?,
        parse_credential(&status.credential)?,
        parse_entitlement(&status.entitlement)?,
        parse_endpoint(&status.endpoint_authorization)?,
        parse_readiness(&status.runtime_readiness)?,
        parse_authority(&status.support_authority)?,
    ))
}

fn required_id<T, E>(
    ctor: impl FnOnce(String) -> Result<T, E>,
    field: &'static str,
    value: String,
) -> Result<T, ConnectionLifecycleStoreFailure> {
    ctor(value).map_err(|_| {
        ConnectionLifecycleStoreFailure::new(
            "swallowtail.connection_lifecycle.json_invalid",
            format!("JSON-file store contained an invalid {field}"),
        )
    })
}

fn parse_topology(value: &str) -> Result<RouteTopology, ConnectionLifecycleStoreFailure> {
    match value {
        "hosted" => Ok(RouteTopology::Hosted),
        "installed" => Ok(RouteTopology::Installed),
        "local_runtime" => Ok(RouteTopology::LocalRuntime),
        _ => invalid_enum("topology"),
    }
}

fn parse_enablement(value: &str) -> Result<InstanceEnablement, ConnectionLifecycleStoreFailure> {
    match value {
        "enabled" => Ok(InstanceEnablement::Enabled),
        "disabled" => Ok(InstanceEnablement::Disabled),
        _ => invalid_enum("enablement"),
    }
}

fn parse_credential(value: &str) -> Result<CredentialState, ConnectionLifecycleStoreFailure> {
    match value {
        "not_required" => Ok(CredentialState::NotRequired),
        "unknown" => Ok(CredentialState::Unknown),
        "required" => Ok(CredentialState::Required),
        "ready" => Ok(CredentialState::Ready),
        "expired" => Ok(CredentialState::Expired),
        "rejected" => Ok(CredentialState::Rejected),
        _ => invalid_enum("credential"),
    }
}

fn parse_entitlement(value: &str) -> Result<EntitlementState, ConnectionLifecycleStoreFailure> {
    match value {
        "unknown" => Ok(EntitlementState::Unknown),
        "available" => Ok(EntitlementState::Available),
        "unavailable" => Ok(EntitlementState::Unavailable),
        "exhausted" => Ok(EntitlementState::Exhausted),
        "restricted" => Ok(EntitlementState::Restricted),
        _ => invalid_enum("entitlement"),
    }
}

fn parse_endpoint(value: &str) -> Result<EndpointAuthorization, ConnectionLifecycleStoreFailure> {
    match value {
        "unknown" => Ok(EndpointAuthorization::Unknown),
        "allowed" => Ok(EndpointAuthorization::Allowed),
        "denied" => Ok(EndpointAuthorization::Denied),
        _ => invalid_enum("endpoint authorization"),
    }
}

fn parse_readiness(value: &str) -> Result<RuntimeReadiness, ConnectionLifecycleStoreFailure> {
    match value {
        "unknown" => Ok(RuntimeReadiness::Unknown),
        "ready" => Ok(RuntimeReadiness::Ready),
        "degraded" => Ok(RuntimeReadiness::Degraded),
        "unavailable" => Ok(RuntimeReadiness::Unavailable),
        _ => invalid_enum("runtime readiness"),
    }
}

fn parse_authority(value: &str) -> Result<SupportAuthority, ConnectionLifecycleStoreFailure> {
    match value {
        "provider_supported" => Ok(SupportAuthority::ProviderSupported),
        "integration_maintainer_supported" => Ok(SupportAuthority::IntegrationMaintainerSupported),
        "experimental_observed" => Ok(SupportAuthority::ExperimentalObserved),
        "prohibited" => Ok(SupportAuthority::Prohibited),
        _ => invalid_enum("support authority"),
    }
}

fn topology_name(topology: RouteTopology) -> &'static str {
    match topology {
        RouteTopology::Hosted => "hosted",
        RouteTopology::Installed => "installed",
        RouteTopology::LocalRuntime => "local_runtime",
    }
}

fn enablement_name(enablement: InstanceEnablement) -> &'static str {
    match enablement {
        InstanceEnablement::Enabled => "enabled",
        InstanceEnablement::Disabled => "disabled",
    }
}

fn credential_name(state: CredentialState) -> &'static str {
    match state {
        CredentialState::NotRequired => "not_required",
        CredentialState::Unknown => "unknown",
        CredentialState::Required => "required",
        CredentialState::Ready => "ready",
        CredentialState::Expired => "expired",
        CredentialState::Rejected => "rejected",
    }
}

fn entitlement_name(state: EntitlementState) -> &'static str {
    match state {
        EntitlementState::Unknown => "unknown",
        EntitlementState::Available => "available",
        EntitlementState::Unavailable => "unavailable",
        EntitlementState::Exhausted => "exhausted",
        EntitlementState::Restricted => "restricted",
    }
}

fn endpoint_name(state: EndpointAuthorization) -> &'static str {
    match state {
        EndpointAuthorization::Unknown => "unknown",
        EndpointAuthorization::Allowed => "allowed",
        EndpointAuthorization::Denied => "denied",
    }
}

fn readiness_name(state: RuntimeReadiness) -> &'static str {
    match state {
        RuntimeReadiness::Unknown => "unknown",
        RuntimeReadiness::Ready => "ready",
        RuntimeReadiness::Degraded => "degraded",
        RuntimeReadiness::Unavailable => "unavailable",
    }
}

fn authority_name(authority: SupportAuthority) -> &'static str {
    match authority {
        SupportAuthority::ProviderSupported => "provider_supported",
        SupportAuthority::IntegrationMaintainerSupported => "integration_maintainer_supported",
        SupportAuthority::ExperimentalObserved => "experimental_observed",
        SupportAuthority::Prohibited => "prohibited",
    }
}

fn invalid_enum<T>(field: &'static str) -> Result<T, ConnectionLifecycleStoreFailure> {
    Err(ConnectionLifecycleStoreFailure::new(
        "swallowtail.connection_lifecycle.json_invalid",
        format!("JSON-file store contained an invalid {field}"),
    ))
}

#[cfg(test)]
mod tests {
    use super::refuse_secret_byte_fields;

    #[test]
    fn json_document_refuses_secret_byte_fields() {
        let value = serde_json::json!({
            "instances": [{ "id": "work", "secret_bytes": "sk-secret-bytes-xyz" }]
        });
        let error = refuse_secret_byte_fields(&value).expect_err("secret bytes must be refused");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.connection_lifecycle.json_secret_bytes_refused"
        );
    }
}
