# 078 Kimi Headless And Retained Local Structured Runs

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../024-kimi-structured-coverage-and-matrix-closeout.md`

## Objective

Add one native Kimi headless run and one retained local-server run while
preserving their different transport and lifecycle authority.

## Scope

1. Add a separate Kimi headless driver, compatibility binding, fixtures, and
   prepared operation.
2. Add local-server structured execution through exact session create, prompt,
   WebSocket terminal, and close.
3. Require `DurableAllowed` for local-server provider retention.
4. Claim no delete, hard delete, secure erasure, or implicit archive.
5. Cover attached and owned-foreground local-server topology with joined child
   cleanup.

## Acceptance Criteria

- [x] headless, ACP, and local server retain separate route identities
- [x] one prompt and one terminal outcome per run
- [x] local-server close preserves the Kimi thread
- [x] retention mismatch fails before session creation
- [x] callbacks, questions, cancellation, deadline, and disconnect remain exact
- [x] `0.29.2` guaranteed evidence is used

## Validation

- Kimi full adapter and package suites
- projection conformance under both host identities
- owned and attached local-server cleanup matrix
- strict Clippy, docs, routes, and `git diff --check`

Completed evidence:

- Kimi headless owns a separate descriptor, compatibility claim, prepared
  operation, stream-JSON parser, cancellation, deadline, and joined process
  path across `0.29.0..=0.29.2`
- Kimi local server registers an independent structured role and projects one
  operation-private session plus one prompt without exposing session authority
- attached, remote-authoritative, and owned-foreground fixtures preserve
  durable thread retention, callbacks, cancellation, timeout, disconnect,
  reasoning, and cleanup truth
- one installed Kimi Code facade requires explicit ACP or headless selection;
  the local-server facade remains separate
- 75 deterministic Kimi tests pass; one gated live probe is ignored
- strict Kimi Clippy and rustdoc, all workspace examples, route and docs checks
  pass

## Stop Conditions

- headless cannot bound background work under host deadline
- retained session identity leaks through stable diagnostics
- owned cleanup would kill work outside Swallowtail authority

## Auto-Continuation

Yes. Continue to card 079.
