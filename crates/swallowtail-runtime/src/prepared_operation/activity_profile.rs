use super::{PreparedInterfaceCompatibility, PreparedOperationBinding};
use crate::{PreparationFailure, PreparationStage};
use swallowtail_core::{
    ActivityInterfaceBasis, Capability, Diagnostic, DriverRole, ObservableActivityAvailability,
    ObservableActivityProfile, OperationShape, PreflightPlan, SafeDiagnostic,
};

pub(super) fn prepare_activity_profile(
    plan: &PreflightPlan,
    binding: &PreparedOperationBinding,
    interface_compatibility: &[PreparedInterfaceCompatibility],
    supplied: Option<ObservableActivityProfile>,
) -> Result<ObservableActivityProfile, PreparationFailure> {
    let activity_requirements = plan
        .requirements()
        .capabilities()
        .filter(|requirement| requirement.capability() == Capability::ObservableActivity)
        .collect::<Vec<_>>();
    let streaming_required = plan
        .requirements()
        .capabilities()
        .any(|requirement| requirement.capability() == Capability::StreamingEvents);
    let applicable = matches!(
        (binding.operation_shape(), binding.driver_role()),
        (OperationShape::StructuredRun, DriverRole::StructuredRun)
            | (
                OperationShape::InteractiveSession,
                DriverRole::InteractiveSession
            )
    );

    if !applicable {
        if !activity_requirements.is_empty() {
            return Err(failure(
                "swallowtail.prepared_operation.activity_not_applicable",
                "Observable activity is not applicable to this prepared operation role",
            ));
        }
        return match supplied {
            None => Ok(ObservableActivityProfile::not_applicable()),
            Some(profile)
                if profile.availability() == ObservableActivityAvailability::NotApplicable =>
            {
                Ok(profile)
            }
            Some(_) => Err(failure(
                "swallowtail.prepared_operation.activity_not_applicable",
                "Prepared operation cannot attach an applicable activity profile",
            )),
        };
    }

    let expected_basis = interface_compatibility
        .iter()
        .filter_map(|compatibility| {
            compatibility
                .assessment()
                .behavior_revision()
                .map(|revision| {
                    ActivityInterfaceBasis::new(
                        compatibility.binding().axis().clone(),
                        revision.clone(),
                    )
                })
        })
        .collect::<Vec<_>>();

    let Some(profile) = supplied else {
        if !activity_requirements.is_empty() {
            return Err(failure(
                "swallowtail.prepared_operation.activity_profile_required",
                "Observable activity requirements need an explicit prepared route profile",
            ));
        }
        return ObservableActivityProfile::unavailable(expected_basis).map_err(|_| {
            failure(
                "swallowtail.prepared_operation.activity_profile_invalid",
                "Prepared route activity profile could not be derived",
            )
        });
    };

    validate_profile(
        plan,
        profile,
        &activity_requirements,
        streaming_required,
        &expected_basis,
    )
}

fn validate_profile(
    plan: &PreflightPlan,
    profile: ObservableActivityProfile,
    activity_requirements: &[&swallowtail_core::CapabilityRequirement],
    streaming_required: bool,
    expected_basis: &[ActivityInterfaceBasis],
) -> Result<ObservableActivityProfile, PreparationFailure> {
    if profile.availability() == ObservableActivityAvailability::NotApplicable {
        return Err(failure(
            "swallowtail.prepared_operation.activity_profile_invalid",
            "Ordinary run and turn operations require an applicable activity profile",
        ));
    }
    if !profile.interface_basis().eq(expected_basis.iter().cloned()) {
        return Err(failure(
            "swallowtail.prepared_operation.activity_profile_basis_mismatch",
            "Prepared route activity profile does not match qualified interface behavior",
        ));
    }
    if activity_requirements.is_empty() {
        return if profile.availability() == ObservableActivityAvailability::Available {
            Err(failure(
                "swallowtail.prepared_operation.activity_profile_unchecked",
                "Available activity fidelity must be checked by immutable preflight",
            ))
        } else {
            Ok(profile)
        };
    }
    if !streaming_required {
        return Err(failure(
            "swallowtail.prepared_operation.activity_streaming_required",
            "Observable activity requires ordered streaming-event delivery",
        ));
    }
    if profile.availability() != ObservableActivityAvailability::Available {
        return Err(failure(
            "swallowtail.prepared_operation.activity_profile_unavailable",
            "Prepared route does not satisfy required observable activity",
        ));
    }
    if profile
        .capability_requirement()
        .is_none_or(|advertised| !plan.supports_capability_requirement(&advertised))
    {
        return Err(failure(
            "swallowtail.prepared_operation.activity_profile_unqualified",
            "Prepared route activity profile exceeds qualified capability evidence",
        ));
    }
    if activity_requirements
        .iter()
        .any(|requirement| !profile.supports(requirement))
    {
        return Err(failure(
            "swallowtail.prepared_operation.activity_constraint_mismatch",
            "Prepared route activity profile does not satisfy required semantics",
        ));
    }
    Ok(profile)
}

fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(SafeDiagnostic::new(code, message)),
    )
}
