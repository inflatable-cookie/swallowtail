# 089 Alibaba Conversation Driver And Conformance

Status: completed
Owner: Tom
Updated: 2026-07-22
Milestone: `../029-remaining-direct-provider-breadth.md`

## Objective

Implement the exact Alibaba Model Studio direct-conversation driver, conform it
under both execution-host topologies, and close roadmap 029.

## Readiness Gate

Card 088 realizes the provider-neutral session policy, deletion records,
deterministic corpus, and exact production seam. Its focused tests and
warnings-denied clippy pass without a contract or currentness gap.

## Governing References

- Research 019
- Contracts 005, 006, 009, 011, 014, 020, and 025
- roadmap 029
- card 088 completion evidence

## Scope

- separately registered resource-free interactive direct-inference driver
- session-scoped host-approved endpoint and API-key lease
- provider conversation creation, maximum two serial Responses turns, and no
  resume
- exact Singapore workspace, regional audience, pay-as-you-go access, and
  `qwen3.7-plus-2026-05-26` route
- one HTTP/SSE attempt per turn with no response storage, cache, tool, retry,
  reattachment, or fallback
- bounded item inventory, item deletion, then conversation deletion
- local-only cancellation and deadline with honest remote-state uncertainty
- local and remote-authoritative hosted session conformance
- optional live authentication probe remains separately gated
- architecture, roadmap, front-door, log, and continuation closeout

## Ordered Work

1. Implement the production protocol mapper and driver against card 088's
   frozen corpus.
2. Bind endpoint, credential, conversation, turn, and cleanup work to one
   session scope with no detached task.
3. Prove serial success, bounds, failures, cancellation, deadline, disconnect,
   redaction, item inventory, deletion, and cleanup races deterministically.
4. Run provider-neutral direct-session assertions under local and remote-
   authoritative host identities without weakening existing profiles.
5. Run one meaningful full validation round, close roadmap 029, and compile the
   next coverage checkpoint inside g01.

## Acceptance Criteria

- [x] exact instance, workspace, region, endpoint, access, route, model, and
      execution-host identities survive open, turns, and close
- [x] one conversation and maximum two serial turns make only frozen requests
- [x] unsupported fields and access profiles fail before provider effects
- [x] ordered output, usage, errors, unknowns, cancellation, and deadline match
      the frozen provider truth
- [x] complete item inventory precedes item deletion and conversation deletion
- [x] remote-state uncertainty and cleanup failure remain visible
- [x] connection and deletion work join before credential release
- [x] existing conformance profiles and production adapters remain unchanged
- [x] roadmap 029 closes with one clear next task

## Evidence Required

- focused production-driver and provider-neutral conformance transcripts
- exact local and remote-authoritative topology assertions
- request and attempt counts for open, turns, inventory, and deletion
- cleanup ordering and remote-race evidence
- secret, endpoint, workspace, provider-id, and raw-payload redaction checks
- full repository QA and doctor delta

## Validation

- focused Alibaba, runtime, host, and testkit tests
- focused warnings-denied clippy
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- production evidence contradicts the frozen `store=false` conversation or
  deletion boundary
- response work can mutate conversation items after claimed confirmed cleanup
- the driver requires ambient workspace, region, endpoint, key, retry, or model
  selection
- conformance needs a provider-specific branch in an existing profile

## Auto-Continuation

No. Close the proof and return to the wider provider-coverage checkpoint.

## Completion Evidence

- `AlibabaModelStudioDriver` implements the exact direct interactive role over
  host-approved HTTP/SSE with one session-scoped endpoint and API-key lease
- one provider conversation and two successful serial turns produce nine exact
  requests in the full transcript: create, two Responses attempts, complete
  inventory, four item deletions, then conversation deletion
- parallel and third turns, resume, unsupported session/turn inputs, wrong
  access, aliases, and elapsed deadlines reject before another provider effect
- provider sequence, output, returned model, usage, request correlation,
  provider failure, disconnect, cancellation, deadline, redaction, and cleanup
  failure remain distinct
- cancellation and deadline stop and join local connection work, end the
  session, and report provider stop plus both deletion kinds as unconfirmed
- local and remote-authoritative host fixtures preserve the same exact
  instance, target, audience, credential, route, model, and lifecycle
- all 16 Alibaba tests and warnings-denied clippy pass; full repository QA
  passes with 404 tests and three gated probes ignored
- doctor remains at the inherited 19 findings with no new oversized file
