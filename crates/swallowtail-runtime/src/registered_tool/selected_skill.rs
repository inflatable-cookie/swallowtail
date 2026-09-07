//! Immutable selected skill bundles and their declared required references.
//!
//! Contract 062 keeps discovery and description authority. This module carries
//! one *already selected* bundle: the consumer names the skill, its revision,
//! its content digest, and every required reference as an opaque host
//! reference. Swallowtail validates identity, bounds, digests, and lifecycle.
//! It discovers no repository, decides no relevance, generates no product
//! prompt, and never claims the provider followed the skill.

use super::failure::{RegisteredToolFailure, RegisteredToolFailureKind, reject};
use super::identity::{
    RequiredReferenceId, SelectedContentDigest, SelectedSkillId, SelectedSkillRevision,
};
use super::limits::{MAX_SELECTED_CONTENT_REFERENCE_BYTES, SelectedSkillBundleBounds};
use super::schema::RegisteredToolSchemaMediaType;
use crate::host_reference::SelectedContentRef;
use std::collections::BTreeSet;
use std::fmt;

/// Contract 062 source authority the consumer selected this skill from.
///
/// Provenance is descriptive. It is carried by the selection, never derived by
/// Swallowtail, and grants no traversal, scan, or root authority.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SelectedSkillProvenance {
    /// An explicitly host-approved global root.
    HostApprovedGlobal,
    /// A root bound to the exact admitted working resource.
    ProjectBound,
    /// A harness-distribution root resolved for one exact configured instance.
    HarnessDistribution,
}

impl SelectedSkillProvenance {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostApprovedGlobal => "host-approved-global",
            Self::ProjectBound => "project-bound",
            Self::HarnessDistribution => "harness-distribution",
        }
    }
}

impl fmt::Display for SelectedSkillProvenance {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// One opaque host reference plus the exact bound its content must respect.
///
/// The reference is a capability, not a path. It is bounded to Contract 062's
/// 512-byte opaque-reference maximum at construction, and it never enters
/// `Debug`, diagnostics, failures, or projected rows.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedContentDescriptor {
    reference: SelectedContentRef,
    media_type: RegisteredToolSchemaMediaType,
    declared_bytes: usize,
}

impl SelectedContentDescriptor {
    /// Binds one opaque reference to its media type and positive byte bound.
    /// The rejection carries only its safe failure class: an over-long or
    /// otherwise rejected reference value is never echoed back to the caller.
    pub fn new(
        reference: SelectedContentRef,
        media_type: RegisteredToolSchemaMediaType,
        declared_bytes: usize,
    ) -> Result<Self, RegisteredToolFailure> {
        if declared_bytes == 0 {
            return Err(reject(RegisteredToolFailureKind::IdentityRejected));
        }
        if reference.as_host_value().len() > MAX_SELECTED_CONTENT_REFERENCE_BYTES
            || declared_bytes > SelectedSkillBundleBounds::ceiling().max_aggregate_content_bytes()
        {
            return Err(reject(RegisteredToolFailureKind::LimitExceeded));
        }
        Ok(Self {
            reference,
            media_type,
            declared_bytes,
        })
    }

    /// Returns the opaque host reference for host resolution only.
    #[must_use]
    pub const fn reference(&self) -> &SelectedContentRef {
        &self.reference
    }

    /// Returns the exact declared media type.
    #[must_use]
    pub const fn media_type(&self) -> &RegisteredToolSchemaMediaType {
        &self.media_type
    }

    /// Returns the positive consumer-declared byte bound.
    #[must_use]
    pub const fn declared_bytes(&self) -> usize {
        self.declared_bytes
    }
}

/// Stable identity, provenance, and body descriptor of one selected skill.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedSkillIdentity {
    id: SelectedSkillId,
    provenance: SelectedSkillProvenance,
    body: SelectedContentDescriptor,
}

impl SelectedSkillIdentity {
    /// Binds one skill id and provenance to its bounded body descriptor.
    #[must_use]
    pub const fn new(
        id: SelectedSkillId,
        provenance: SelectedSkillProvenance,
        body: SelectedContentDescriptor,
    ) -> Self {
        Self {
            id,
            provenance,
            body,
        }
    }

    /// Returns the stable selected-skill id.
    #[must_use]
    pub const fn id(&self) -> &SelectedSkillId {
        &self.id
    }

    /// Returns the Contract 062 source authority the consumer selected from.
    #[must_use]
    pub const fn provenance(&self) -> SelectedSkillProvenance {
        self.provenance
    }

