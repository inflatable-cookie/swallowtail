# 068 Claude Code Response-Only Protocol Compatibility

Status: active
Owner: Tom
Created: 2026-08-12
Depends on: g03.066; Figmatic `g04.005` preparation evidence
Contract refs: 039, 044

## Problem

Figmatic's packaged `g04.005` mutation-runway smoke exposed a brittle route
boundary: every automatic Claude Code patch update fails discovery before the
strict response-only protocol validator can assess compatibility.

## Generation Runway

Advance g03's compatibility and consumer-proven hardening goals. Keep the
proven protocol baseline while allowing later stable releases to reach the
strict runtime validator without manufacturing qualified evidence.

## Execution Plan

- [x] card 216: admit stable newer releases provisionally, bind every run to
      its observed executable version, add a known-bad deny-list, diagnostics,
      dual-version fixtures, and fail-closed protocol proof

## Downstream Handoff

- [ ] card 217: link the exact implementation commit in Figmatic and run the
      packaged `g04.005` mutation-runway smoke

## Goals

- [x] retain exact `2.1.227` as the minimum qualified baseline and exact
      `2.1.228` as qualified live evidence
- [x] permit stable newer versions only as explicit unverified-newer evidence
- [x] preserve ordinary response text, empty tools and MCP, validated and
      discarded private thinking, and Max/OAuth without an API key
- [x] keep the prepared API, command, route identity, and host authority
      unchanged
- [x] reject below-baseline, prerelease, build-qualified, malformed, and
      statically denied releases before execution

## Acceptance Criteria

- [x] deterministic discovery, init, progress, private-thinking, text, result,
      failure, cancellation, and cleanup cases pass
- [x] every run requires init to echo its preflight-bound observed version
- [x] preparation and run diagnostics expose the exact observed version and
      qualified or provisional posture
- [x] focused and affected-package validation pass
- [x] current guide, route matrix, architecture, testkit, and contract surfaces
      describe protocol compatibility rather than patch equality
- [x] the gated `2.1.228` live probe passes through local Max/OAuth with
      `ANTHROPIC_API_KEY` absent

## Boundaries

- no prepared API, command flag, model, credential, host-service, capability,
  output, retry, continuation, or fallback change
- provisional admission supplies no qualified support claim and no tolerance
  for protocol drift
- no edit to immutable `v0.3.2`, historical research, release, or prior
  qualification records
- no Figmatic mutation in this repository
