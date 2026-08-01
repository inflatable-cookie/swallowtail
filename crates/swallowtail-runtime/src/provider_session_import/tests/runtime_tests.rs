use super::*;
use crate::{
    AccessEvidenceSourceId, BoxFuture, CancellationControl, CleanupOutcome, DriverRegistration,
    HostServices, ImmediateCancellation, JoinedTask, PreparedAccessEvidence,
    ProviderSessionCatalogueDriver, ProviderSessionImportDriver, ProviderSessionImportRevalidation,
    ProviderSessionOperationFailure, ProviderSessionOperationFailureStage, ResourceLease, ScopeId,
    ScopedTaskService, TimeService, WorkingResourceService,
    validate_provider_session_catalogue_execution, validate_provider_session_import_execution,
};
use std::sync::Arc;
use swallowtail_core::{
    CancellationScope, ProviderSessionBindingOrigin, ResourceRepresentation, SafeDiagnostic,
};

struct NoopJoinedTask;

impl JoinedTask for NoopJoinedTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), crate::RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }
}

struct NoopTaskService;

impl ScopedTaskService for NoopTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        _task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, crate::RuntimeFailure> {
        Ok(Box::new(NoopJoinedTask))
    }
}

struct NoopTimeService;

impl TimeService for NoopTimeService {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, crate::DeadlineObservation> {
        Box::pin(async move { crate::DeadlineObservation::new(deadline, deadline.instant()) })
    }
}

struct NoopWorkingResourceService;

impl WorkingResourceService for NoopWorkingResourceService {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, crate::RuntimeFailure>> {
        Box::pin(async move {
            Ok(ResourceLease::consumer_owned(
                scope,
                reference,
                access,
                representation,
            ))
        })
    }

    fn create_temporary(
        &self,
        scope: ScopeId,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, crate::RuntimeFailure>> {
        Box::pin(async move {
            Ok(ResourceLease::consumer_owned(
                scope,
                WorkingResourceRef::new("temporary").expect("resource is valid"),
                access,
                representation,
            ))
        })
    }

    fn release(&self, _lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async { CleanupOutcome::Clean })
    }
}

fn host_services(fixture: &Fixture) -> HostServices {
    HostServices::new(fixture.instance.execution_host_id().clone())
        .with_task(Arc::new(NoopTaskService))
        .with_time(Arc::new(NoopTimeService))
        .with_working_resource(Arc::new(NoopWorkingResourceService))
}

struct RoleStub;

impl ProviderSessionCatalogueDriver for RoleStub {
    fn list_provider_sessions(
        &self,
        _plan: ProviderSessionCataloguePlan,
        _request: ProviderSessionCatalogueRequest,
        _services: HostServices,
    ) -> BoxFuture<
        '_,
        Result<crate::ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure>,
    > {
        Box::pin(async {
            Err(ProviderSessionOperationFailure::new(
                ProviderSessionOperationFailureStage::CatalogueDispatch,
                SafeDiagnostic::new("fixture.catalogue", "fixture catalogue stop"),
            ))
        })
    }
}