    /// Returns the bounded body descriptor.
    #[must_use]
    pub const fn body(&self) -> &SelectedContentDescriptor {
        &self.body
    }
}

/// One required reference a selected skill declares.
///
/// Every declared reference must resolve before provider work. A missing,
/// changed, oversized, inaccessible, or foreign reference is a typed failure,
/// never a silently dropped input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequiredReferenceDescriptor {
    id: RequiredReferenceId,
    digest: SelectedContentDigest,
    content: SelectedContentDescriptor,
}

impl RequiredReferenceDescriptor {
    /// Binds one reference id and declared digest to its bounded descriptor.
    #[must_use]
    pub const fn new(
        id: RequiredReferenceId,
        digest: SelectedContentDigest,
        content: SelectedContentDescriptor,
    ) -> Self {
        Self {
            id,
            digest,
            content,
        }
    }

    /// Returns the stable reference id.
    #[must_use]
    pub const fn id(&self) -> &RequiredReferenceId {
        &self.id
    }

    /// Returns the consumer-declared content digest.
    #[must_use]
    pub const fn digest(&self) -> &SelectedContentDigest {
        &self.digest
    }

    /// Returns the bounded content descriptor.
    #[must_use]
    pub const fn content(&self) -> &SelectedContentDescriptor {
        &self.content
    }
}

/// Immutable selected skill bundle that owns no runtime resource.
///
/// A bundle describes what the consumer selected. It resolves nothing on its
/// own: [`SelectedSkillBundle::resolve`] validates host-supplied content
/// against these declarations before any provider dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedSkillBundle {
    identity: SelectedSkillIdentity,
    revision: SelectedSkillRevision,
    digest: SelectedContentDigest,
    references: Vec<RequiredReferenceDescriptor>,
    bounds: SelectedSkillBundleBounds,
}

impl SelectedSkillBundle {
    /// Admits one immutable bundle within its positive declared bounds.
    ///
    /// Duplicate reference ids, an over-count, and a declared aggregate beyond
    /// the bundle bound all fail here, before any host or provider work.
    pub fn new(
        identity: SelectedSkillIdentity,
        revision: SelectedSkillRevision,
        digest: SelectedContentDigest,
        references: impl IntoIterator<Item = RequiredReferenceDescriptor>,
        bounds: SelectedSkillBundleBounds,
    ) -> Result<Self, RegisteredToolFailure> {
        let references: Vec<_> = references.into_iter().collect();
        if references.len() > bounds.max_required_references() {
            return Err(reject(RegisteredToolFailureKind::LimitExceeded));
        }
        let mut seen: BTreeSet<&RequiredReferenceId> = BTreeSet::new();
        for reference in &references {
            if !seen.insert(reference.id()) {
                return Err(reject(RegisteredToolFailureKind::IdentityRejected));
            }
        }
        let declared_bytes = references
            .iter()
            .fold(identity.body().declared_bytes(), |total, reference| {
                total.saturating_add(reference.content().declared_bytes())
            });
        if declared_bytes > bounds.max_aggregate_content_bytes() {
            return Err(reject(RegisteredToolFailureKind::LimitExceeded));
        }
        Ok(Self {
            identity,
            revision,
            digest,
            references,
            bounds,
        })
    }

    /// Returns the stable skill identity, provenance, and body descriptor.
    #[must_use]
    pub const fn identity(&self) -> &SelectedSkillIdentity {
        &self.identity
    }

    /// Returns the exact bundle revision.
    #[must_use]
    pub const fn revision(&self) -> &SelectedSkillRevision {
        &self.revision
    }

    /// Returns the consumer-declared digest of the skill body.
    #[must_use]
    pub const fn digest(&self) -> &SelectedContentDigest {
        &self.digest
    }

    /// Returns every declared required reference in selection order.
    #[must_use]
    pub fn references(&self) -> &[RequiredReferenceDescriptor] {
        &self.references
    }

    /// Returns the declared reference for one exact reference id.
    #[must_use]
    pub fn reference(&self, id: &RequiredReferenceId) -> Option<&RequiredReferenceDescriptor> {
        self.references
            .iter()
            .find(|reference| reference.id() == id)
    }

    /// Returns the positive bounds this bundle was admitted under.
    #[must_use]
    pub const fn bounds(&self) -> SelectedSkillBundleBounds {
        self.bounds
    }

    /// Returns the aggregate byte bound the consumer declared.
    #[must_use]
    pub fn declared_content_bytes(&self) -> usize {
        self.references
            .iter()
            .fold(self.identity.body().declared_bytes(), |total, reference| {
                total.saturating_add(reference.content().declared_bytes())
            })
    }
}
