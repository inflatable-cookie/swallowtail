# 041 Qualified Support And Newer-Version Execution

Status: completed
Owner: Tom
Updated: 2026-07-24

## Purpose

Separate Swallowtail's guaranteed compatibility windows from permitted
unverified-newer execution. Avoid forcing a consuming application release for
every non-breaking upstream patch without treating untested versions as
supported.

## Generation Runway

Keep g01 active. It contains 41 numbered roadmaps and remains inside the normal
30-50 roadmap range.

## Goals

- [x] Correct Contract 029 and installed discovery classification.
- [x] Add qualified, unverified-newer, and incompatible core assessments.
- [x] Permit unverified-newer preflight only for explicitly opted-in ordered
      claims.
- [x] Prove Codex and OpenCode forward attempts without widening their
      guaranteed ranges.
- [x] Recompile the provider-coverage checkpoint after the policy correction.

## Execution Plan

- [x] Forward-compatibility policy correction: card 124.
- [x] Core assessment and preflight: card 125.
- [x] Codex and OpenCode newer-version dispatch: card 126.
- [x] Forward-compatibility conformance and closeout: card 127.

## Cards

- `batch-cards/124-forward-compatibility-policy-correction.md` — completed
- `batch-cards/125-unverified-newer-core-assessment.md` — completed
- `batch-cards/126-codex-opencode-newer-version-dispatch.md` — completed
- `batch-cards/127-forward-compatibility-conformance-and-closeout.md` —
  completed

## Boundaries

- the qualified range remains the only guaranteed support claim
- unverified newer is never silently promoted to qualified
- exact version identity remains bound through discovery, preflight, and
  execution
- explicit exclusions, unsupported in-range gaps, prereleases, malformed
  values, and below-baseline versions remain closed
- opaque axes remain qualified-only
- no install, update, downgrade, provider, model, endpoint, credential, or
  route fallback
- no consumer, Nucleus, or Soundcheck edit

## Current Evidence

Contract 029 previously classified every version above latest-qualified as
incompatible. The operator corrected that product policy on 2026-07-24.
Consuming applications may remain deployed while harnesses release frequently;
qualified windows therefore mean guaranteed support, not a hard upper
execution ceiling.

Codex and OpenCode are the first proof routes because they already have exact
ordered observations, closed qualified ranges, private latest behavior
revisions, and deterministic range corpora. Both now permit newer stable exact
versions as unverified without widening their qualified support claims.

Roadmap 042 and card 128 resume provider-coverage selection.
