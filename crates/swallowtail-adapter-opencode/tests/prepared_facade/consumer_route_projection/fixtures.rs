use std::collections::{BTreeMap, BTreeSet};
use std::num::{NonZeroU32, NonZeroU64};

use futures_executor::block_on;
use swallowtail_adapter_opencode::{
    OpenCodeCatalogueProfileInput, OpenCodeRunProfileInput, OpenCodeSessionCatalogueInput,
    OpenCodeSessionHistoryInput, OpenCodeSessionManagementInput, OpenCodeSessionProfileInput,
    OpenCodeSessionReconciliationInput,
};
use swallowtail_core::{ProviderSessionCatalogueBounds, ReasoningMode, SessionRef};
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId, OperationContent,
    ProviderSessionCatalogueId, ProviderSessionHistoryBounds, ProviderSessionHistoryId,
    ProviderSessionManagementBinding, ProviderSessionReconciliationBounds, RequestId,
    RuntimeTurnId, SchemaDocument, SessionResumeBinding, StructuredOutputDescriptor,
};

use super::ledger::*;
use super::naming::{RowIdentity, identities};

fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("fixture source id is valid")
}

fn contribution(
    contribution: ConsumerRouteProjectionContribution,
    id: &str,
) -> ConsumerRouteProjectionContribution {
    let source = contribution
        .sources()
        .next()
        .expect("facade source is present");
    assert_eq!(source.id().as_str(), id);
    assert_eq!(contribution.sources().count(), 1);
    contribution
}

fn bounds() -> ProviderSessionCatalogueBounds {
    ProviderSessionCatalogueBounds::new(
        NonZeroU32::new(2).expect("bounds"),
        NonZeroU32::new(8).expect("bounds"),
        NonZeroU32::new(32).expect("bounds"),
        NonZeroU32::new(1024).expect("bounds"),
        NonZeroU32::new(256).expect("bounds"),
    )
    .expect("catalogue bounds are valid")
}

fn schema() -> StructuredOutputDescriptor {
    StructuredOutputDescriptor::new(
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"result":{"type":"string"}},"required":["result"],"additionalProperties":false}"#,
            4096,
        )
        .expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("schema descriptor is valid")
}

fn binding(
    fixture: &super::super::fixture::PreparedFixture,
    session: &swallowtail_adapter_opencode::OpenCodePreparedSession,
) -> SessionResumeBinding {
    let plan = session.plan();
    SessionResumeBinding::new(
        SessionRef::new("ses_fixture").expect("session ref"),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().expect("model route").clone(),
        plan.model_id().expect("model id").clone(),
        fixture.resource.clone(),
        session.request().access_policy().clone(),
    )
}

fn run_maximal() -> ConsumerRouteProjectionContribution {
    let fixture =
        super::super::fixture::PreparedFixture::new("opencode.ledger.run.maximal", "1.18.10");
    let prepared = fixture.prepared();
    let mut models = block_on(
        prepared
            .prepare_catalogue(OpenCodeCatalogueProfileInput::new(
                RequestId::new("opencode-ledger-run-catalogue").expect("request id"),
            ))
            .expect("catalogue prepares")
            .list_models(fixture.services()),
    )
    .expect("catalogue lists");
    let input = OpenCodeRunProfileInput::new(
        RequestId::new("opencode-ledger-run").expect("request id"),
        fixture.model().with_catalogue_entry(models.remove(0)),
        OperationContent::new("private OpenCode ledger prompt").expect("content"),
        fixture.resource.clone(),
    )
    .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning"))
    .with_structured_output(schema())
    .with_attachments([fixture.attachment()])
    .with_provider_callbacks();
    let run = prepared.prepare_run(input).expect("maximal run prepares");
    contribution(
        run.consumer_route_projection_contribution(source(RUN_MAXIMAL))
            .expect("maximal run contributes"),
        RUN_MAXIMAL,
    )
}

