//! Claude Agent SDK's additive selected-skill input binding.
//!
//! The SDK session profile remains `Copy`, like the Card 084 profile. A
//! resolved bundle owns bounded content, so it travels beside that profile in
//! this explicit binding rather than turning MCP or selected-skill content
//! into ambient profile state.

use super::profile::ClaudeAgentSdkSessionProfile;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    MAX_SELECTED_SKILL_CONTENT_BYTES, MAX_SELECTED_SKILL_REQUIRED_REFERENCES,
    RegisteredToolPayload, ResolvedSkillBundle, RuntimeFailure, SelectedContentDigest,
};

/// Copy profile plus one immutable, already-resolved selected-skill bundle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSdkSelectedSkillBinding {
    profile: ClaudeAgentSdkSessionProfile,
    bundle: ResolvedSkillBundle,
}

impl ClaudeAgentSdkSelectedSkillBinding {
    pub(crate) fn from_parts(
        profile: ClaudeAgentSdkSessionProfile,
        bundle: ResolvedSkillBundle,
    ) -> Self {
        Self { profile, bundle }
    }

    /// Returns the Copy profile carried by this binding.
    #[must_use]
    pub const fn session_profile(&self) -> ClaudeAgentSdkSessionProfile {
        self.profile
    }

    /// Returns the immutable resolved bundle carried by this binding.
    #[must_use]
    pub const fn bundle(&self) -> &ResolvedSkillBundle {
        &self.bundle
    }
}

/// Renders and revalidates one resolved bundle before provider-facing work.
pub(crate) fn render_bundle(bundle: &ResolvedSkillBundle) -> Result<Value, RuntimeFailure> {
    if bundle.references().len() > MAX_SELECTED_SKILL_REQUIRED_REFERENCES {
        return Err(selected_skill_failure(
            "swallowtail.claude-agent.sdk.selected_skill_limit_exceeded",
            "Claude Agent SDK selected skill bundle exceeds its reference bound",
        ));
    }
    let body = render_payload(bundle.body(), bundle.digest())?;
    let mut bytes = bundle.body().byte_len();
    let mut ids = BTreeSet::new();
    let mut references = Vec::with_capacity(bundle.references().len());
    for reference in bundle.references() {
        if !ids.insert(reference.id().as_str()) {
            return Err(selected_skill_failure(
                "swallowtail.claude-agent.sdk.selected_skill_reference_invalid",
                "Claude Agent SDK selected skill bundle repeats a required reference",
            ));
        }
        bytes = bytes.saturating_add(reference.payload().byte_len());
        references.push(json!({
            "id": reference.id().as_str(),
            "digest": reference.digest().as_str(),
            "content": render_payload(reference.payload(), reference.digest())?,
        }));
    }
    if bytes != bundle.resolved_content_bytes() || bytes > MAX_SELECTED_SKILL_CONTENT_BYTES {
        return Err(selected_skill_failure(
            "swallowtail.claude-agent.sdk.selected_skill_limit_exceeded",
            "Claude Agent SDK selected skill bundle exceeds its bounded content total",
        ));
    }
    Ok(json!({
        "identity": bundle.identity().id().as_str(),
        "provenance": bundle.identity().provenance().as_str(),
        "revision": bundle.revision().as_str(),
        "digest": bundle.digest().as_str(),
        "body": body,
        "requiredReferences": references,
    }))
}

fn render_payload(
    payload: &RegisteredToolPayload,
    expected_digest: &SelectedContentDigest,
) -> Result<Value, RuntimeFailure> {
    if &SelectedContentDigest::of_bytes(payload.expose_for_execution()) != expected_digest {
        return Err(selected_skill_failure(
            "swallowtail.claude-agent.sdk.selected_skill_digest_mismatch",
            "Claude Agent SDK selected skill content digest could not be verified",
        ));
    }
    let body = std::str::from_utf8(payload.expose_for_execution()).map_err(|_| {
        selected_skill_failure(
            "swallowtail.claude-agent.sdk.selected_skill_payload_not_text",
            "Claude Agent SDK selected skill payload is not valid UTF-8 text",
        )
    })?;
    Ok(json!({
        "mediaType": payload.media_type().as_str(),
        "content": body,
    }))
}

fn selected_skill_failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}
