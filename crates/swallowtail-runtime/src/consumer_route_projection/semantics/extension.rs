use super::super::MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES;
use super::super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind};
use super::super::text::admit_text;

/// Bounded provider-native descriptor identity qualified by route and version.
///
/// The extension carries no raw provider payload, command, path, or credential
/// material, and never widens support, availability, or lifecycle.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConsumerRouteNamespacedExtension {
    route: String,
    version_segment: String,
    semantic_id: String,
}

impl ConsumerRouteNamespacedExtension {
    /// Admits one bounded route, qualified version segment, and semantic id.
    pub fn new(
        route: impl Into<String>,
        version_segment: impl Into<String>,
        semantic_id: impl Into<String>,
    ) -> Result<Self, ConsumerRouteProjectionFailure> {
        let route = route.into();
        let version_segment = version_segment.into();
        let semantic_id = semantic_id.into();
        for value in [&route, &version_segment, &semantic_id] {
            admit_text(
                value,
                MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES,
                ConsumerRouteProjectionFailureKind::LimitExceeded,
                "swallowtail.consumer_route_projection.extension_text_limit_exceeded",
                "Namespaced extension text exceeds the fixed extension byte maximum",
            )?;
        }
        Ok(Self {
            route,
            version_segment,
            semantic_id,
        })
    }

    #[must_use]
    /// Returns the exact route this extension belongs to.
    pub fn route(&self) -> &str {
        &self.route
    }

    #[must_use]
    /// Returns the qualified provider-interface version segment.
    pub fn version_segment(&self) -> &str {
        &self.version_segment
    }

    #[must_use]
    /// Returns the route-local semantic id.
    pub fn semantic_id(&self) -> &str {
        &self.semantic_id
    }
}