fn session_maximal() -> ConsumerRouteProjectionContribution {
    let fixture =
        super::super::fixture::PreparedFixture::new("opencode.ledger.session.maximal", "1.18.10");
    let session = fixture
        .prepared()
        .prepare_session(
            OpenCodeSessionProfileInput::new(
                RequestId::new("opencode-ledger-session").expect("request id"),
                fixture.model(),
                fixture.resource.clone(),
            )
            .with_image_attachments()
            .with_provider_callbacks(),
        )
        .expect("maximal session prepares");
    contribution(
        session
            .consumer_route_projection_contribution(source(SESSION_MAXIMAL))
            .expect("maximal session contributes"),
        SESSION_MAXIMAL,
    )
}

fn session_detached() -> ConsumerRouteProjectionContribution {
    let fixture =
        super::super::fixture::PreparedFixture::new("opencode.ledger.session.detached", "1.18.10");
    let session = fixture
        .prepared()
        .prepare_session(
            OpenCodeSessionProfileInput::new(
                RequestId::new("opencode-ledger-detached").expect("request id"),
                fixture.model(),
                fixture.resource.clone(),
            )
            .with_active_turn_detachment(),
        )
        .expect("detached session prepares");
    contribution(
        session
            .consumer_route_projection_contribution(source(SESSION_DETACHED))
            .expect("detached session contributes"),
        SESSION_DETACHED,
    )
}

fn catalogue() -> ConsumerRouteProjectionContribution {
    let fixture =
        super::super::fixture::PreparedFixture::new("opencode.ledger.catalogue", "1.18.10");
    let catalogue = fixture
        .prepared()
        .prepare_catalogue(OpenCodeCatalogueProfileInput::new(
            RequestId::new("opencode-ledger-catalogue").expect("request id"),
        ))
        .expect("catalogue prepares");
    contribution(
        catalogue
            .consumer_route_projection_contribution(source(CATALOGUE))
            .expect("catalogue contributes"),
        CATALOGUE,
    )
}

fn session_catalogue() -> ConsumerRouteProjectionContribution {
    let fixture =
        super::super::fixture::PreparedFixture::new("opencode.ledger.session.catalogue", "1.18.10");
    let catalogue = fixture
        .prepared()
        .prepare_session_catalogue(OpenCodeSessionCatalogueInput::new(
            RequestId::new("opencode-ledger-session-catalogue").expect("request id"),
            ProviderSessionCatalogueId::new("opencode-ledger-session-catalogue")
                .expect("catalogue id"),
            fixture.resource.clone(),
            bounds(),
        ))
        .expect("session catalogue prepares");
    contribution(
        catalogue
            .consumer_route_projection_contribution(source(SESSION_CATALOGUE))
            .expect("session catalogue contributes"),
        SESSION_CATALOGUE,
    )
}

fn session_import() -> ConsumerRouteProjectionContribution {
    let fixture =
        super::super::fixture::PreparedFixture::new("opencode.ledger.session.import", "1.18.10");
    let prepared = fixture.prepared();
    let catalogue = prepared
        .prepare_session_catalogue(OpenCodeSessionCatalogueInput::new(
            RequestId::new("opencode-ledger-import-catalogue").expect("request id"),
            ProviderSessionCatalogueId::new("opencode-ledger-import-catalogue")
                .expect("catalogue id"),
            fixture.resource.clone(),
            bounds(),
        ))
        .expect("session catalogue prepares");
    let page = block_on(catalogue.list_sessions(fixture.services())).expect("sessions list");
    let candidate = page
        .candidates()
        .next()
        .expect("fixture has an import candidate")
        .clone();
    let imported = prepared
        .prepare_session_import(
            &catalogue,
            candidate,
            OpenCodeSessionProfileInput::new(
                RequestId::new("opencode-ledger-import").expect("request id"),
                fixture.model(),
                fixture.resource.clone(),
            ),
        )
        .expect("session import prepares");
    contribution(
        imported
            .consumer_route_projection_contribution(source(SESSION_IMPORT))
            .expect("session import contributes"),
        SESSION_IMPORT,
    )
}

