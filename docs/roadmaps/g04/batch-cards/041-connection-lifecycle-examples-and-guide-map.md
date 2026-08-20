# 041 Connection Lifecycle Examples And Guide Map

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../014-connection-lifecycle-consumer-path.md`
Depends on: card 040

## Goal

Ship compiling 057 examples for the three first-proofs and complete
Contract 052 traceability for the new feature family.

## Scope

1. Add compile-only examples in the same shape as
   `prepared_direct.rs` (`fn main() {}` plus public helpers):
   `crates/swallowtail-adapter-anthropic/examples/connection_lifecycle.rs`,
   `crates/swallowtail-adapter-codex/examples/connection_lifecycle.rs`,
   `crates/swallowtail-adapter-ollama/examples/connection_lifecycle.rs`.
2. Each example assembles the adapter-local descriptor into
   `AddableRouteCatalog`, admits through
   `MemoryConnectionLifecycleStore`, and reaches the existing prepare
   function. Anthropic shows `CredentialRef` collection. Codex and
   Ollama show no credential field. Overlay may mark Anthropic
   `anthropic` rows; Codex and Ollama rows stay unmarked.
3. Examples are deterministic. No live provider, install, start, pull,
   login, or billing. Do not call `start_sign_in` with a pending host
   future. Do not pretend stored `ConfigFieldRef` values feed
   `prepare_*`; prepare still takes host target refs.
4. Add `connection_lifecycle` to `PORTABLE_FEATURES` in
   `scripts/check-integration-guide-coverage.py`. Do not add a feature-
   matrix column and do not change the 34-column count.
5. Add one complete feature-family row to
   `docs/guides/integration-guide-map.md` whose machine-readable surface
   is exactly `connection_lifecycle` and whose canonical guide is
   `connection-lifecycle.md`. Coverage must be `complete`.
6. Link the three examples from the feature guide and from the 057
   sections of the three route guides. Leave the route-map example
   column on the existing prepared-facade examples.
7. Update architecture: the 052 consumer path is realized for the three
   first-proofs. Remaining production routes have no addable
   descriptors. Hosted interactive OAuth is still not realized.

## Out Of Scope

- hosted OAuth
- addable descriptors for other routes
- changing route-map coverage of the 47 production routes
- rewriting `public-api-0.3.3`
- fixing `start_sign_in` pending-future panic
- feeding stored config refs into prepare

## Acceptance Criteria

- [ ] the three examples compile through `effigy check:examples`
- [ ] `effigy qa:guides` passes with the new complete family
- [ ] no guide or map row claims addable coverage beyond the three
      first-proofs
- [ ] examples contain no secret bytes
- [ ] `public-api-0.3.3` is unchanged

## Validation

- `effigy check:examples`
- `effigy qa:guides`
- `effigy qa:docs`
- `git diff --check`

## Auto-Continuation

No. Hosted OAuth stays a remaining gate. After this PR, the
g04.010 first-proof-plus-consumer-path goal is ready to mark completed
on merge. Do not compile hosted OAuth.

## Stop Conditions

- Stop if `qa:guides` is made to accept partial coverage instead of
      adding the token and complete family together.
- Stop if a matrix column is added.
- Stop if overlay invents a catalogue provider id.
- Stop if hosted OAuth or OpenHands production wiring starts.
