#![deny(missing_docs)]

//! Contract 061 consumer route feature and control projection.
//!
//! The projection composes evidence that already exists. It authorizes no
//! operation, chooses no route or model, invents no default, mutates no
//! session, and owns no consumer layout, localization, or routing policy.

mod acknowledgement;
mod admission;
mod agreement;
mod applicability;
mod compose;
mod contribution;
mod failure;
mod identity;
mod model_binding;
mod provider_operation_observation;
mod row;
mod semantics;
mod text;
mod value;
mod view;
mod views;

pub use acknowledgement::{
    ConsumerRouteAcknowledgementState, ConsumerRouteCompoundAcknowledgement,
};
pub use applicability::ConsumerRouteApplicability;
pub use compose::{ConsumerRouteProjectionInput, compose_consumer_route_projection};
pub use contribution::ConsumerRouteProjectionContribution;
pub use failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind};
pub use identity::{
    ConsumerRouteProjectionSourceId, ConsumerRouteProjectionSourceIdentity,
    ConsumerRouteProjectionSourceKind,
};
pub use model_binding::ConsumerRouteModelBinding;
pub use provider_operation_observation::{
    ConsumerRouteProviderOperationObservation, ConsumerRouteProviderOperationOutcome,
};
pub use row::ConsumerRouteProjectionRow;
pub use semantics::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteAvailabilityDimension,
    ConsumerRouteControlId, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteRowIdentity, ConsumerRouteSafeReason, ConsumerRouteSourceClass,
    ConsumerRouteStateSupport, ConsumerRouteSupportPosture,
};
pub use value::{
    ConsumerRouteControlValue, ConsumerRouteEnumerableValue, ConsumerRouteEnumeratedValues,
    ConsumerRouteOmissionSemantics, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};
pub use views::{
    ConsumerRouteActiveSessionState, ConsumerRouteProjection, ConsumerRouteProjectionIdentity,
    ConsumerRouteProviderOperationState, ConsumerRouteSelectionSummary,
    ConsumerRouteSessionStartControls,
};

/// Maximum selection-summary rows in one exact route projection.
pub const MAX_CONSUMER_ROUTE_SELECTION_SUMMARY_ROWS: usize = 32;
/// Maximum session-start and per-turn control rows in one route projection.
pub const MAX_CONSUMER_ROUTE_SESSION_START_ROWS: usize = 16;
/// Maximum active-session rows in one exact route projection.
pub const MAX_CONSUMER_ROUTE_ACTIVE_SESSION_ROWS: usize = 8;
/// Maximum provider-operation observation rows in one exact route projection.
pub const MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS: usize = 4;
/// Maximum admitted values in one projected control domain.
pub const MAX_CONSUMER_ROUTE_ENUMERABLE_VALUES: usize = 512;
/// Maximum UTF-8 bytes in one projected control value.
pub const MAX_CONSUMER_ROUTE_ENUMERABLE_VALUE_BYTES: usize = 512;
/// Maximum bounded namespaced extensions in one route projection.
pub const MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS: usize = 16;
/// Maximum UTF-8 bytes in one namespaced extension text component.
pub const MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES: usize = 128;
/// Maximum source identities in one exact route projection.
pub const MAX_CONSUMER_ROUTE_SOURCE_IDENTITIES: usize = 16;
/// Maximum UTF-8 bytes in one projection source id.
pub const MAX_CONSUMER_ROUTE_SOURCE_ID_BYTES: usize = 128;
/// Maximum UTF-8 bytes in one bounded safe reason message.
pub const MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES: usize = 256;
