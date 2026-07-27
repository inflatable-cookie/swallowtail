# 053 Claude Agent ACP Session Management

Status: completed
Owner: Tom
Created: 2026-07-26
Milestone: `../017-acp-lifecycle-and-claude-agent-proof.md`

## Objective

Use qualified ACP native close during Claude Agent cleanup and expose
user-directed deletion through one durable management binding.

## Governing Refs

- Contracts 015, 017, 029, 037-038
- cards 048 and 052
- existing Claude Agent prepared facade and range dispatch

## Scope

1. Return a management binding without claiming load or resume support not
   implemented by the adapter.
2. Send `session/close` during handle cleanup only when negotiated and
   qualified.
3. Add a separate prepared delete operation for an inactive bound session.
4. Map exact Claude deletion strength from card 052 without promoting generic
   ACP semantics.
5. Preserve API-key access, ambient configuration and isolation, exact model,
   callbacks, cancellation, deadline, and credential-last cleanup.
6. Keep Claude subscription and private OAuth routes excluded.

## Acceptance Criteria

- [x] connection close and native session close remain distinct cleanup legs
- [x] native close preserves persistent history
- [x] delete requires negotiated capability and explicit consumer action
- [x] no resume or provider-history listing claim is fabricated
- [x] qualified and unverified-newer status remains visible
- [x] all protocol and process work joins before access release

## Validation

- focused Claude range, driver, and prepared tests
- ACP protocol regressions
- management conformance pack
- `effigy check:rust`
- `effigy format:check`

## Stop Conditions

- deletion would require extracting Claude credentials or local state paths
- selected access authority cannot invoke the tagged handler
- native close changes provider retention unexpectedly
- a stronger result than card 052 evidence is needed

## Auto-Continuation

Yes after card 052 acceptance. Continue to card 054.

## Completion Evidence

- initialization binds independent ACP close and delete capabilities before
  session creation; missing delete stops before `session/new`
- qualified handle cleanup sends native `session/close`, then closes and joins
  the connection, process, active turn, resource, and credential legs
- prepared sessions expose one opaque management binding with delete and
  native-close capability but no load or resume binding
- `ClaudeAgentPreparedDelete` creates one immutable typed delete plan for an
  inactive target and delegates to the unchanged low-level driver role
- deletion starts one fresh scoped ACP process and reports
  `ProviderDataDeleted` with `ProviderDefinedDescendants` only after the exact
  empty response
- unverified-newer deletion requires explicit prepared-facade acceptance;
  unverified native close is not promoted from the guaranteed range
- API-key audience, ambient harness authority, exact executable and version,
  read-only resource, cancellation, deadline, and credential-last cleanup
  remain bound
- focused Claude, ACP, runtime, and management suites pass 213 tests; targeted
  clippy, Rust, format, docs, Northstar, and diff checks pass
- `effigy doctor` remains at the inherited 25 findings
  (17 warnings, 8 errors)
- card 054 is ready for shared conformance and explicit remote-ACP closeout
