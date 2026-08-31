//! Portable semantic vocabulary shared by every Contract 061 projection row.

mod authority;
mod extension;
mod identity;
mod posture;
mod reason;

pub use authority::{
    ConsumerRouteEvidenceStrength, ConsumerRouteMutationAuthority, ConsumerRouteSourceClass,
    ConsumerRouteStateSupport,
};
pub use extension::ConsumerRouteNamespacedExtension;
pub use identity::{ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteRowIdentity};
pub use posture::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteAvailabilityDimension,
    ConsumerRouteLifecycle, ConsumerRouteSupportPosture,
};
pub use reason::ConsumerRouteSafeReason;