fn delete() -> ConsumerRouteProjectionContribution {
    let fixture = super::super::fixture::PreparedFixture::new("opencode.ledger.delete", "1.18.10");
    let prepared = fixture.prepared();
    let session = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("opencode-ledger-delete-session").expect("request id"),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .expect("session prepares");
    let handle = block_on(session.open_session(fixture.services())).expect("session opens");
    let binding: ProviderSessionManagementBinding = handle
        .management_binding()
        .expect("session has management binding")
        .clone();
    assert!(matches!(
        block_on(fixture.close_session(handle)),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    let deletion = prepared
        .prepare_delete_session(OpenCodeSessionManagementInput::new(
            RequestId::new("opencode-ledger-delete").expect("request id"),
            binding,
        ))
        .expect("delete prepares");
    contribution(
        deletion
            .consumer_route_projection_contribution(source(DELETE))
            .expect("delete contributes"),
        DELETE,
    )
}

fn history() -> ConsumerRouteProjectionContribution {
    let fixture = super::super::fixture::PreparedFixture::new("opencode.ledger.history", "1.18.10");
    let prepared = fixture.prepared();
    let session = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("opencode-ledger-history-session").expect("request id"),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .expect("session prepares");
    let history = prepared
        .prepare_session_history(OpenCodeSessionHistoryInput::new(
            RequestId::new("opencode-ledger-history").expect("request id"),
            ProviderSessionHistoryId::new("opencode-ledger-history").expect("history id"),
            fixture.model(),
            binding(&fixture, &session),
            ProviderSessionHistoryBounds::new(
                NonZeroU32::new(2).expect("bounds"),
                NonZeroU64::new(4096).expect("bounds"),
                NonZeroU32::new(64).expect("bounds"),
                NonZeroU32::new(8).expect("bounds"),
            ),
        ))
        .expect("history prepares");
    contribution(
        history
            .consumer_route_projection_contribution(source(HISTORY))
            .expect("history contributes"),
        HISTORY,
    )
}

fn reconciliation() -> ConsumerRouteProjectionContribution {
    let fixture =
        super::super::fixture::PreparedFixture::new("opencode.ledger.reconciliation", "1.18.10");
    let prepared = fixture.prepared();
    let session = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("opencode-ledger-reconciliation-session").expect("request id"),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .expect("session prepares");
    let reconciliation = prepared
        .prepare_session_reconciliation(OpenCodeSessionReconciliationInput::new(
            RequestId::new("opencode-ledger-reconciliation").expect("request id"),
            fixture.model(),
            binding(&fixture, &session),
            RuntimeTurnId::new("opencode-ledger-interrupted-turn").expect("turn id"),
            ProviderSessionReconciliationBounds::new(
                NonZeroU32::new(8).expect("bounds"),
                NonZeroU64::new(4096).expect("bounds"),
            ),
        ))
        .expect("reconciliation prepares");
    contribution(
        reconciliation
            .consumer_route_projection_contribution(source(RECONCILIATION))
            .expect("reconciliation contributes"),
        RECONCILIATION,
    )
}

pub(super) fn observed() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    BTreeMap::from([
        (CATALOGUE, identities(&catalogue())),
        (RUN_MAXIMAL, identities(&run_maximal())),
        (SESSION_MAXIMAL, identities(&session_maximal())),
        (SESSION_DETACHED, identities(&session_detached())),
        (SESSION_CATALOGUE, identities(&session_catalogue())),
        (SESSION_IMPORT, identities(&session_import())),
        (DELETE, identities(&delete())),
        (HISTORY, identities(&history())),
        (RECONCILIATION, identities(&reconciliation())),
    ])
}
