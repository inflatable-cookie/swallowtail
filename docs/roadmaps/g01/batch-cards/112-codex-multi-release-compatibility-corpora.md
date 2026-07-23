# 112 Codex Multi-Release Compatibility Corpora

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../039-installed-harness-compatibility-range-audit.md`

## Objective

Freeze exact Codex exec and app-server release behavior without widening either
production descriptor.

## Scope

- exec checkpoints `0.122.0`, `0.130.0`, `0.140.0`, `0.144.6`, and `0.145.0`
- app-server checkpoints `0.110.0`, `0.120.0`, `0.131.0`, `0.140.0`,
  `0.144.6`, and `0.145.0`
- exact tagged commits, CLI help, selected source, generated stable schemas,
  and generated experimental schemas
- frozen argv, JSONL events, JSON-RPC handshake, catalogue, session, callback,
  cancellation, failure, and close fixtures
- experimental gate enforcement for dynamic tools, user input, provider
  requests, runtime workspace roots, and later experimental fields
- explicit rejection fixtures for exec `0.121.0`, app-server `0.107.0`,
  unpublished `0.108.0` and `0.109.0`, `0.146.0-alpha.4`, malformed versions,
  and unknown newer versions
- no live auth, model request, binary installation during default QA, or
  production descriptor claim

## Acceptance Criteria

- [x] every candidate boundary and milestone has frozen exact evidence
- [x] stable and experimental app-server surfaces cannot substitute for each
      other
- [x] mock app-server fixtures enforce `experimentalApi`
- [x] exec isolation flags are absent at the rejection point and present at
      every claimed candidate
- [x] additive unknown events remain distinguishable from malformed required
      fields
- [x] corpus tests need no installed Codex, credential, network, or container

## Validation

- focused Codex corpus and protocol tests
- focused warnings-denied clippy
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

Yes. Continue to card 113 when both offline corpora pass.

## Evidence

- exact release records freeze tag commits, npm integrity, archive shasums,
  selected source and help digests, generated stable schema digests, and
  generated experimental schema digests
- exec candidates are `0.122.0`, `0.130.0`, `0.140.0`, `0.144.6`, and
  `0.145.0`; `0.121.0` proves both required isolation flags absent
- app-server candidates are `0.110.0`, `0.120.0`, `0.131.0`, `0.140.0`,
  `0.144.6`, and `0.145.0`; stable and experimental bundles have distinct
  exact digests at every point
- `0.131.0` begins the runtime-workspace-root segment;
  `allowProviderModelFallback` appears as experimental at `0.144.6`
- the gate-enforcing server accepts explicitly negotiated dynamic tools and
  provider requests, and rejects the current ungated default-false fallback
  field
- 52 Codex adapter tests pass; focused warnings-denied clippy and
  `git diff --check` pass
