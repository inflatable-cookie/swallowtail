use crate::PreparedOperationEvidence;
use std::collections::BTreeSet;
use swallowtail_core::DriverRole;

use super::failure::failure;
use super::model::ConfiguredProviderModelCatalogueOutcome;
use super::{
    ConfiguredProviderInstanceAdmission, ConfiguredProviderInstanceCatalogueFailure,
    ConfiguredProviderInstanceCatalogueFailureKind, ConfiguredProviderInstanceRoute,
    ConfiguredProviderModelCatalogue, ConfiguredProviderModelCatalogueInput,
    MAX_CONFIGURED_PROVIDER_MODELS_PER_INSTANCE,
};

pub(super) fn validate_base(
    admission: &ConfiguredProviderInstanceAdmission,
) -> Result<(), ConfiguredProviderInstanceCatalogueFailure> {
    if admission.instance.driver_id() != admission.driver.identity().id() {
        return Err(failure(
            ConfiguredProviderInstanceCatalogueFailureKind::DriverMismatch,
            "swallowtail.provider_instance_catalogue.driver_mismatch",
            "Configured provider instance does not match its driver descriptor",
        ));
    }
    let status = admission.access_evidence.status();
    if admission.instance.access_profile_id() != admission.access_profile.id()
        || status.profile_id() != admission.access_profile.id()
        || admission.instance.support_authority() != admission.access_profile.support_authority()
        || status.support_authority() != admission.access_profile.support_authority()
    {
        return Err(failure(
            ConfiguredProviderInstanceCatalogueFailureKind::AccessMismatch,
            "swallowtail.provider_instance_catalogue.access_mismatch",
            "Configured provider instance access evidence does not match its access profile",
        ));
    }
    Ok(())
}

pub(super) fn validate_route(
    admission: &ConfiguredProviderInstanceAdmission,
    evidence: &PreparedOperationEvidence,
) -> Result<(), ConfiguredProviderInstanceCatalogueFailure> {
    let binding = evidence.binding();
    let plan = evidence.plan();
    if binding.driver_identity() != admission.driver.identity()
        || binding.transport_family() != admission.driver.transport_family()
        || binding.instance_id() != admission.instance.id()
        || binding.instance_revision() != admission.instance.revision()
        || binding.execution_host_id() != admission.instance.execution_host_id()
        || binding.instance_target() != admission.instance.target_reference()
        || binding.protocol_facade_id() != admission.instance.protocol_facade_id()
        || plan.instance_policy_id() != admission.instance.policy_id()
        || plan.access_profile_id() != admission.access_profile.id()
        || plan.credential_mechanism() != admission.access_profile.credential_mechanism()
        || plan.endpoint_audience() != admission.access_profile.endpoint_audience()
        || evidence.access() != &admission.access_evidence
    {
        return Err(failure(
            ConfiguredProviderInstanceCatalogueFailureKind::RouteMismatch,
            "swallowtail.provider_instance_catalogue.route_mismatch",
            "Prepared provider route does not match its configured instance admission",
        ));
    }
    Ok(())
}

pub(super) fn project_routes(
    evidence: &[PreparedOperationEvidence],
) -> Result<Vec<ConfiguredProviderInstanceRoute>, ConfiguredProviderInstanceCatalogueFailure> {
    let mut routes = Vec::with_capacity(evidence.len());
    for evidence in evidence {
        let route = ConfiguredProviderInstanceRoute::from_evidence(evidence);
        if routes.contains(&route) {
            return Err(failure(
                ConfiguredProviderInstanceCatalogueFailureKind::DuplicateRoute,
                "swallowtail.provider_instance_catalogue.route_duplicate",
                "Configured provider instance contains a duplicate prepared route",
            ));
        }
        routes.push(route);
    }
    Ok(routes)
}

pub(super) fn project_model_catalogue(
    admission: &ConfiguredProviderInstanceAdmission,
    routes: &[ConfiguredProviderInstanceRoute],
    input: &ConfiguredProviderModelCatalogueInput,
) -> Result<ConfiguredProviderModelCatalogue, ConfiguredProviderInstanceCatalogueFailure> {
    validate_route(admission, &input.source)?;
    let Some(source_index) = admission
        .prepared_routes
        .iter()
        .position(|evidence| evidence == &input.source)
    else {
        return Err(failure(
            ConfiguredProviderInstanceCatalogueFailureKind::ModelCatalogueSourceMissing,
            "swallowtail.provider_instance_catalogue.model_source_missing",
            "Model catalogue source is not an admitted prepared route",
        ));
    };
    let source_route = routes[source_index].clone();
    if source_route.driver_role() != DriverRole::ModelCatalog
        || source_route.model_route().is_some()
    {
        return Err(failure(
            ConfiguredProviderInstanceCatalogueFailureKind::ModelCatalogueSourceInvalid,
            "swallowtail.provider_instance_catalogue.model_source_invalid",
            "Model catalogue source is not an unselected model-catalogue route",
        ));
    }
    validate_models(&input.outcome)?;
    Ok(ConfiguredProviderModelCatalogue {
        source_route,
        outcome: input.outcome.clone(),
    })
}

fn validate_models(
    outcome: &ConfiguredProviderModelCatalogueOutcome,
) -> Result<(), ConfiguredProviderInstanceCatalogueFailure> {
    let ConfiguredProviderModelCatalogueOutcome::Available(entries) = outcome else {
        return Ok(());
    };
    if entries.len() > MAX_CONFIGURED_PROVIDER_MODELS_PER_INSTANCE {
        return Err(failure(
            ConfiguredProviderInstanceCatalogueFailureKind::LimitExceeded,
            "swallowtail.provider_instance_catalogue.model_limit_exceeded",
            "Configured provider instance exceeds the portable model limit",
        ));
    }
    let mut identities = BTreeSet::new();
    if entries
        .iter()
        .any(|entry| !identities.insert((entry.provider_id().cloned(), entry.id().clone())))
    {
        return Err(failure(
            ConfiguredProviderInstanceCatalogueFailureKind::DuplicateModel,
            "swallowtail.provider_instance_catalogue.model_duplicate",
            "Configured provider model catalogue contains a duplicate model identity",
        ));
    }
    Ok(())
}
