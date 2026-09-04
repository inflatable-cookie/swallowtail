use crate::prepared_fixtures::{attempt_input, inventory_input, prepared, session_input};
use crate::support::Fixture;
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_adapter_ollama::{
    OllamaContextWindow, OllamaPreparedInferenceAttempt, OllamaPreparedIntegration,
    OllamaPreparedInventory, OllamaPreparedSession,
};
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, SchemaDocument, StructuredOutputDescriptor,
};

use super::ledger::{INFERENCE, INFERENCE_MAXIMAL, INVENTORY, OLLAMA_PROFILES, SESSION};
use super::naming::{RowIdentity, identities, source};

pub(super) struct Host {
    _fixture: Fixture,
    prepared: OllamaPreparedIntegration,
}

impl Host {
    pub(super) fn new() -> Self {
        let fixture = Fixture::new();
        let prepared = prepared(&fixture);
        Self {
            _fixture: fixture,
            prepared,
        }
    }

    pub(super) fn inventory(&self) -> OllamaPreparedInventory {
        self.prepared
            .prepare_inventory(inventory_input("ollama.projection.inventory"))
            .expect("inventory prepares")
    }

    pub(super) fn inference(&self) -> OllamaPreparedInferenceAttempt {
        self.prepared
            .prepare_inference_attempt(attempt_input("ollama.projection.inference"))
            .expect("inference prepares")
    }

    pub(super) fn inference_maximal(&self) -> OllamaPreparedInferenceAttempt {
        self.prepared
            .prepare_inference_attempt(
                attempt_input("ollama.projection.inference-maximal")
                    .with_reasoning_mode(ReasoningMode::new("high").expect("mode"))
                    .with_structured_output(schema())
                    .with_context_window(
                        OllamaContextWindow::from_u64(4096).expect("admitted window"),
                    ),
            )
            .expect("maximal inference prepares")
    }

    pub(super) fn session(&self) -> OllamaPreparedSession {
        self.prepared
            .prepare_session(
                session_input("ollama.projection.session").with_context_window(
                    OllamaContextWindow::from_u64(4096).expect("admitted window"),
                ),
            )
            .expect("session prepares")
    }

    pub(super) fn session_without_context(&self) -> OllamaPreparedSession {
        self.prepared
            .prepare_session(session_input("ollama.projection.session-min"))
            .expect("session prepares")
    }
}

pub(super) fn contribute(
    contribution: Result<
        ConsumerRouteProjectionContribution,
        swallowtail_runtime::ConsumerRouteProjectionFailure,
    >,
) -> ConsumerRouteProjectionContribution {
    contribution.expect("prepared Ollama facade contributes")
}

pub(super) fn inventory_contribution(id: &str) -> ConsumerRouteProjectionContribution {
    contribute(
        Host::new()
            .inventory()
            .consumer_route_projection_contribution(source(id)),
    )
}

pub(super) fn inference_contribution(id: &str) -> ConsumerRouteProjectionContribution {
    contribute(
        Host::new()
            .inference()
            .consumer_route_projection_contribution(source(id)),
    )
}

pub(super) fn inference_maximal_contribution(id: &str) -> ConsumerRouteProjectionContribution {
    contribute(
        Host::new()
            .inference_maximal()
            .consumer_route_projection_contribution(source(id)),
    )
}

pub(super) fn session_contribution(id: &str) -> ConsumerRouteProjectionContribution {
    contribute(
        Host::new()
            .session()
            .consumer_route_projection_contribution(source(id)),
    )
}

pub(super) fn observed_dispositions() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    assert_eq!(OLLAMA_PROFILES.len(), 4);
    BTreeMap::from([
        (
            INVENTORY,
            identities(&inventory_contribution("ollama.attached.inventory")),
        ),
        (
            INFERENCE,
            identities(&inference_contribution("ollama.attached.inference")),
        ),
        (
            INFERENCE_MAXIMAL,
            identities(&inference_maximal_contribution(
                "ollama.attached.inference-maximal",
            )),
        ),
        (
            SESSION,
            identities(&session_contribution("ollama.attached.session")),
        ),
    ])
}

pub(super) fn schema() -> StructuredOutputDescriptor {
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
