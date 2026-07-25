# 121 OpenCode Range Corpus And Health Observation

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../040-cross-harness-compatibility-range-expansion.md`

## Objective

Freeze the selected OpenCode server range and its exact health observation
before production dispatch changes.

## Scope

- `opencode.server` semantic-version axis and
  `opencode.http.server-window-1` candidate claim
- candidate baseline `1.14.48` and latest boundary `1.18.4`
- recursively closed selected-surface manifests for the six HTTP/SSE routes
- exact default-QA coverage for all 45 stable releases, 18 recursively closed
  selected surfaces, and 20 contiguous published segments
- exact membership evidence for all 45 stable releases in the envelope
- lower `1.14.47`, unknown newer `1.18.5`, prerelease, malformed, missing,
  unhealthy, and session-mismatch rejection fixtures
- classify every relevant schema or lifecycle boundary into exact private
  behavior revisions; merge only byte- or structure-proven equal spans
- parse `GET /global/health` into one safe exact server binding and
  compatibility classification without returning endpoint or raw payload
- keep health observation separate from installed-executable discovery,
  endpoint authority, provider authentication, and configured-instance
  promotion
- deterministic fixtures before driver dispatch changes

## Acceptance Criteria

- [x] every claimed point has authoritative frozen evidence
- [x] every published stable release belongs to one evidence-backed span
- [x] both sides of every selected-surface boundary are fixtures
- [x] observation grants no install, auth, route, or execution authority
- [x] corpus exposes no secret, path, or raw provider payload
- [x] unknown and malformed versions fail closed
- [x] production dispatch work is exact enough to compile

## Validation

- focused corpus and observation tests
- workspace all-target check
- workspace warnings-denied clippy
- `git diff --check`

## Auto-Continuation

Yes, after every selected-surface boundary has an exact behavior revision.

## Outcome

The corpus records all 45 stable tag commits and OpenAPI digests, 18
recursively closed selected-surface digests, and 20 contiguous semantic
segments. Unpublished patch and cross-minor gaps remain unsupported.

The adapter-private candidate claim classifies every frozen release and rejects
outer, prerelease, malformed, unpublished, and synthetic-gap versions. Safe
health observation returns only the exact `opencode.server` binding and
compatibility match. Session-version drift rejects.

The production descriptor, preflight, health gate, and session gate remain
exact `1.14.48`; card 122 owns widening them.
