# 082 Qwen Headless Conformance And Closeout

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../026-qwen-headless-structured-harness-proof.md`

## Objective

Run the production Qwen driver through the one-shot structured-CLI profile,
prove isolation and native-bound assertions, and close roadmap 026.

## Governing References

- Research 017
- Contracts 005, 006, 008-011, 014, and 023
- roadmap 026
- cards 080-081 evidence

## Scope

- local and remote-authoritative host identities
- success, native budget, cancellation, deadline, provider failure, malformed
  output, disconnect, redaction, durable-retention truth, and joined cleanup
- explicit `AmbientHost` claim with no provider- or host-enforced implication
- installed/authenticated probe remains separately gated
- full roadmap, architecture, front-door, log, and continuation closeout

## Acceptance Criteria

- [x] the existing one-shot profile passes without provider-specific branches
- [x] Contract 023 assertions pass under both host topologies
- [x] enforced-isolation substitution fails before process effects
- [x] process exit leaves no detached task and claims no transcript deletion
- [x] existing drivers and profiles remain unchanged
- [x] one next provider-breadth checkpoint remains after closeout

## Validation

- focused Qwen and testkit tests
- focused warnings-denied clippy
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- conformance requires provider identity in the common profile
- remote-authoritative topology changes access or isolation semantics
- authenticated evidence contradicts the frozen access or terminal boundary

## Auto-Continuation

No. Return to the direct-provider compatibility evidence checkpoint.

## Completion Evidence

- the existing one-shot structured-CLI profile remains provider-free and
  unchanged
- a separate Contract 023 assertion pack proves ambient harness authority,
  explicit durable retention, native-budget independence, and no transcript-
  deletion claim without creating an eleventh transport profile
- production Qwen fixtures pass under local and remote-authoritative host
  identities across success, native exits, provider and protocol failure,
  disconnect, cancellation, timeout, redaction, and pre-effect isolation
  rejection
- every started process is waited and every scoped reader task is joined;
  terminal outcomes contain no remote-resource deletion record
- focused tests and warnings-denied clippy pass; full `effigy qa` passes with
  360 tests and three separately gated probes ignored
- `effigy doctor` remains at the inherited 19 findings: 12 warnings and 7
  errors; `git diff --check` passes
