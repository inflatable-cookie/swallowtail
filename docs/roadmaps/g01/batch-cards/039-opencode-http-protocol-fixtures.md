# 039 OpenCode HTTP Protocol Fixtures

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../012-opencode-http-harness-proof.md`

## Objective

Freeze the exact attached OpenCode HTTP/SSE subset before adapter code.

## Governing References

- Research 004
- Contracts 005, 006, 008, 009, 013, and 014
- Roadmap 012

## Scope

- health/version, provider/model discovery, session create, async prompt,
  events, abort, and attached close
- deterministic HTTP/SSE fixtures
- exact provider/model identity and access mapping
- permission and callback stop rules

## Out Of Scope

- server auth mutation
- OpenCode config or credential-store access
- provider selection or fallback
- production adapter

## Implementation Steps

1. Capture the installed OpenAPI subset and compare maintained docs.
2. Record version/default drift and exact supported requests.
3. Build bounded fake HTTP/SSE fixtures, including disconnect and error.
4. Ready card 040 only if permission and session semantics are settled.

## Acceptance Criteria

- [x] fixture subset is versioned and minimal
- [x] explicit endpoint replaces port assumptions
- [x] provider and model ids remain separate
- [x] no auth/config mutation is required

## Validation

- fixture parser tests
- `git diff --check`

## Evidence Required

- source inventory and installed OpenAPI comparison
- success, abort, disconnect, and unknown-event fixtures

## Evidence

- installed `1.14.48` health and OpenAPI were probed through an explicit
  loopback `--pure` server without provider or config access
- the captured OpenAPI SHA-256 and six operation ids are recorded beside the
  fixture
- maintained default-port and permission-reply drift is explicit; neither
  unstable default belongs to the supported subset
- session creation uses deny-first rules with only `read`, `glob`, and `grep`
  allowed; permission and question events stop and abort
- bounded fixtures cover health, catalogue, session create, async prompt,
  abort, HTTP failure, ordered success, provider failure, cancellation,
  disconnect, provider requests, keepalive, and unknown events
- six focused protocol-fixture tests pass
- preflight plans now retain opaque credential references and separate harness
  provider ids required by the frozen request shapes
- full repository QA passes with 137 tests

## Stop Conditions

- maintained docs and installed schema disagree on a required stable method
- safe read-only permission mapping is unresolved

## Auto-Continuation

Yes, only if card 040 is marked ready during closeout.