impl ProviderSessionImportDriver for RoleStub {
    fn import_provider_session(
        &self,
        _plan: ProviderSessionImportPlan,
        _request: ProviderSessionImportRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<crate::ProviderSessionImportOutcome, ProviderSessionOperationFailure>>
    {
        Box::pin(async {
            Err(ProviderSessionOperationFailure::new(
                ProviderSessionOperationFailureStage::ImportRevalidation,
                SafeDiagnostic::new("fixture.import", "fixture import stop"),
            ))
        })
    }
}

#[test]
fn runtime_roles_are_object_safe_and_register_independently() {
    let fixture = fixture();
    let role = Arc::new(RoleStub);
    let registration = DriverRegistration::new(fixture.driver)
        .with_provider_session_catalogue(role.clone())
        .expect("catalogue role is declared")
        .with_provider_session_import(role)
        .expect("import role is declared");

    assert!(registration.provider_session_catalogue().is_some());
    assert!(registration.provider_session_import().is_some());
}

#[test]
fn catalogue_outcome_enforces_page_identity_bounds_and_cleanup() {
    let fixture = fixture();
    let plan = catalogue_plan(&fixture, "catalogue-a", "resource-a", bounds(128, 128));
    let request = ProviderSessionCatalogueRequest::from_plan(
        RequestId::new("list-a").expect("request id is valid"),
        &plan,
        None,
    )
    .expect("request is valid");
    let selected = candidate(
        &plan,
        "candidate-a",
        ProviderSessionImportAvailability::Available,
    );
    let outcome = crate::ProviderSessionCatalogueOutcome::new(
        &plan,
        &request,
        vec![selected.clone()],
        Some("private-next".to_owned()),
        CleanupOutcome::Clean,
    )
    .expect("page is valid");
    let duplicate = crate::ProviderSessionCatalogueOutcome::new(
        &plan,
        &request,
        vec![selected.clone(), selected],
        None,
        CleanupOutcome::Clean,
    )
    .expect_err("duplicate candidates must fail");
    let next_request = ProviderSessionCatalogueRequest::from_plan(
        RequestId::new("list-b").expect("request id is valid"),
        &plan,
        outcome.next_cursor().cloned(),
    )
    .expect("next-page request is valid");
    let cross_page_duplicate = crate::ProviderSessionCatalogueOutcome::new(
        &plan,
        &next_request,
        vec![candidate(
            &plan,
            "candidate-a",
            ProviderSessionImportAvailability::Available,
        )],
        None,
        CleanupOutcome::Clean,
    )
    .expect_err("cross-page duplicate must fail");
    let degraded = crate::ProviderSessionCatalogueOutcome::new(
        &plan,
        &request,
        Vec::new(),
        None,
        CleanupOutcome::Degraded(SafeDiagnostic::new("fixture.cleanup", "cleanup degraded")),
    )
    .expect_err("cleanup degradation must fail the page");

    assert_eq!(outcome.candidates().len(), 1);
    assert_eq!(
        outcome
            .next_cursor()
            .expect("next cursor exists")
            .observed_candidates(),
        1
    );
    assert_eq!(
        duplicate.stage(),
        ProviderSessionOperationFailureStage::CatalogueProjection
    );
    assert_eq!(
        cross_page_duplicate.stage(),
        ProviderSessionOperationFailureStage::CatalogueProjection
    );
    assert_eq!(
        degraded.stage(),
        ProviderSessionOperationFailureStage::Cleanup
    );
}

#[test]
fn import_outcome_requires_revalidation_and_mints_only_imported_binding() {
    let fixture = fixture();
    let catalogue = catalogue_plan(&fixture, "catalogue-a", "resource-a", bounds(128, 128));
    let selected = candidate(
        &catalogue,
        "candidate-a",
        ProviderSessionImportAvailability::Available,
    );
    let import = import_plan(&fixture, catalogue, selected, "resource-a", true)
        .expect("import plan is valid");
    let request = ProviderSessionImportRequest::from_plan(
        RequestId::new("import-a").expect("request id is valid"),
        &import,
    )
    .expect("import request is valid");
    let wrong_cancellation = ProviderSessionImportRequest::new(
        RequestId::new("import-wrong-scope").expect("request id is valid"),
        &import,
        Arc::new(ImmediateCancellation::new(
            CancellationScope::ProviderSessionCatalogue,
        )),
    )
    .expect_err("wrong cancellation scope must fail before dispatch");
    let revalidation = ProviderSessionImportRevalidation::new(
        import.agreement().candidate_id().clone(),
        SessionRef::new("provider/private/session").expect("session ref is valid"),
        WorkingResourceRef::new("resource-a").expect("resource is valid"),
        ProviderSessionActivityState::Inactive,
        ProviderSessionImportAvailability::Available,
    );
    let degraded = crate::ProviderSessionImportOutcome::new(
        &import,
        &request,
        revalidation.clone(),
        CleanupOutcome::Degraded(SafeDiagnostic::new("fixture.cleanup", "cleanup degraded")),
    )
    .expect_err("cleanup degradation must issue no binding");
    let outcome = crate::ProviderSessionImportOutcome::new(
        &import,
        &request,
        revalidation,
        CleanupOutcome::Clean,
    )
    .expect("matching revalidation issues a binding");
    let mismatch = ProviderSessionImportRevalidation::new(
        import.agreement().candidate_id().clone(),
        SessionRef::new("provider/other/session").expect("session ref is valid"),
        WorkingResourceRef::new("resource-a").expect("resource is valid"),
        ProviderSessionActivityState::Inactive,
        ProviderSessionImportAvailability::Available,
    );
    let mismatch = crate::ProviderSessionImportOutcome::new(
        &import,
        &request,
        mismatch,
        CleanupOutcome::Clean,
    )
    .expect_err("changed provider identity must not issue a binding");

    assert_eq!(
        outcome.binding().origin(),
        ProviderSessionBindingOrigin::ExplicitlyImported
    );
    assert!(outcome.binding().matches_attachment(
        import.preflight(),
        import.agreement().working_resource(),
        import.agreement().session().access_policy(),
    ));
    assert_eq!(
        mismatch.stage(),
        ProviderSessionOperationFailureStage::ImportRevalidation
    );
    assert_eq!(
        degraded.stage(),
        ProviderSessionOperationFailureStage::Cleanup
    );
    assert_eq!(
        wrong_cancellation.diagnostic().code(),
        "swallowtail.provider_session_import.cancellation_scope_mismatch"
    );
}

#[test]
fn prepared_evidence_and_host_validation_preserve_exact_plan() {
    let fixture = fixture();
    let catalogue = catalogue_plan(&fixture, "catalogue-a", "resource-a", bounds(128, 128));
    let catalogue_request = ProviderSessionCatalogueRequest::from_plan(
        RequestId::new("list-a").expect("request id is valid"),
        &catalogue,
        None,
    )
    .expect("catalogue request is valid");
    let prepared_catalogue = crate::PreparedProviderSessionCatalogueEvidence::from_plan(
        catalogue.clone(),
        PreparedAccessEvidence::observed(
            fixture.access_status.clone(),
            AccessEvidenceSourceId::new("fixture.access.source").expect("source id is valid"),
        ),
    )
    .expect("catalogue evidence is valid");
    let selected = candidate(
        &catalogue,
        "candidate-a",
        ProviderSessionImportAvailability::Available,
    );
    let import = import_plan(&fixture, catalogue, selected, "resource-a", true)
        .expect("import plan is valid");
    let import_request = ProviderSessionImportRequest::from_plan(
        RequestId::new("import-a").expect("request id is valid"),
        &import,
    )
    .expect("import request is valid");
    let prepared_import = crate::PreparedProviderSessionImportEvidence::from_plan(
        import.clone(),
        PreparedAccessEvidence::observed(
            fixture.access_status.clone(),
            AccessEvidenceSourceId::new("fixture.access.source").expect("source id is valid"),
        ),
    )
    .expect("import evidence is valid");
    let services = host_services(&fixture);

    validate_provider_session_catalogue_execution(
        prepared_catalogue.plan(),
        &catalogue_request,
        &services,
    )
    .expect("catalogue services match");
    validate_provider_session_import_execution(prepared_import.plan(), &import_request, &services)
        .expect("import services match");

    let missing = HostServices::new(fixture.instance.execution_host_id().clone());
    let missing = validate_provider_session_import_execution(&import, &import_request, &missing)
        .expect_err("missing services fail before dispatch");
    assert_eq!(
        missing.stage(),
        ProviderSessionOperationFailureStage::BeforeDispatch
    );
    assert_eq!(
        import_request.cancellation().scope(),
        CancellationScope::ProviderSessionImport
    );
}
