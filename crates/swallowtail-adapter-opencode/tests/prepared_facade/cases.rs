use super::fixture::PreparedFixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use std::sync::atomic::Ordering;
use swallowtail_adapter_opencode::{
    OpenCodeCatalogueProfileInput, OpenCodeRunProfileInput, OpenCodeSessionManagementInput,
    OpenCodeSessionProfileInput, prepare_opencode_attached,
};
use swallowtail_core::{
    Capability, CapabilityConstraint, DriverRole, ExecutionHostId, HarnessConfigurationPosture,
    HarnessIsolation, InstanceOwnership, InterfaceCompatibilityAssessment,
    ObservableActivityAvailability, OwnedRemoteResourceKind, ProviderSessionDeletionStrength,
    ProviderSessionEffectTruth, ReasoningMode, SessionProviderStatePolicy,
    StructuredOutputEnforcement,
};
use swallowtail_runtime::{
    CancellationControl, CleanupOutcome, DiscoveryCancellation, HostServices, OperationContent,
    PreparationStage, ProviderRetentionPolicy, RemoteResourceDeletionOutcome, RequestId,
    SchemaDocument, StructuredOutputDescriptor, StructuredRunDriver, TerminalStatus,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

include!("cases/catalogue_and_session.rs");
include!("cases/structured_runs.rs");
include!("cases/structured_failures.rs");

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

include!("cases/preparation_and_deletion.rs");
include!("cases/delete_failures.rs");
