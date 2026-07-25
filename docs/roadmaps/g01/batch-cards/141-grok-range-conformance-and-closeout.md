# 141 Grok Range Conformance And Closeout

Status: backlog
Owner: Tom
Updated: 2026-07-24
Milestone: `../047-grok-build-maintained-acp-range.md`

## Objective

Prove the qualified Grok range through the existing public runtime profiles and
close the final compiled g01 lane.

## Governing Refs

- Research 030 and 031
- Spec 003
- Contracts 011, 012, 013, 015, 017, 023, 029, 032, 033, and 034
- roadmap g01.047
- cards 137-140

## Scope

1. Run the unchanged long-lived ACP profile and applicable persistent-session,
   negotiated-option, configuration, isolation, and installed-range assertion
   packs.
2. Prove baseline, latest-qualified, every milestone, exclusion, deprecated
   segment, and one unverified-newer point.
3. Prove local and remote-authoritative host identity.
4. Prove cancellation, deadline, malformed protocol, provider request,
   disconnect, explicit close, cleanup failure, and redaction.
5. Run full repository QA and close roadmap 047.
6. Return to a deliberate g01/g02 generation checkpoint.

## Boundaries

- no live login, credential, account, model request, or paid inference
- no provider scope beyond the card-137 corpus
- no consumer edit
- no automatic g02 rollover

## Acceptance Criteria

- [ ] common profiles remain provider-neutral
- [ ] qualified and unverified-newer results remain distinct
- [ ] every exact behavior milestone passes both host topologies
- [ ] permissions, configuration, retention, access, and isolation remain
      independent
- [ ] all owned work joins on every terminal path
- [ ] full QA passes or failures are recorded honestly
- [ ] roadmap and front-door state close coherently
- [ ] one generation checkpoint remains next

## Validation

- focused range and topology tests
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Evidence Required

- focused and full test counts
- compatibility matrix
- topology, redaction, and cleanup results
- explicit remaining auth, protocol, capability, support, and generation risks

## Stop Conditions

- cross-topology behavior diverges
- any exact release needs an uncontracted authority
- full QA exposes a contract-level defect

## Auto-Continuation

No. Return to a deliberate generation-boundary checkpoint.

## Generation Disposition

This card remains behind cards 138-140 in the shared
[roadmap backlog](../../backlog/grok-build-maintained-acp-range.md). It stays
with its source generation and is not ready.
