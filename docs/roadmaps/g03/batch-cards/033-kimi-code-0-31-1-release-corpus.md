# 033 Kimi Code 0.31.1 Release Corpus

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../013-kimi-code-0-31-1-range-maintenance.md`
Depends on: card 032

## Goal

Freeze exact Kimi Code `0.31.1` identity and selected route deltas before
widening any production claim.

## Scope

1. Record exact tag, commit, tree, npm, release-asset, and signer identity.
2. Record ACP initialization, unchanged stream renderer, and selected
   local-server source identities.
3. Freeze optional `turn.ended.interruptReason` as bounded ignored input.
4. Add deterministic corpus assertions while `0.31.0` remains the production
   ceiling.

## Acceptance Criteria

- [x] exact official identity and selected source blobs are fixture data
- [x] ACP, headless, and local-server deltas are classified separately
- [x] fixture data contains no credential, host path, account observation, or
  provider payload
- [x] exact `0.31.0` remains the production boundary during corpus acceptance
- [x] focused Kimi tests pass
- [x] card 034 becomes sole ready and next

## Validation

- `effigy validate:focused swallowtail-adapter-kimi`
- `git diff --check`
- no authentication or provider prompt

## Auto-Continuation

Yes. Continue to card 034 when the corpus settles all three route decisions.

## Evidence

- exact source, npm, artifact, signer, and selected blob identities are frozen
- optional `turn.ended.interruptReason` decodes without replacing terminal reason
- 89 focused Kimi tests passed in six seconds
- diff hygiene passed
- no installation, authentication, provider prompt, or durable mutation ran
