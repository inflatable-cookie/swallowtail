use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use swallowtail_core::{
    ConfiguredInstanceId, ModelCatalogEntry, ModelId, OverlayMarker, ProviderId, SafeDiagnostic,
};

use super::{ConnectionLifecycleStore, ConnectionLifecycleStoreFailure};
use crate::{ConfiguredProviderInstanceRecord, ConfiguredProviderInstanceSelectionReadiness};

/// Stable reason model-presentation overlay projection failed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModelPresentationOverlayFailureKind {
    /// A marker names a model identity that is not in this catalogue.
    UnknownModel,
    /// A marker belongs to a different configured instance.
    CrossInstance,
    /// The store rejected the overlay-marker list.
    Store,
}

/// Rejection raised while projecting overlay markers onto a catalogue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelPresentationOverlayFailure {
    kind: ModelPresentationOverlayFailureKind,
    diagnostic: SafeDiagnostic,
}

impl ModelPresentationOverlayFailure {
    fn new(
        kind: ModelPresentationOverlayFailureKind,
        code: &'static str,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    pub(super) fn unknown_model() -> Self {
        Self::new(
            ModelPresentationOverlayFailureKind::UnknownModel,
            "swallowtail.connection_lifecycle.overlay_unknown_model",
            "Overlay marker does not match a model in this catalogue",
        )
    }

    pub(super) fn cross_instance() -> Self {
        Self::new(
            ModelPresentationOverlayFailureKind::CrossInstance,
            "swallowtail.connection_lifecycle.overlay_cross_instance",
            "Overlay marker belongs to a different configured instance",
        )
    }

    pub(super) fn from_store(failure: ConnectionLifecycleStoreFailure) -> Self {
        Self {
            kind: ModelPresentationOverlayFailureKind::Store,
            diagnostic: failure.diagnostic().clone(),
        }
    }

    #[must_use]
    /// Returns the stable failure classification.
    pub const fn kind(&self) -> ModelPresentationOverlayFailureKind {
        self.kind
    }

    #[must_use]
    /// Returns the redacted overlay diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for ModelPresentationOverlayFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for ModelPresentationOverlayFailure {}

/// One catalogue identity with consumer overlay fields applied.
///
/// Provider catalogue default stays on [`Self::provider_default`]. It is not
/// rewritten as [`Self::consumer_default`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelPresentationOverlayEntry {
    provider_id: Option<ProviderId>,
    model_id: ModelId,
    hidden: bool,
    ordinal: Option<u32>,
    consumer_default: bool,
    favourite: bool,
    provider_default: bool,
}

impl ModelPresentationOverlayEntry {
    fn from_catalogue(entry: &ModelCatalogEntry, marker: Option<&OverlayMarker>) -> Self {
        Self {
            provider_id: entry.provider_id().cloned(),
            model_id: entry.id().clone(),
            hidden: marker.is_some_and(OverlayMarker::hidden),
            ordinal: marker.and_then(OverlayMarker::ordinal),
            consumer_default: marker.is_some_and(OverlayMarker::consumer_default),
            favourite: marker.is_some_and(OverlayMarker::favourite),
            provider_default: entry.metadata().is_default(),
        }
    }

    #[must_use]
    /// Returns the catalogue provider identity, when the source reported one.
    pub const fn provider_id(&self) -> Option<&ProviderId> {
        self.provider_id.as_ref()
    }

    #[must_use]
    /// Returns the exact catalogue model identity.
    pub const fn model_id(&self) -> &ModelId {
        &self.model_id
    }

    #[must_use]
    /// Returns whether the consumer hid this model.
    pub const fn hidden(&self) -> bool {
        self.hidden
    }

    #[must_use]
    /// Returns the consumer ordinal, when one was set.
    pub const fn ordinal(&self) -> Option<u32> {
        self.ordinal
    }

    #[must_use]
    /// Returns whether this is the consumer-default model for the instance.
    pub const fn consumer_default(&self) -> bool {
        self.consumer_default
    }

    #[must_use]
    /// Returns whether the consumer marked this model as a favourite.
    pub const fn favourite(&self) -> bool {
        self.favourite
    }

    #[must_use]
    /// Returns the provider catalogue default. This is not consumer-default.
    pub const fn provider_default(&self) -> bool {
        self.provider_default
    }
}

