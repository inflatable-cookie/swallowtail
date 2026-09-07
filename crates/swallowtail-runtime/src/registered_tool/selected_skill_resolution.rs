//! Host-supplied selected content and its fail-closed bundle resolution.
//!
//! The execution host owns approval, containment, and the bounded read behind
//! each opaque reference. It reports the bounded body it read here; Swallowtail
//! recomputes the canonical digest over exactly those bytes and rejects any
//! disagreement with the immutable selection. The host never supplies the
//! digest its own read is checked against.

use super::failure::{RegisteredToolFailureKind, fail};
use super::identity::{RequiredReferenceId, SelectedContentDigest, SelectedSkillRevision};
use super::payload::RegisteredToolPayload;
use super::selected_skill::{
    RequiredReferenceDescriptor, SelectedContentDescriptor, SelectedSkillBundle,
    SelectedSkillIdentity,
};
use crate::RuntimeFailure;
use std::collections::BTreeMap;

/// What the host could report for one declared opaque reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectedContentResolution {
    /// The host read exactly this bounded body behind the opaque reference.
    Resolved(RegisteredToolPayload),
    /// The host could not read the content it was asked to resolve.
    ///
    /// This is distinct from absence: the host reached the reference and
    /// refused or failed, so the bundle fails closed rather than silently
    /// dropping a required input.
    Inaccessible,
}

/// Bounded host-resolved content offered for one exact bundle resolution.
///
/// The map is host-supplied evidence, not authority. Anything it carries that
/// the bundle never declared is rejected as foreign content.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SelectedSkillHostResources {
    body: Option<SelectedContentResolution>,
    references: BTreeMap<RequiredReferenceId, SelectedContentResolution>,
}

impl SelectedSkillHostResources {
    /// Creates an empty set of host-resolved content.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records what the host resolved for the selected skill body.
    #[must_use]
    pub fn with_skill_body(mut self, resolution: SelectedContentResolution) -> Self {
        self.body = Some(resolution);
        self
    }

    /// Records what the host resolved for one declared reference id.
    #[must_use]
    pub fn with_reference(
        mut self,
        id: RequiredReferenceId,
        resolution: SelectedContentResolution,
    ) -> Self {
        self.references.insert(id, resolution);
        self
    }

    /// Returns what the host reported for the skill body, if anything.
    #[must_use]
    pub const fn skill_body(&self) -> Option<&SelectedContentResolution> {
        self.body.as_ref()
    }

    /// Returns what the host reported for one reference id, if anything.
    #[must_use]
    pub fn reference(&self, id: &RequiredReferenceId) -> Option<&SelectedContentResolution> {
        self.references.get(id)
    }
}

/// One resolved required reference bound to its declared descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedRequiredReference {
    id: RequiredReferenceId,
    digest: SelectedContentDigest,
    payload: RegisteredToolPayload,
}

impl ResolvedRequiredReference {
    /// Returns the stable reference id.
    #[must_use]
    pub const fn id(&self) -> &RequiredReferenceId {
        &self.id
    }

    /// Returns the declared digest Swallowtail recomputed over the body.
    #[must_use]
    pub const fn digest(&self) -> &SelectedContentDigest {
        &self.digest
    }

    /// Returns the bounded resolved body.
    #[must_use]
    pub const fn payload(&self) -> &RegisteredToolPayload {
        &self.payload
    }
}

/// Immutable bundle whose body and every required reference resolved.
///
/// A resolved bundle proves bounded, digest-verified transport. It never
/// claims the provider read, understood, or followed the skill.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedSkillBundle {
    identity: SelectedSkillIdentity,
    revision: SelectedSkillRevision,
    digest: SelectedContentDigest,
    body: RegisteredToolPayload,
    references: Vec<ResolvedRequiredReference>,
    resolved_content_bytes: usize,
}

impl ResolvedSkillBundle {
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

    /// Returns the declared digest Swallowtail recomputed over the body.
    #[must_use]
    pub const fn digest(&self) -> &SelectedContentDigest {
        &self.digest
    }

    /// Returns the bounded resolved skill body.
    #[must_use]
    pub const fn body(&self) -> &RegisteredToolPayload {
        &self.body
    }

    /// Returns every resolved required reference in declaration order.
    #[must_use]
    pub fn references(&self) -> &[ResolvedRequiredReference] {
        &self.references
    }

    /// Returns the exact resolved aggregate byte length.
    #[must_use]
    pub const fn resolved_content_bytes(&self) -> usize {
        self.resolved_content_bytes
    }
}

impl SelectedSkillBundle {
    /// Validates host-resolved content against this immutable selection.
    ///
    /// Every declared reference must be present, accessible, digest-agreeing,
    /// and within its positive declared bound; the aggregate must stay within
    /// the bundle bound; and content the bundle never declared is rejected.
    /// Resolution reads nothing and dispatches nothing.
    pub fn resolve(
        &self,
        host_resources: &SelectedSkillHostResources,
    ) -> Result<ResolvedSkillBundle, RuntimeFailure> {
        reject_foreign_references(self, host_resources)?;
        let body = admit_content(
            host_resources.skill_body(),
            self.digest(),
            self.identity().body(),
        )?;
        let mut resolved_content_bytes = body.byte_len();
        let mut references = Vec::with_capacity(self.references().len());
        for declaration in self.references() {
            let payload = admit_reference(host_resources, declaration)?;
            resolved_content_bytes = resolved_content_bytes.saturating_add(payload.byte_len());
            references.push(ResolvedRequiredReference {
                id: declaration.id().clone(),
                digest: declaration.digest().clone(),
                payload,
            });
        }
        if resolved_content_bytes > self.bounds().max_aggregate_content_bytes() {
            return Err(fail(RegisteredToolFailureKind::LimitExceeded));
        }
        Ok(ResolvedSkillBundle {
            identity: self.identity().clone(),
            revision: self.revision().clone(),
            digest: self.digest().clone(),
            body,
            references,
            resolved_content_bytes,
        })
    }
}

fn reject_foreign_references(
    bundle: &SelectedSkillBundle,
    host_resources: &SelectedSkillHostResources,
) -> Result<(), RuntimeFailure> {
    if host_resources
        .references
        .keys()
        .any(|id| bundle.reference(id).is_none())
    {
        return Err(fail(RegisteredToolFailureKind::ForeignSelectedContent));
    }
    Ok(())
}

fn admit_reference(
    host_resources: &SelectedSkillHostResources,
    declaration: &RequiredReferenceDescriptor,
) -> Result<RegisteredToolPayload, RuntimeFailure> {
    admit_content(
        host_resources.reference(declaration.id()),
        declaration.digest(),
        declaration.content(),
    )
}

fn admit_content(
    resolution: Option<&SelectedContentResolution>,
    declared_digest: &SelectedContentDigest,
    descriptor: &SelectedContentDescriptor,
) -> Result<RegisteredToolPayload, RuntimeFailure> {
    let Some(SelectedContentResolution::Resolved(payload)) = resolution else {
        return Err(fail(
            RegisteredToolFailureKind::RequiredReferenceUnavailable,
        ));
    };
    if payload.byte_len() > descriptor.declared_bytes() {
        return Err(fail(RegisteredToolFailureKind::LimitExceeded));
    }
    if payload.media_type() != descriptor.media_type()
        || &SelectedContentDigest::of_bytes(payload.expose_for_execution()) != declared_digest
    {
        return Err(fail(RegisteredToolFailureKind::SelectedContentMismatch));
    }
    Ok(payload.clone())
}
