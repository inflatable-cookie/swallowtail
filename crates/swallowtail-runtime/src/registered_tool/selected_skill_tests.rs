//! Provider-free conformance for bounded selected skill/reference transport.

use super::*;
use crate::host_reference::SelectedContentRef;

const SKILL_BODY: &[u8] = b"# fixture skill\nsecret-skill-body\n";
const REFERENCE_BODY: &[u8] = b"secret-reference-body";

fn media_type() -> RegisteredToolSchemaMediaType {
    RegisteredToolSchemaMediaType::new("text/markdown").expect("media type")
}

fn other_media_type() -> RegisteredToolSchemaMediaType {
    RegisteredToolSchemaMediaType::new("application/json").expect("media type")
}

fn content_ref(label: &str) -> SelectedContentRef {
    SelectedContentRef::new(label).expect("selected content reference")
}

fn descriptor(label: &str, declared_bytes: usize) -> SelectedContentDescriptor {
    SelectedContentDescriptor::new(content_ref(label), media_type(), declared_bytes)
        .expect("content descriptor")
}

fn reference_id(label: &str) -> RequiredReferenceId {
    RequiredReferenceId::new(label).expect("reference id")
}

fn payload(body: &[u8]) -> RegisteredToolPayload {
    RegisteredToolPayload::new(
        media_type(),
        body.to_vec(),
        MAX_SELECTED_SKILL_CONTENT_BYTES,
    )
    .expect("payload")
}

fn resolved(body: &[u8]) -> SelectedContentResolution {
    SelectedContentResolution::Resolved(payload(body))
}

fn skill_identity() -> SelectedSkillIdentity {
    SelectedSkillIdentity::new(
        SelectedSkillId::new("desktop.skill.review").expect("skill id"),
        SelectedSkillProvenance::ProjectBound,
        descriptor("host://skill", SKILL_BODY.len()),
    )
}

fn required_reference() -> RequiredReferenceDescriptor {
    RequiredReferenceDescriptor::new(
        reference_id("checklist"),
        SelectedContentDigest::of_bytes(REFERENCE_BODY),
        descriptor("host://reference", REFERENCE_BODY.len()),
    )
}

fn bundle() -> SelectedSkillBundle {
    SelectedSkillBundle::new(
        skill_identity(),
        SelectedSkillRevision::new("7").expect("revision"),
        SelectedContentDigest::of_bytes(SKILL_BODY),
        [required_reference()],
        SelectedSkillBundleBounds::ceiling(),
    )
    .expect("bundle")
}

fn host_resources() -> SelectedSkillHostResources {
    SelectedSkillHostResources::new()
        .with_skill_body(resolved(SKILL_BODY))
        .with_reference(reference_id("checklist"), resolved(REFERENCE_BODY))
}

fn resolve_failure(host_resources: &SelectedSkillHostResources) -> String {
    bundle()
        .resolve(host_resources)
        .expect_err("resolution fails closed")
        .diagnostic()
        .code()
        .to_owned()
}

#[test]
fn a_bundle_resolves_only_when_every_declared_body_agrees() {
    let resolved = bundle()
        .resolve(&host_resources())
        .expect("bundle resolves");

    assert_eq!(resolved.identity().id().as_str(), "desktop.skill.review");
    assert_eq!(resolved.revision().as_str(), "7");
    assert_eq!(
        resolved.identity().provenance(),
        SelectedSkillProvenance::ProjectBound
    );
    assert_eq!(resolved.references().len(), 1);
    assert_eq!(resolved.references()[0].id().as_str(), "checklist");
    assert_eq!(
        resolved.resolved_content_bytes(),
        SKILL_BODY.len() + REFERENCE_BODY.len()
    );
    assert_eq!(resolved.body().expose_for_execution(), SKILL_BODY);
}

#[test]
fn a_missing_required_reference_fails_before_provider_work() {
    let resources = SelectedSkillHostResources::new().with_skill_body(resolved(SKILL_BODY));

    assert_eq!(
        resolve_failure(&resources),
        RegisteredToolFailureKind::RequiredReferenceUnavailable.code()
    );
}

#[test]
fn an_absent_skill_body_fails_before_provider_work() {
    let resources = SelectedSkillHostResources::new()
        .with_reference(reference_id("checklist"), resolved(REFERENCE_BODY));

    assert_eq!(
        resolve_failure(&resources),
        RegisteredToolFailureKind::RequiredReferenceUnavailable.code()
    );
}

#[test]
fn an_inaccessible_reference_is_distinct_from_a_silent_drop() {
    let resources = host_resources().with_reference(
        reference_id("checklist"),
        SelectedContentResolution::Inaccessible,
    );

    assert_eq!(
        resolve_failure(&resources),
        RegisteredToolFailureKind::RequiredReferenceUnavailable.code()
    );
}

#[test]
fn changed_content_is_detected_by_the_digest_swallowtail_computes() {
    let resources =
        host_resources().with_reference(reference_id("checklist"), resolved(b"changed-reference"));

    assert_eq!(
        resolve_failure(&resources),
        RegisteredToolFailureKind::SelectedContentMismatch.code()
    );
}

#[test]
fn a_changed_skill_body_is_detected_by_the_same_digest_check() {
    let resources = host_resources().with_skill_body(resolved(b"changed-skill-body"));

    assert_eq!(
        resolve_failure(&resources),
        RegisteredToolFailureKind::SelectedContentMismatch.code()
    );
}

