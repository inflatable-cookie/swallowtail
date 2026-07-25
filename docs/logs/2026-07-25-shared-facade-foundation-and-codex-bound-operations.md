# 2026-07-25 Shared Facade Foundation And Codex Bound Operations

Status: complete

## Changed

`swallowtail-runtime` now exposes provider-neutral prepared-operation
evidence:

- exact driver, role, execution layer, and operation shape
- configured instance, revision, execution host, opaque target, and facade
- access status and provenance
- every exact interface binding and compatibility assessment
- the immutable expanded preflight plan

Construction rejects access evidence that does not match the plan.
`swallowtail-testkit` asserts the record against installed-harness,
hosted-direct, and attached-runtime fixtures and proves mismatch before
provider effects. No generic execution trait or provider selector was added.

Codex prepared evidence embeds the shared record. Prepared values now expose:

- `CodexPreparedCatalogue::list_models`
- `CodexPreparedExec::start_run`
- `CodexPreparedSession::open_session`
- `CodexPreparedSession::resume_session`

Each method constructs the exact selected Codex low-level driver and delegates
the immutable plan, explicit request, and caller-supplied host services to the
existing role. `low_level_driver`, `plan`, `request`, and `into_parts` remain
available.

## Preserved

- exec and app-server remain separate drivers
- catalogue, structured run, open, and resume remain separate operations
- read-only and bounded-workspace authority remain separate
- exact qualified, deprecated, and unverified-newer evidence remains visible
- preflight, topology, cancellation, deadlines, callbacks, interruption,
  terminal outcomes, and joined cleanup are unchanged
- Nucleus and Soundcheck source remain untouched

## Validation

- runtime, testkit, and complete Codex focused suites: 209 pass
- prepared Codex bound-operation suite: 9 pass
- compile-tested prepared example: pass
- full repository QA: pass, including documentation, workspace checks,
  warnings-denied Clippy, 654 deterministic tests, and four gated live probes
  ignored
- Doctor: no regression; the known structural debt remains at 19 oversized
  files, comprising seven errors and 12 warnings
- public-API comparison: expected additive drift in `swallowtail-core`,
  `swallowtail-runtime`, `swallowtail-testkit`, and
  `swallowtail-adapter-codex`

The held `0.1.0` candidate baseline was not rewritten. Card 036 owns the
replacement candidate and its public-API baseline after provider-wide facade
work.

## Next

Card 020 adds the representative Kimi Code ACP prepared facade.
