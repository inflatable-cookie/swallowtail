use std::collections::BTreeMap;
use swallowtail_core::{AddableRouteDescriptor, AddableRouteId, RouteTopology};

use super::AddableRouteCatalogFailure;

/// Consumer-assembled catalog of adapter-local addable-route descriptors.
///
/// Consumers insert the descriptors they linked, the same way they assemble
/// prepared facades. There is no umbrella registry and no runtime inventory of
/// every production route. Absence of a descriptor means that adapter was not
/// linked. Discovery candidates are not catalog rows.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AddableRouteCatalog {
    routes: BTreeMap<AddableRouteId, AddableRouteDescriptor>,
}

impl AddableRouteCatalog {
    /// Creates an empty catalog.
    #[must_use]
    pub fn new() -> Self {
        Self {
            routes: BTreeMap::new(),
        }
    }

    /// Assembles a catalog from adapter-local descriptors.
    pub fn from_descriptors(
        descriptors: impl IntoIterator<Item = AddableRouteDescriptor>,
    ) -> Result<Self, AddableRouteCatalogFailure> {
        let mut catalog = Self::new();
        for descriptor in descriptors {
            catalog = catalog.with_descriptor(descriptor)?;
        }
        Ok(catalog)
    }

    /// Inserts one adapter-local descriptor.
    pub fn with_descriptor(
        mut self,
        descriptor: AddableRouteDescriptor,
    ) -> Result<Self, AddableRouteCatalogFailure> {
        if self.routes.contains_key(descriptor.id()) {
            return Err(AddableRouteCatalogFailure::duplicate_route());
        }
        self.routes.insert(descriptor.id().clone(), descriptor);
        Ok(self)
    }

    #[must_use]
    /// Returns the descriptor with this addable-route id, when the consumer linked it.
    pub fn get(&self, id: &AddableRouteId) -> Option<&AddableRouteDescriptor> {
        self.routes.get(id)
    }

    /// Iterates descriptors in stable addable-route id order.
    pub fn routes(&self) -> impl ExactSizeIterator<Item = &AddableRouteDescriptor> {
        self.routes.values()
    }

    /// Iterates descriptors in one hosted, installed, or local-runtime group.
    pub fn routes_with_topology(
        &self,
        topology: RouteTopology,
    ) -> impl Iterator<Item = &AddableRouteDescriptor> {
        self.routes
            .values()
            .filter(move |descriptor| descriptor.topology() == topology)
    }
}
