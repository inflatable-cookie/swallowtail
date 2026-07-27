# 057 OpenCode Session Deletion Conformance And Closeout

Status: completed
Owner: Tom
Created: 2026-07-26
Updated: 2026-07-27
Milestone: `../018-opencode-session-deletion-proof.md`

## Objective

Prove OpenCode deletion across the maintained server range, both host
topologies, and destructive failure boundaries.

## Governing Refs

- Contracts 011, 014, 029, and 038
- cards 055-056
- existing OpenCode attached-harness conformance

## Scope

1. Run shared management assertions against the production HTTP driver.
2. Cover baseline, all selected-surface revisions, latest-qualified, gaps,
   exclusions, and one permitted unverified-newer point.
3. Cover target drift, authorization failure, already absent, provider error,
   cancellation, deadline, disconnect after dispatch, and access release.
4. Run local and remote-authoritative hosts without a live OpenCode service.
5. Update OpenCode prepared guidance and roadmap evidence.

## Acceptance Criteria

- [x] every qualified range segment passes its exact delete mapping
- [x] all pre-dispatch failures cause no delete attempt
- [x] after-dispatch loss remains unconfirmed
- [x] provider data deletion is never relabeled hard deletion
- [x] transport work joins before delegated access release
- [x] existing health, catalogue, session, SSE, and abort suites remain green

## Validation

- full OpenCode adapter suite
- provider-neutral management conformance
- relevant package and docs checks
- one meaningful repository validation round

## Stop Conditions

- topology changes provider result semantics
- the maintained support range must shrink
- a failure path loses the effect boundary

## Auto-Continuation

No. Close roadmap 018.

## Outcome

The production prepared path now executes at the minimum of every exact
deletion segment plus latest-qualified `1.18.4`. Local and
remote-authoritative execution-host identities preserve the same provider-data
truth. Stable `1.18.5` executes only after explicit unverified-newer
acceptance.

Deterministic failure coverage includes denied endpoint authorization, elapsed
pre-dispatch deadline, missing target, provider 401, malformed success, 5xx,
disconnect, cancellation, and deadline after DELETE dispatch. Confirmed
deletion never exceeds `ProviderDataDeleted`. Missing remains rejection, not
idempotent already-absent success. Every post-dispatch ambiguity remains
unconfirmed.

The fixture server now resets accepted sockets to blocking mode. This removes
a macOS-only `EAGAIN` race inherited from its nonblocking listener and makes
the full parallel adapter suite deterministic without changing production
transport behavior.

Prepared guidance and the compiled OpenCode example now include the explicit
inactive-session delete path. Roadmap 018 is complete.

## Validation Evidence

- full OpenCode adapter: 56 passed; one live installed probe skipped
- repeated focused deletion conformance: 5 passed
- isolated prior leak marker rerun: passed
- `effigy check:examples`: passed
- `effigy check:rust`: passed
- `effigy format:check`: passed
- `effigy qa:docs`: passed
- `effigy qa:northstar`: passed
- `git diff --check`: passed
