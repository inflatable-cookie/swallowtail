use crate::{
    ResourceLease, ResourceRepresentation, RuntimeFailure, SessionAccessPolicy, WorkingResourceRef,
};
use swallowtail_core::{PreflightPlan, SafeDiagnostic};

pub fn validate_session_access_plan(
    plan: &PreflightPlan,
    requested: &SessionAccessPolicy,
) -> Result<(), RuntimeFailure> {
    if plan.requirements().session_access_policy() == Some(requested) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.session_access.plan_mismatch",
            "Session access policy does not match its immutable preflight plan",
        ))
    }
}

pub fn validate_session_resource_lease(
    requested: &SessionAccessPolicy,
    resource: &WorkingResourceRef,
    lease: &ResourceLease,
) -> Result<(), RuntimeFailure> {
    if lease.reference() != resource {
        return Err(failure(
            "swallowtail.session_access.resource_mismatch",
            "Working-resource lease does not match the session request",
        ));
    }
    if lease.access() != requested.resource_access() {
        return Err(failure(
            "swallowtail.session_access.resource_access_mismatch",
            "Working-resource lease access does not match the session policy",
        ));
    }
    if lease.representation() != ResourceRepresentation::Filesystem {
        return Err(failure(
            "swallowtail.session_access.resource_representation_mismatch",
            "Working-resource lease is not a filesystem representation",
        ));
    }
    if lease.filesystem().is_none() {
        return Err(failure(
            "swallowtail.session_access.resource_filesystem_missing",
            "Working-resource lease does not expose its host-authorized filesystem root",
        ));
    }
    Ok(())
}

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

#[cfg(test)]
mod tests {
    use super::validate_session_resource_lease;
    use crate::{
        ResourceAccess, ResourceLease, ResourceRepresentation, ScopeId, SessionAccessPolicy,
        WorkingResourceRef,
    };

    #[test]
    fn resource_lease_must_match_reference_and_access() {
        let resource = WorkingResourceRef::new("resource-a").expect("reference is valid");
        let other = WorkingResourceRef::new("resource-b").expect("reference is valid");
        let scope = ScopeId::new("session-scope").expect("scope is valid");
        let policy = SessionAccessPolicy::bounded_workspace([]);
        let read_lease = ResourceLease::consumer_owned(
            scope.clone(),
            resource.clone(),
            ResourceAccess::Read,
            ResourceRepresentation::Filesystem,
        )
        .with_filesystem(
            crate::MaterializedResourceRef::new("/private/resource-a")
                .expect("materialized resource is valid"),
        );
        let other_lease = ResourceLease::consumer_owned(
            scope,
            other,
            ResourceAccess::ReadWrite,
            ResourceRepresentation::Filesystem,
        )
        .with_filesystem(
            crate::MaterializedResourceRef::new("/private/resource-b")
                .expect("materialized resource is valid"),
        );

        assert!(validate_session_resource_lease(&policy, &resource, &read_lease).is_err());
        assert!(validate_session_resource_lease(&policy, &resource, &other_lease).is_err());
    }
}
