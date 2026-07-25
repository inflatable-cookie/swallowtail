# 012 Nucleus Integration Simplification And Acceptance

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../004-nucleus-prepared-facade-adoption.md`

## Objective

Delete superseded Nucleus integration glue and prove the simplified facade path
through deterministic and gated live acceptance.

## Governing Refs

- completed card 011
- Swallowtail Contract 037
- Nucleus Contracts 030-031
- Nucleus authority and validation surfaces

## Scope

1. Remove superseded local task, host-service, installed-discovery,
   configured-instance, requirements, preflight, and policy-copy helpers.
   Retain the minimal executable-path resolver because target selection remains
   Nucleus execution-host authority.
2. Retain only Nucleus-owned translation and product behavior.
3. Update tests and consumer docs to the prepared path.
4. Run deterministic failure, cancellation, callback, and cleanup acceptance.
5. Run separately gated exact installed-version, catalogue, read-only session,
   and bounded-task probes when authorized.
6. Record source rollback and remaining live-auth limits.

## Acceptance Criteria

- [x] duplicated Swallowtail setup is removed
- [x] no old and new integration paths coexist
- [x] Nucleus product ownership remains unchanged
- [x] deterministic runtime preparation and cleanup pass
- [x] gated probes are reported separately from default QA
- [x] rollback is exact and consumer-owned
- [x] roadmap g02.004 closes with Soundcheck migration ready

## Validation

- Nucleus focused adapter/server tests
- Nucleus `effigy check:rust` and normal QA
- gated live probes only under their declared selectors/authorization
- dependency and dead-code audit
- `git diff --check`

## Evidence Required

- before/after line and dependency delta
- deterministic and gated validation results
- Nucleus log and roadmap closeout
- Soundcheck migration readiness statement

## Stop Conditions

- deletion removes consumer policy or persisted compatibility
- live testing requires new authority or provider mutation
- rollback cannot be stated exactly
- Soundcheck would require a facade change rather than ordinary adoption

## Completion Evidence

- Production facade translation is 83 lines in `preparation.rs`; it replaces a
  310-line tracked manual preflight module, the separate installed-discovery
  helper, and adapter-local task/service assembly.
- No Nucleus dependency was added. Existing product traits, DTOs, turn loops,
  callback execution, task outcomes, receipts, persistence, and UI remain
  unchanged.
- A dead-path audit found no manual `ConfiguredInstance`, discovery request,
  open-session policy copy, or legacy plan helper in the live adapter.
- `effigy check:rust` passes.
- `effigy health --json` passes and clears the original doctor compile failure.
- `effigy test nextest -p nucleus-agent-adapters` passes 18 tests; two
  authenticated tests remain gated.
- `effigy test nextest -p nucleus-server` passes 1,991 tests; 12 tests remain
  gated. Existing server test-only unused-import warnings are unchanged.
- Nucleus docs QA, Northstar QA, and `git diff --check` pass.
- Nucleus doctor still reports its known god-file error and generated-source
  warning; 14 checks pass.
- The Nucleus log records exact source rollback and the unrun live-auth gate.
- Soundcheck needs ordinary facade adoption, not a Swallowtail API change.

## Auto-Continuation

No. Return to Swallowtail, close roadmap g02.004, and rebaseline card 013 under
Soundcheck authority.
