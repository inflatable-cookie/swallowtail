# 063 Kimi Cross-Transport Binding Import And Conformance

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../020-kimi-code-local-server-route.md`

## Objective

Allow one consumer-authorized Kimi ACP session to receive a new local-server
management binding without turning raw provider identity into authority.

## Governing Refs

- cards 061-062
- Contracts 017, 029, 037-038

## Scope

1. Add one explicit adapter-local import input and side-effect-free preflight.
2. Require exact agreement on executable release, execution host, opaque Kimi
   state-root resource, provider session reference, server configured instance,
   endpoint, access, and target lookup.
3. Produce a new local-server management binding; never mutate or widen the
   ACP binding.
4. Prove mismatch, missing target, archived state, unsupported action,
   unverified-newer acceptance, cancellation, deadline, disconnect, and joined
   cleanup.
5. Run the lifecycle conformance pack under attached and owned topologies.
6. Keep local Nucleus thread state and consumer authorization outside
   Swallowtail.

## Acceptance Criteria

- [x] a raw session id, list result, path, or ACP family match cannot import
- [x] import performs no archive, restore, load, resume, prompt, or delete
- [x] target lookup is identity evidence, not an implicit management effect
- [x] the new binding advertises archive and restore only
- [x] mismatched host, version, state root, endpoint, credential, instance, or
      target fails before effects
- [x] unverified-newer posture remains exact and consumer-accepted
- [x] all transport, credential, process, task, and timer work joins

## Evidence

- Kimi ACP preparation can bind one opaque state-root identity. A matching
  durable ACP resume binding can mint `KimiAcpSessionImportAuthority`; the
  type has no raw-id constructor and grants no lifecycle operation.
- Local-server preparation emits one exact import-target snapshot containing
  configured instance, endpoint, access profile and evidence, server
  observation, and state-root identity.
- `prepare_binding_import` compares source host, exact executable release,
  state root, target snapshot, lifecycle capability set, and explicit
  unverified-newer acceptance without host or provider work.
- Execution authorizes the exact endpoint, acquires the exact bearer lease,
  and performs one bounded authenticated
  `GET /api/v1/sessions/{session_id}`. Only an exact unarchived target issues a
  new `ExplicitlyImported` local-server management binding.
- The imported binding advertises archive and restore only. ACP remains
  unchanged and provider-session delete remains unsupported.
- Deterministic attached and owned fixtures prove target drift, missing and
  archived targets, disconnect, cancellation, deadline, credential release,
  owned-child join, and explicit archive/restore after import.

## Validation Evidence

Passed on 2026-07-27:

- `cargo test -p swallowtail-adapter-kimi` — 47 passed, one separately gated
  live installed probe ignored
- `cargo clippy -p swallowtail-adapter-kimi --all-targets -- -D warnings`
- `effigy check:rust`
- `effigy format:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy qa:routes`

`effigy doctor` returns the pre-existing 32 oversized-file findings
(23 warnings, 9 errors). This batch introduced no finding.

`effigy package:api` returns the expected held-candidate diff. The Kimi import
surface joins the unbaselined provider-session and local-server APIs; card 059
still owns replacement after canonical source history exists.

## Validation

- focused import and lifecycle conformance
- attached and owned topology matrix
- ACP and provider-session regression
- public API and docs checks

## Stop Conditions

- state-root equality requires exposing or comparing raw paths in stable values
- the server cannot authoritatively look up the exact target
- import weakens the existing driver-bound management model

## Auto-Continuation

Yes. Continue to card 064 after cross-transport lifecycle conformance passes.
