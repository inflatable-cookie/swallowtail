use super::{
    ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass, ActivityKindProfile,
    ActivityLifecycleFidelity, ActivityUnknownEventPosture, InvalidObservableActivityProfile,
    ObservableActivityAvailability,
};
use crate::{
    Capability, CapabilityConstraint, CapabilityRequirement, InterfaceBehaviorRevision,
    InterfaceVersionAxis,
};
use std::collections::{BTreeMap, BTreeSet};

/// Immutable semantic fidelity advertised by one prepared route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservableActivityProfile {
    availability: ObservableActivityAvailability,
    interface_basis: BTreeMap<InterfaceVersionAxis, InterfaceBehaviorRevision>,
    kinds: BTreeMap<ActivityKindClass, ActivityKindProfile>,
    unknown_event_posture: ActivityUnknownEventPosture,
}

impl ObservableActivityProfile {
    #[must_use]
    pub fn not_applicable() -> Self {
        Self {
            availability: ObservableActivityAvailability::NotApplicable,
            interface_basis: BTreeMap::new(),
            kinds: BTreeMap::new(),
            unknown_event_posture: ActivityUnknownEventPosture::FailClosed,
        }
    }

    pub fn unavailable(
        interface_basis: impl IntoIterator<Item = ActivityInterfaceBasis>,
    ) -> Result<Self, InvalidObservableActivityProfile> {
        Ok(Self {
            availability: ObservableActivityAvailability::Unavailable,
            interface_basis: collect_interface_basis(interface_basis)?,
            kinds: BTreeMap::new(),
            unknown_event_posture: ActivityUnknownEventPosture::FailClosed,
        })
    }

    pub fn available(
        interface_basis: impl IntoIterator<Item = ActivityInterfaceBasis>,
        kinds: impl IntoIterator<Item = ActivityKindProfile>,
        unknown_event_posture: ActivityUnknownEventPosture,
    ) -> Result<Self, InvalidObservableActivityProfile> {
        let mut indexed_kinds = BTreeMap::new();
        for profile in kinds {
            let kind = profile.kind();
            if indexed_kinds.insert(kind, profile).is_some() {
                return Err(InvalidObservableActivityProfile::new(
                    "Activity route profile contains a duplicate activity kind",
                ));
            }
        }
        if indexed_kinds.is_empty() {
            return Err(InvalidObservableActivityProfile::new(
                "Available activity route profile requires at least one activity kind",
            ));
        }
        let has_unknown_kind = indexed_kinds.contains_key(&ActivityKindClass::Unknown);
        let preserves_unknown =
            unknown_event_posture == ActivityUnknownEventPosture::PreserveNamespaced;
        if has_unknown_kind != preserves_unknown {
            return Err(InvalidObservableActivityProfile::new(
                "Unknown activity kind and preservation posture must be declared together",
            ));
        }
        Ok(Self {
            availability: ObservableActivityAvailability::Available,
            interface_basis: collect_interface_basis(interface_basis)?,
            kinds: indexed_kinds,
            unknown_event_posture,
        })
    }

    #[must_use]
    pub const fn availability(&self) -> ObservableActivityAvailability {
        self.availability
    }

    pub fn interface_basis(&self) -> impl ExactSizeIterator<Item = ActivityInterfaceBasis> + '_ {
        self.interface_basis
            .iter()
            .map(|(axis, revision)| ActivityInterfaceBasis::new(axis.clone(), revision.clone()))
    }

    pub fn kinds(&self) -> impl ExactSizeIterator<Item = &ActivityKindProfile> {
        self.kinds.values()
    }

    #[must_use]
    pub fn kind(&self, kind: ActivityKindClass) -> Option<&ActivityKindProfile> {
        self.kinds.get(&kind)
    }

    #[must_use]
    pub fn lifecycle(&self, kind: ActivityKindClass) -> ActivityLifecycleFidelity {
        self.kind(kind).map_or(
            ActivityLifecycleFidelity::Unavailable,
            ActivityKindProfile::lifecycle,
        )
    }

    #[must_use]
    pub fn disclosure(&self, kind: ActivityKindClass) -> ActivityDisclosure {
        self.kind(kind).map_or(
            ActivityDisclosure::Unavailable,
            ActivityKindProfile::disclosure,
        )
    }

    #[must_use]
    pub const fn unknown_event_posture(&self) -> ActivityUnknownEventPosture {
        self.unknown_event_posture
    }

    #[must_use]
    pub fn capability_requirement(&self) -> Option<CapabilityRequirement> {
        if self.availability != ObservableActivityAvailability::Available {
            return None;
        }
        let mut constraints =
            BTreeSet::from(
                [CapabilityConstraint::ObservableActivityUnknownEventPosture(
                    self.unknown_event_posture,
                )],
            );
        for profile in self.kinds.values() {
            constraints.extend(profile.capability_constraints());
        }
        Some(CapabilityRequirement::new(
            Capability::ObservableActivity,
            constraints,
        ))
    }

    #[must_use]
    pub fn supports(&self, requirement: &CapabilityRequirement) -> bool {
        requirement.capability() == Capability::ObservableActivity
            && self.capability_requirement().is_some_and(|available| {
                let supported = available.constraints().collect::<BTreeSet<_>>();
                requirement
                    .constraints()
                    .all(|required| supported.contains(required))
            })
    }
}

fn collect_interface_basis(
    values: impl IntoIterator<Item = ActivityInterfaceBasis>,
) -> Result<
    BTreeMap<InterfaceVersionAxis, InterfaceBehaviorRevision>,
    InvalidObservableActivityProfile,
> {
    let mut basis = BTreeMap::new();
    for value in values {
        let (axis, revision) = value.into_parts();
        if basis.insert(axis, revision).is_some() {
            return Err(InvalidObservableActivityProfile::new(
                "Activity route profile contains a duplicate interface axis",
            ));
        }
    }
    Ok(basis)
}
