# 052 ACP v1 Lifecycle Currentness And Corpus

Status: completed
Owner: Tom
Created: 2026-07-26
Milestone: `../017-acp-lifecycle-and-claude-agent-proof.md`

## Objective

Refresh Swallowtail's bounded ACP v1 subset for capability-gated
`session/close` and `session/delete`, then freeze Claude Agent tagged behavior.

## Governing Refs

- Research 036
- Contracts 015, 029, 035, and 038
- stable ACP v1 schema and documentation
- Claude Agent `0.53.0..=0.61.0` tagged sources

## Scope

1. Pin the exact stable ACP v1 source revision used for the additive refresh.
2. Add bounded close/delete request, response, capability, and error fixtures.
3. Preserve ACP deletion as history removal by default.
4. Freeze Claude Agent close/delete handler behavior at every existing
   qualified milestone and exclusion boundary.
5. Record whether exact Claude behavior supports a stronger deletion claim.
6. Keep ACP v2, list/import UX, fork, export, and unrelated protocol additions
   out of scope.

## Acceptance Criteria

- [x] close and delete capability gates are independent
- [x] missing capability forbids dispatch
- [x] ACP soft/hard ambiguity remains visible
- [x] active and missing-session behavior is exact per Claude segment
- [x] current main does not substitute for tagged range evidence
- [x] stdio and remote ACP use the same bounded protocol records

## Validation

- protocol fixture and codec tests
- Claude compatibility corpus tests
- schema hash and source inventory checks
- `git diff --check`

## Stop Conditions

- stable ACP v1 source cannot be pinned
- the additive subset requires ACP v2
- Claude tagged behavior conflicts inside one claimed segment
- stronger deletion truth lacks primary evidence

## Auto-Continuation

No until the corpus is reviewed. Then make card 053 ready.

## Completion Evidence

- Research 038 pins stable ACP schema `v1.20.0`, source commit
  `5e89c71497fe07dd4ae633c181a17224f4a8956d`, and the unchanged stable
  lifecycle schema hash
- protocol fixtures separate close-only, delete-only, omitted, null, success,
  and error shapes while retaining portable `HistoryRemoved` truth
- the shared ACP message codec accepts the same lifecycle records through
  bounded stdio and remote framing
- Claude tagged source, tests, ACP SDK, and Agent SDK packages are frozen at
  all four qualified behavior milestones plus tag-only `0.58.0`
- qualified Claude delete is `ProviderDataDeleted` with
  `ProviderDefinedDescendants`; hard erasure and Anthropic API service-data
  deletion remain excluded
- missing and repeated close or delete reject; active delete tears down before
  SDK deletion
- published Claude Agent `0.62.0` remains unverified-newer and does not extend
  the guaranteed range
- ACP protocol, remote transport, and Claude Agent suites pass 87 tests; Rust,
  format, docs, Northstar, and diff checks pass
- `effigy doctor` remains at the inherited 25 findings
  (17 warnings, 8 errors)
- card 053 is ready for production mapping
