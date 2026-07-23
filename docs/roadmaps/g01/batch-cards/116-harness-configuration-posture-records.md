# 116 Harness Configuration Posture Records

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../039-installed-harness-compatibility-range-audit.md`

## Objective

Bind ambient, provider-suppressed, and host-scoped harness configuration
independently from process isolation, credentials, retention, and workspace
authority.

## Scope

- promote Contract 033 for operation-shape-neutral harness configuration
- core posture records for `Ambient`, `ProviderSuppressed`, and `HostScoped`
- exact requirement, configured-instance capability, request-policy, and
  preflight agreement
- no fallback from suppressed or scoped posture to ambient
- provider retention and authentication remain independent
- reusable conformance assertions over the existing structured-CLI and
  long-lived harness profiles
- no provider implementation, config parsing, secret copying, or temporary
  credential home

## Acceptance Criteria

- [x] configuration posture is not process isolation
- [x] ambient config requires explicit acceptance
- [x] suppressed config requires exact provider evidence
- [x] host-scoped config requires a separately bound host lease
- [x] retention and credential posture cannot substitute for configuration
- [x] every mismatch rejects before provider work
- [x] common operation shapes gain no Codex branch

## Validation

- focused core, runtime, and testkit tests
- workspace all-target check
- workspace warnings-denied clippy
- `effigy qa:docs`
- `git diff --check`

## Evidence

- Contract 033 binds `Ambient`, `ProviderSuppressed`, and `HostScoped`
  independently from isolation, access, retention, and working resources.
- Configured instances, operation requirements, immutable plans, and runtime
  request policy carry one exact optional posture.
- Absence remains unmigrated state, not an ambient alias.
- Pure preflight rejects direct inference, instance/requirement mismatch, and
  `HostScoped` without a separate host lease before effects.
- The reusable assertion pack covers the existing one-shot structured-CLI and
  long-lived RPC harness profiles without provider identity.
- 66 focused core, runtime, and testkit tests pass.
- Workspace all-target check and warnings-denied clippy pass.

## Auto-Continuation

Completed. Card 117 is ready.
