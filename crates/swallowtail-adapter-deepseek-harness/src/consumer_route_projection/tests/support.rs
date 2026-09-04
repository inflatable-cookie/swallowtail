use super::*;

pub(super) fn assert_observation(
    observation: &ConsumerRouteProviderOperationObservation,
    semantic: &str,
    prepared: &ConsumerRouteProjectionContribution,
) {
    assert_eq!(observation.rows().len(), 1);
    let row = observation.rows().next().unwrap();
    let extension = row.identity().namespaced_extension().unwrap();
    assert_eq!(extension.route(), WEB_ROUTE);
    assert_eq!(extension.semantic_id(), semantic);
    assert_eq!(
        row.lifecycle(),
        ConsumerRouteLifecycle::PostOperationObservationOnly
    );
    assert_eq!(
        row.source_class(),
        ConsumerRouteSourceClass::ProviderOperationOutcome
    );
    assert_eq!(
        row.evidence_strength(),
        ConsumerRouteEvidenceStrength::CompletedProviderOperation
    );
    assert_eq!(
        row.actor_posture(),
        ConsumerRouteActorPosture::ObservationOnly
    );
    assert_eq!(
        row.state_support(),
        ConsumerRouteStateSupport::descriptor_only().with_observed()
    );
    assert_eq!(
        row.mutation_authority(),
        &ConsumerRouteMutationAuthority::Absent
    );
    assert_eq!(observation.applicability(), row.applicability());
    assert_eq!(
        observation.source().kind(),
        ConsumerRouteProjectionSourceKind::ProviderOperationObservation
    );
    assert!(
        prepared
            .sources()
            .all(|source| source != observation.source())
    );
    assert!(rows(prepared).all(|identity| &identity != row.identity()));
}

pub(super) fn rows(
    contribution: &ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = ConsumerRouteRowIdentity> + '_ {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| row.identity().clone())
}

pub(super) fn feature(id: ConsumerRouteFeatureId) -> ConsumerRouteRowIdentity {
    ConsumerRouteRowIdentity::Feature(id)
}

pub(super) fn control(id: ConsumerRouteControlId) -> ConsumerRouteRowIdentity {
    ConsumerRouteRowIdentity::Control(id)
}

pub(super) fn owned_runtime_lifecycle(identity: &ConsumerRouteRowIdentity) -> bool {
    identity
        .namespaced_extension()
        .is_some_and(|extension| extension.semantic_id() == "feature.owned-runtime-lifecycle")
}

pub(super) fn catalogue(
    web: &crate::DeepSeekHarnessWebPreparedIntegration,
    suffix: &str,
) -> DeepSeekHarnessWebPreparedSessionCatalogue {
    web.prepare_session_catalogue(DeepSeekHarnessWebSessionCatalogueInput::new(
        request(&format!("catalogue-{suffix}")),
        ProviderSessionCatalogueId::new(format!("projection-catalogue-{suffix}")).unwrap(),
        resource(),
        crate::web_prepared::tests::catalogue_bounds(),
    ))
    .unwrap()
}

pub(super) fn history(
    web: &crate::DeepSeekHarnessWebPreparedIntegration,
    session: SessionRef,
    suffix: &str,
) -> DeepSeekHarnessWebPreparedSessionHistory {
    let binding = SessionResumeBinding::new(
        session,
        web.instance().id().clone(),
        web.instance().execution_host_id().clone(),
        route(),
        model(),
        resource(),
        SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
    );
    web.prepare_session_history(DeepSeekHarnessWebSessionHistoryInput::new(
        request(&format!("history-{suffix}")),
        ProviderSessionHistoryId::new(format!("projection-history-{suffix}")).unwrap(),
        web_model(),
        binding,
        crate::web_prepared::tests::history_bounds(),
    ))
    .unwrap()
}

pub(super) fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).unwrap()
}
pub(super) fn request(id: &str) -> RequestId {
    RequestId::new(id).unwrap()
}
pub(super) fn route() -> ModelRouteId {
    ModelRouteId::new("deepseek-harness.web.fixture.route").unwrap()
}
pub(super) fn revision() -> ModelRouteRevision {
    ModelRouteRevision::new("fixture-v1").unwrap()
}
pub(super) fn provider() -> ProviderId {
    ProviderId::new("fixture-provider").unwrap()
}
pub(super) fn model() -> ModelId {
    ModelId::new("fixture-model").unwrap()
}
pub(super) fn web_model() -> DeepSeekHarnessWebModelSelection {
    DeepSeekHarnessWebModelSelection::new(route(), revision(), provider(), model())
}
pub(super) fn resource() -> WorkingResourceRef {
    WorkingResourceRef::new("deepseek-harness.web.fixture.workspace").unwrap()
}
pub(super) fn content() -> OperationContent {
    OperationContent::new("projection fixture").unwrap()
}
pub(super) fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(10_000))
}
