# 014 Soundcheck Integration Simplification And Acceptance

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../005-soundcheck-prepared-facade-adoption.md`

## Objective

Delete superseded Soundcheck integration glue and prove the prepared catalogue
and exec paths through deterministic and gated live acceptance.

## Governing Refs

- completed card 013
- Swallowtail Contract 037
- Soundcheck authority and validation surfaces

## Scope

1. Remove superseded local task, host-service, configured-instance,
   requirements, access, and preflight helpers.
2. Retain only Soundcheck-owned request/result projection.
3. Update tests and consumer docs to the prepared path.
4. Prove deterministic version, policy, schema, attachment, search, deadline,
   cancellation, failure, and cleanup behavior.
5. Run separately gated installed catalogue and bounded structured-run probes
   when authorized.
6. Record source rollback and remaining live-auth limits.

## Acceptance Criteria

- [x] duplicated setup is removed
- [x] no old and new integration paths coexist
- [x] product behavior remains downstream
- [x] deterministic runtime preparation closes the compile-only gap
- [x] gated probes remain separate from default QA
- [x] rollback is exact and consumer-owned
- [x] roadmap g02.005 closes with card 015 ready

## Validation

- Soundcheck focused tests and `effigy health`
- Soundcheck normal test/QA surface
- gated live probes only under declared authorization
- dependency and dead-code audit
- `git diff --check`

## Evidence Required

- before/after line and dependency delta
- deterministic and gated validation results
- Soundcheck log and roadmap closeout
- cross-consumer release-proof readiness statement

## Execution Evidence

Soundcheck deletes the 202-line manual preflight module and the local thread
task service. The replacement adds one consumer-input preparation module and
uses Swallowtail's joined local host composition. Across the three integration
source files, the tracked-plus-new diff adds 372 lines and removes 328; the
added deterministic exact-version fixture accounts for the remaining net
growth. The migration changes no dependency manifest.

Validation:

- `effigy health`: pass
- `effigy test cargo-nextest`: 106 passed, 2 skipped
- `effigy test vitest`: 13 passed
- `cargo check -p soundcheck-app --all-targets --locked`: pass
- `effigy qa`: pass
- direct/legacy transport and superseded setup audit: pass
- `git diff --check`: pass

No installed or authenticated Codex probe ran. Those probes remain separately
gated and are not part of default QA. Source rollback restores the prior
`swallowtail_codex.rs`, `host.rs`, and `preflight.rs`; there is no settings,
schema, or product-data migration.

Card 015 is ready for packaged cross-consumer runtime proof.

## Stop Conditions

- deletion removes product policy
- live testing requires new authority or provider mutation
- rollback cannot be stated exactly
- packaged consumer proof requires a new Swallowtail API change

## Auto-Continuation

No. Return to Swallowtail, close roadmap g02.005, and rebaseline card 015.
