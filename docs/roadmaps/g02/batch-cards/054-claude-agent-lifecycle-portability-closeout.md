# 054 Claude Agent Lifecycle Portability Closeout

Status: completed
Owner: Tom
Created: 2026-07-26
Milestone: `../017-acp-lifecycle-and-claude-agent-proof.md`

## Objective

Prove Claude Agent close and delete semantics through production stdio ACP and
explicit remote-ACP composition.

## Governing Refs

- Contracts 011, 015, 029, 035, and 038
- cards 052-053
- existing ACP topology conformance

## Scope

1. Run shared management assertions against the Claude production driver.
2. Cover every Claude behavior segment, the unpublished exclusion, and one
   permitted unverified-newer point.
3. Cover absent capability, active-target rejection, missing target,
   provider failure, disconnect after dispatch, cancellation, deadline, and
   credential release.
4. Prove stdio and remote ACP retain explicit transport identity with no
   fallback.
5. Update ACP and Claude prepared guidance.

## Acceptance Criteria

- [x] both transports preserve identical qualified lifecycle semantics
- [x] remote transport failure cannot trigger local stdio retry
- [x] ACP history-removal truth is retained unless Claude evidence is stronger
- [x] close and delete cleanup order is joined and deterministic
- [x] existing Claude prompt, callback, and compatibility evidence remains
      green
- [x] no live authentication is required by default tests

## Validation

- full Claude adapter suite
- ACP protocol and remote transport suites
- provider-neutral management conformance
- relevant docs and package checks
- one meaningful repository validation round

## Stop Conditions

- transport choice changes provider deletion truth
- default tests require live Claude access
- remote ACP needs implicit recovery or affinity widening

## Auto-Continuation

No. Close roadmap 017.

## Completion Evidence

- the production stdio driver passes one delete contract across all four
  qualified behavior segments, the `0.58.0` exclusion, and explicit
  unverified-newer `0.62.0` execution
- missing negotiation, missing target, provider rejection, disconnect after
  dispatch, malformed success, cancellation, and deadline retain exact
  failure-before-effect or unconfirmed-after-effect truth
- the public prepared path binds the common role's sole
  `CallerAssertedInactive` activity evidence; an active-target management
  request is not constructible
- process join precedes working-resource release and credential release in
  deterministic fixture evidence; credentials remain last
- the real remote WebSocket ACP transport carries the same qualified
  `initialize` and `session/delete` records under local and
  remote-authoritative host identities
- a remote disconnect after delete has one connection, no process service,
  no retry, no reconnect, and no stdio fallback
- remote ACP remains provider-neutral and unauthenticated; this proof does not
  claim a production remote Claude endpoint or change Claude's stdio route
- focused Claude, ACP, remote-transport, runtime, and testkit validation passes
  without live authentication
