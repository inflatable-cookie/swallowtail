# 110 Installed Harness Compatibility Range Audit

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../039-installed-harness-compatibility-range-audit.md`

## Objective

Revalidate installed harness release history and select the first exact
compatibility-range retrofit.

## Scope

- inventory Codex exec, Codex app-server, OpenCode, Gemini CLI, Kimi Code,
  Qwen Code, and Pi installed-version discovery and current claims
- begin with both Codex drivers because one consumer release may meet client
  installations spanning at least six months
- use current official provider or maintained-project documentation, tagged
  source, release notes, schemas, and exact artifacts
- identify baseline, latest-qualified, behavior milestones, deprecations,
  exclusions, and probe availability per driver and transport
- keep executable, wire protocol, behavior revision, model catalogue, access,
  and adapter release as separate axes
- promote a dated research delta and compile only the first evidence-backed
  fixture and descriptor tranche
- no provider implementation, consumer edit, live credential, compatibility
  shim, or inferred range

## Acceptance Criteria

- [x] every installed production harness route has an explicit compatibility
      posture
- [x] Codex exec and app-server evidence is current and separately classified
- [x] no range spans an untested behavior or schema transition
- [x] the first retrofit has exact corpus checkpoints and rejection points
- [x] one sole next task remains

## Validation

- `effigy qa:docs`
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

No. Stop if the first maintained range remains ambiguous or would establish
consumer support policy not fixed by current authority.

## Evidence

- Research 025 records every installed production harness route's exact pin,
  latest maintained release, runtime observation seam, and claim posture.
- Codex `0.145.0` supersedes the prior `0.144.6` current point. `0.144.6`
  remains a regression checkpoint.
- Exec `0.122.0` is the first candidate for the current isolated invocation;
  `0.121.0` lacks both required isolation flags.
- App-server `0.110.0` is the first published v2-only candidate.
  `0.131.0` begins the runtime-workspace-root behavior segment.
- Stable and experimental schema gates require frozen version-specific
  corpora before either descriptor can publish a range.
- Cards 111-114 compile the first observation, corpus, production, and
  conformance tranche. Card 115 keeps the January-to-April legacy span in
  bounds without weakening the first claims.