/// Overlay projection for one bound 047 catalogue result.
///
/// The projection does not mutate the snapshot and does not change
/// [`ConfiguredProviderInstanceSelectionReadiness`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelPresentationOverlay {
    instance_id: ConfiguredInstanceId,
    selection_readiness: ConfiguredProviderInstanceSelectionReadiness,
    entries: Vec<ModelPresentationOverlayEntry>,
}

impl ModelPresentationOverlay {
    #[must_use]
    /// Returns the configured instance this overlay belongs to.
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        &self.instance_id
    }

    #[must_use]
    /// Returns the snapshot's selection readiness, copied unchanged.
    pub const fn selection_readiness(&self) -> ConfiguredProviderInstanceSelectionReadiness {
        self.selection_readiness
    }

    /// Iterates overlay rows in consumer ordinal order, then catalogue order.
    pub fn entries(&self) -> impl ExactSizeIterator<Item = &ModelPresentationOverlayEntry> {
        self.entries.iter()
    }
}

/// Projects overlay markers onto one bound catalogue result.
///
/// Markers must key to this instance and to exact catalogue identity.
/// Rows that report a provider id match that provider id plus model id.
/// Rows that omit a provider id match instance plus model and must not
/// invent a provider id. Unknown models and cross-instance markers fail
/// closed. The snapshot's `Ready` / `NotReady` value is copied, never
/// rewritten.
///
/// Pass only this instance's markers. An unfiltered store list that includes
/// another instance fails as [`ModelPresentationOverlayFailureKind::CrossInstance`].
/// [`apply_stored_model_presentation_overlay`] filters the store first.
pub fn apply_model_presentation_overlay(
    record: &ConfiguredProviderInstanceRecord,
    markers: &[OverlayMarker],
) -> Result<ModelPresentationOverlay, ModelPresentationOverlayFailure> {
    let catalogue_keys = catalogue_identity_set(record);
    let mut by_identity = BTreeMap::new();
    for marker in markers {
        if marker.instance_id() != record.instance_id() {
            return Err(ModelPresentationOverlayFailure::cross_instance());
        }
        let key = overlay_identity(marker.provider_id(), marker.model_id());
        if !catalogue_keys.contains(&key) {
            return Err(ModelPresentationOverlayFailure::unknown_model());
        }
        by_identity.insert(key, marker);
    }

    let mut entries: Vec<(usize, ModelPresentationOverlayEntry)> = record
        .model_catalogue()
        .into_iter()
        .flat_map(|catalogue| catalogue.entries())
        .enumerate()
        .map(|(index, entry)| {
            let marker = by_identity
                .get(&overlay_identity(entry.provider_id(), entry.id()))
                .copied();
            (
                index,
                ModelPresentationOverlayEntry::from_catalogue(entry, marker),
            )
        })
        .collect();
    entries.sort_by_key(|(index, entry)| (entry.ordinal().unwrap_or(u32::MAX), *index));

    Ok(ModelPresentationOverlay {
        instance_id: record.instance_id().clone(),
        selection_readiness: record.selection_readiness(),
        entries: entries.into_iter().map(|(_, entry)| entry).collect(),
    })
}

/// Projects stored overlay markers for one bound catalogue result.
///
/// Markers for other configured instances are skipped so they are not copied
/// onto this catalogue. Same-instance unknown models still fail closed.
pub fn apply_stored_model_presentation_overlay(
    store: &dyn ConnectionLifecycleStore,
    record: &ConfiguredProviderInstanceRecord,
) -> Result<ModelPresentationOverlay, ModelPresentationOverlayFailure> {
    let markers = store
        .list_overlay_markers()
        .map_err(ModelPresentationOverlayFailure::from_store)?;
    let scoped: Vec<_> = markers
        .into_iter()
        .filter(|marker| marker.instance_id() == record.instance_id())
        .collect();
    apply_model_presentation_overlay(record, &scoped)
}

fn catalogue_identity_set(
    record: &ConfiguredProviderInstanceRecord,
) -> BTreeSet<(Option<ProviderId>, ModelId)> {
    record
        .model_catalogue()
        .into_iter()
        .flat_map(|catalogue| catalogue.entries())
        .map(|entry| overlay_identity(entry.provider_id(), entry.id()))
        .collect()
}

fn overlay_identity(
    provider_id: Option<&ProviderId>,
    model_id: &ModelId,
) -> (Option<ProviderId>, ModelId) {
    (provider_id.cloned(), model_id.clone())
}
