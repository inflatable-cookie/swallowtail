# 080 Qwen Headless Isolation Records And Fixtures

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../026-qwen-headless-structured-harness-proof.md`

## Objective

Realize Contract 023's operation-shape-neutral harness-isolation binding and
freeze the exact Qwen Code `v0.19.11` headless corpus without a binary,
credential, or provider request.

## Governing References

- Research 017
- Contracts 005, 006, 008-011, 014, and 023
- roadmap 026

## Scope

- add optional exact harness isolation to common operation requirements and
  runtime operation policy
- pure preflight rejects direct-inference isolation, unsupported shape use, and
  requirement/request mismatches before effects
- keep existing interactive-session access policy and all realized drivers
  unchanged
- add `swallowtail-adapter-qwen` with dated source and protocol fixtures only
- pin release `v0.19.11`, commit `f22cf50`, stable headless support authority,
  delegated access identity, exact model route, ambient isolation, durable
  local retention, stdin prompt, argv exclusions, stream-JSON bounds, native
  budgets, exits, cancellation, redaction, and unknown-event behavior
- freeze safe mode, explicit approval posture, and exact tool exclusions; do
  not infer isolation from them
- no installed probe, live auth, external request, paid inference, container,
  repository write, or production driver

## Ordered Steps

1. Add the minimum common requirement and runtime-policy isolation records.
2. Add pure compatibility and preflight tests with zero recorded effects.
3. Add the Qwen adapter crate and exact `v0.19.11` fixture manifest.
4. Add deterministic stream-JSON success, native-budget, provider-failure,
   cancellation, malformed, unknown-event, retention, and redaction fixtures.
5. Run focused common and Qwen fixture validation.

## Acceptance Criteria

- [x] structured harness runs can bind `AmbientHost`, `ProviderEnforced`, or
      `HostEnforced` independently of interactive sessions
- [x] direct inference cannot carry harness isolation
- [x] mismatched runtime policy fails before process work
- [x] tool denial, safe mode, and native budgets do not imply isolation
- [x] durable local retention is explicit and grants no resume or deletion
- [x] prompt content is absent from argv and fixture diagnostics
- [x] native budget, host cancellation, provider failure, and malformed output
      remain distinct
- [x] card 081 has no unresolved shared-record, argv, output, or access decision

## Completion Evidence

- common operation requirements and runtime policy carry optional exact
  `HarnessIsolation`; pure preflight rejects direct-inference claims
- `validate_harness_isolation_policy` compares the request with its immutable
  preflight binding before any provider effect; legacy `None`/`None` remains
  consistent until each older driver migrates
- `swallowtail-adapter-qwen` pins release `v0.19.11` and full source commit
  `f22cf5009ee3eb26b5c5de2eca6e1f1d0ffee0ad`
- the frozen invocation uses text stdin, stream-JSON stdout, safe mode,
  approval mode `default`, a registry-level five-tool read allowlist, explicit
  high-risk denials, exact native budgets, and no sandbox or container
- terminal fixtures keep provider failure, turn limit `53`, native budget
  `55`, cancellation `130`, malformed output, and success distinct
- durable project-scoped JSONL retention is accepted without resume,
  enumeration, deletion, or exit-implies-deletion authority

## Validation Evidence

- 110 focused tests pass across `swallowtail-core`, `swallowtail-runtime`,
  `swallowtail-testkit`, and `swallowtail-adapter-qwen`
- focused warnings-denied clippy, workspace all-target compile, Effigy docs QA,
  formatting, and diff checks pass
- doctor remains at the inherited 19 oversized-file findings after splitting
  the touched common records; no new structural debt remains
- no executable, credential, network request, paid inference, or container was
  used

## Evidence Required

- common record and pure preflight tests
- dated source manifest and exact version/commit evidence
- deterministic bounded stream-JSON corpus
- zero process-effect proof on policy mismatch
- no live binary, credential, network, or container dependency

## Validation

- focused core and runtime tests
- Qwen adapter fixture tests
- focused warnings-denied clippy
- `git diff --check`

## Stop Conditions

- the exact stable release lacks a bounded headless stream-JSON route
- ambient customizations or mutating tools cannot be suppressed explicitly
- the first proof would need provider sandbox or container authority
- auth identity cannot remain explicit without reading raw harness credentials
- shared records would encode Qwen identity or alter existing driver claims

## Auto-Continuation

No. Promote card 081 only after the frozen corpus proves the selected
read-only ambient invocation and exact native terminal boundaries.