#[test]
fn a_substituted_media_type_fails_even_when_the_bytes_match() {
    let substituted = RegisteredToolPayload::new(
        other_media_type(),
        REFERENCE_BODY.to_vec(),
        MAX_SELECTED_SKILL_CONTENT_BYTES,
    )
    .expect("payload");
    let resources = host_resources().with_reference(
        reference_id("checklist"),
        SelectedContentResolution::Resolved(substituted),
    );

    assert_eq!(
        resolve_failure(&resources),
        RegisteredToolFailureKind::SelectedContentMismatch.code()
    );
}

#[test]
fn oversized_content_is_rejected_against_its_declared_bound() {
    let oversized = [REFERENCE_BODY, b"-and-more"].concat();
    let resources =
        host_resources().with_reference(reference_id("checklist"), resolved(oversized.as_slice()));

    assert_eq!(
        resolve_failure(&resources),
        RegisteredToolFailureKind::LimitExceeded.code()
    );
}

#[test]
fn content_the_bundle_never_declared_is_rejected_as_foreign() {
    let resources =
        host_resources().with_reference(reference_id("unrelated"), resolved(b"unrelated-body"));

    assert_eq!(
        resolve_failure(&resources),
        RegisteredToolFailureKind::ForeignSelectedContent.code()
    );
}

#[test]
fn duplicate_reference_identities_reject_before_any_resolution() {
    let failure = SelectedSkillBundle::new(
        skill_identity(),
        SelectedSkillRevision::new("7").expect("revision"),
        SelectedContentDigest::of_bytes(SKILL_BODY),
        [required_reference(), required_reference()],
        SelectedSkillBundleBounds::ceiling(),
    )
    .expect_err("duplicate ids reject");

    assert_eq!(failure.kind(), RegisteredToolFailureKind::IdentityRejected);
}

#[test]
fn the_declared_aggregate_is_enforced_before_any_resolution() {
    let bounds = SelectedSkillBundleBounds::new(4, SKILL_BODY.len()).expect("bounds");

    let failure = SelectedSkillBundle::new(
        skill_identity(),
        SelectedSkillRevision::new("7").expect("revision"),
        SelectedContentDigest::of_bytes(SKILL_BODY),
        [required_reference()],
        bounds,
    )
    .expect_err("declared aggregate is rejected");

    assert_eq!(failure.kind(), RegisteredToolFailureKind::LimitExceeded);
}

#[test]
fn more_references_than_the_bundle_bound_admits_are_rejected() {
    let bounds =
        SelectedSkillBundleBounds::new(0, MAX_SELECTED_SKILL_CONTENT_BYTES).expect("bounds");

    let failure = SelectedSkillBundle::new(
        skill_identity(),
        SelectedSkillRevision::new("7").expect("revision"),
        SelectedContentDigest::of_bytes(SKILL_BODY),
        [required_reference()],
        bounds,
    )
    .expect_err("reference count is rejected");

    assert_eq!(failure.kind(), RegisteredToolFailureKind::LimitExceeded);
}

#[test]
fn bundle_bounds_never_widen_the_first_tranche_ceiling() {
    assert_eq!(MAX_SELECTED_SKILL_CONTENT_BYTES, 64 * 1024);
    assert_eq!(MAX_SELECTED_SKILL_REQUIRED_REFERENCES, 32);
    assert!(
        SelectedSkillBundleBounds::new(
            MAX_SELECTED_SKILL_REQUIRED_REFERENCES + 1,
            MAX_SELECTED_SKILL_CONTENT_BYTES
        )
        .is_err()
    );
    assert!(SelectedSkillBundleBounds::new(1, MAX_SELECTED_SKILL_CONTENT_BYTES + 1).is_err());
    assert!(SelectedSkillBundleBounds::new(1, 0).is_err());
    assert!(
        SelectedContentDescriptor::new(content_ref("host://x"), media_type(), 0).is_err(),
        "every declared body bound must be positive"
    );
    assert!(
        SelectedContentDescriptor::new(
            content_ref("host://x"),
            media_type(),
            MAX_SELECTED_SKILL_CONTENT_BYTES + 1
        )
        .is_err()
    );
}

#[test]
fn selected_bodies_and_opaque_references_never_reach_formatting() {
    let resolved = bundle()
        .resolve(&host_resources())
        .expect("bundle resolves");
    let rendered = format!("{resolved:?}");

    assert!(!rendered.contains("secret-skill-body"));
    assert!(!rendered.contains("secret-reference-body"));
    assert!(!rendered.contains("host://skill"));
    assert!(!rendered.contains("host://reference"));
    assert!(!format!("{:?}", bundle()).contains("host://skill"));
    assert_eq!(
        format!("{}", resolved.body()),
        "<redacted registered tool payload>"
    );
}

#[test]
fn the_canonical_digest_is_stable_and_bound_to_the_exact_bytes() {
    let digest = SelectedContentDigest::of_bytes(SKILL_BODY);

    assert_eq!(digest, SelectedContentDigest::of_bytes(SKILL_BODY));
    assert_ne!(digest, SelectedContentDigest::of_bytes(b"other"));
    assert!(digest.as_str().starts_with("sha256:"));
    assert_eq!(digest.as_str().len(), "sha256:".len() + 64);
}

#[test]
fn provenance_keeps_the_three_contract_062_source_authorities() {
    assert_eq!(
        SelectedSkillProvenance::HostApprovedGlobal.as_str(),
        "host-approved-global"
    );
    assert_eq!(
        SelectedSkillProvenance::ProjectBound.as_str(),
        "project-bound"
    );
    assert_eq!(
        SelectedSkillProvenance::HarnessDistribution.as_str(),
        "harness-distribution"
    );
}
