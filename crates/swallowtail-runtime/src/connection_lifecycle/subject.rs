use swallowtail_core::{AuthenticatedSubjectObservation, ConfiguredInstanceId};

use super::{ConnectionLifecycleStore, SubjectObservationFailure};

/// Observes a provider-disclosed subject for one admitted instance.
///
/// The observation is redacted by default and is not stored on the admitted
/// record or a 047 snapshot. Revealed values supplied by the adapter are
/// collapsed before return. The subject is never a configured-instance id,
/// routing key, or default diagnostic.
pub fn observe_authenticated_subject(
    store: &dyn ConnectionLifecycleStore,
    instance_id: &ConfiguredInstanceId,
    reported: AuthenticatedSubjectObservation,
) -> Result<AuthenticatedSubjectObservation, SubjectObservationFailure> {
    store
        .get_instance(instance_id)
        .map_err(SubjectObservationFailure::from_store)?
        .ok_or_else(SubjectObservationFailure::instance_absent)?;
    Ok(reported.without_revealed_text())
}
