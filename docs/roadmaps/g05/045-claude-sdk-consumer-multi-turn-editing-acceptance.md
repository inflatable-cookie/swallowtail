# g05.045 Claude SDK Consumer Multi-Turn Editing Acceptance

Status: ready; queued behind Desktop g02.049
Owner: Tom
Created: 2026-09-09
Depends on: Contracts 017, 019, 023, 029, and 041; stopped g05.029 audit;
released `v0.4.4`; Northstar Queue task
`bc4acd98-91a3-4e14-86e0-38376b037da7`
Vision tags: Claude route, consumer parity, multi-turn editing, live evidence

## Outcome

Consume Desktop g02.049's merged immutable capsule provider-free and decide
whether it closes the one failed g05.029 clause: a real consumer drives two
editing turns in one `claude-agent.sdk` session, with every tool call mediated
before it runs.

If the literal oracle passes, freeze the exact Desktop merge, review, capsule
digest, released Swallowtail source identity, tuple, prompts, session
continuity, callback/effect order, file transitions, terminal states, and
cleanup as Research 303. Add a deterministic adapter test fixture that binds
the accepted facts to the existing permission/editing contracts. Record the
follow-up beside g05.029 without rewriting its historical stopped audit.

## Ready-State Rubric

- [x] g05.029 names one exact missing clause and its smallest counterexamples.
- [x] Desktop's current `acceptEdits` mapping is identified as the consumer
      blocker; g02.049 owns its minimum repair and the live proof.
- [x] Tom authorized the one live Desktop session and consumer mutation.
- [x] This task has an immutable queue dependency on the Desktop task and no
      provider authority of its own.
- [x] Exact acceptance, evidence, mutable paths, validation, and stops are
      defined without reopening Card 087 or a release lane.

## Decisions

- The original g05.029 audit remains historically `stopped`; later evidence is
  an explicit follow-up, not a retroactive claim that the audit passed.
- Desktop is the observed consumer. A proof-only direct Swallowtail runner, a
  fake SDK, independent one-turn controls, or two separate sessions cannot
  satisfy this task.
- `acceptEdits` cannot prove pre-effect consumer mediation. The accepted
  capsule must show SDK `default` mode and an ordered Desktop decision before
  dispatch, result, or filesystem effect. A streamed tool-use proposal or
  activity-start observation may precede that decision; visibility is not
  execution.
- Swallowtail performs no second provider call. One source-linked external
  capsule plus a provider-free deterministic binding is the evidence chain.

## Dispatch manifest

- **State:** ready; dispatch only after queue dependency
  `bc4acd98-91a3-4e14-86e0-38376b037da7` reaches `done`.
- **Completion:** one literal pass/stop decision, Research 303, one deterministic
  provider-free evidence fixture/test, one log, and a bounded g05.029 follow-up.
- **Owned mutable paths:** this task; `docs/research/303-*.md` and its index
  line; one new fixture and focused test under
  `crates/swallowtail-adapter-claude-agent/tests/**`; the exact evidence note in
  `docs/guides/claude-agent-sdk-prepared-integration.md`; one appended
  `## Follow-up evidence` subsection in g05.029; one new log and its index;
  `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** `docs/roadmaps/README.md`,
  `docs/roadmaps/g05/README.md`, and `docs/roadmaps/generation-index.md`; queue
  coordinator edits these after merge.
- **Worker:** evidence-focused Rust/documentation worker; no provider access.
- **Excluded:** runtime or sidecar behavior; public API; dependency/version or
  range changes; feature-matrix disposition; Desktop mutation; provider call;
  credentials; Card 087; candidate, tag, release, or publication.
- **Escalation:** Chatterbox owns evidence semantics; operator owns any new
  provider, consumer-mutation, compatibility, or release authority.

## Work

1. Wait for the dependency to reach `done`. Resolve its exact Desktop PR head,
   independent review, merge, canonical closeout, capsule path and SHA-256.
   Verify the capsule from that exact merged tree; do not use a worker branch
   or unreviewed artifact.
2. Score the capsule literally against the oracle below. Recompute file and
   capsule digests. Correlate each callback, decision, tool start/result, file
   transition, turn terminal, and session identity. Distinguish absence from
   an unobserved field.
3. If every invariant passes, write Research 303 and a small deterministic
   fixture/test binding the evidence to the existing SDK default-mode
   mediation semantics. Add the guide evidence note and the historical
   g05.029 follow-up. If any invariant fails, record an honest stop without
   weakening the clause or proposing a retry.
4. Validate only the affected Claude adapter and documentation surfaces. Open
   one PR for independent exact-head review. The queue owns merge and canonical
   closeout.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Desktop is the consumer | a standalone Swallowtail proof binary drives the SDK | capsule producer and call path resolve to the merged Desktop agent service |
| One multi-turn session | two one-turn opens or a hidden reconnect | exactly one open/provider session identity and two ordered user-turn identities |
| Both turns edit | one editing turn plus one text-only turn | one successful `Edit`, `Write`, or `MultiEdit` effect in each turn and both frozen file transitions |
| Every call is mediated first | `acceptEdits`, dispatch/result/effect before allow, or one uncorrelated call | SDK `default`; for every native tool, exact Desktop allow/deny precedes dispatch, result, and filesystem effect; a preceding proposal/activity row is retained but not misclassified as execution |
| Scope stays bounded | Bash, Git, web, MCP, outside path, content repository, fallback, retry, or third turn | capsule counters/events and isolated-root digests prove zero forbidden effects |
| Evidence belongs to released source | local checkout or mismatched lock/tag | Desktop lock and capsule resolve all Swallowtail crates to `v0.4.4` commit `49c9e3b291609c9ebf5b35a284c08302f3b8d5e3` |
| Cost and lifecycle are bounded | Opus/fallback, extra attempt, or unclosed session | requested/effective admitted `claude-sonnet-5`, one session, two turns, and accepted cleanup verdict |

## Stop conditions

- Stop if the Desktop task does not merge or its exact merged capsule is
  missing, mutable, redaction-unsafe, source-mismatched, or independently
  unaccepted.
- Stop if any literal oracle row is absent or false. Do not infer ordering from
  timestamps without correlated sequence evidence and do not rerun Claude.
- Stop if acceptance would require production code, sidecar, public API,
  feature-matrix, compatibility-range, consumer, provider, tag, or release
  mutation.
- Return to Chatterbox with the exact failed row. Any repair or second live
  attempt needs fresh operator authority.

## Operator ruling — corrected ordering oracle

The second Desktop capsule exposed `ordering.tool_start_before_decision` after
the SDK streamed a tool-use proposal/activity row. Tom confirmed on 2026-09-09
that the acceptance boundary is execution, not visibility: Desktop's decision
must precede dispatch, result, and effect, while the proposed call may already
be visible. He authorized one final Desktop two-turn Sonnet session under that
oracle. Both prior capsules remain immutable, and no further retry follows.

## Evidence

On completion record the Desktop task/PR/head/review/merge/closeout, capsule
path and SHA-256, released Swallowtail identity, exact tuple and model,
session/open/turn/tool/callback counts, two file transitions, cleanup verdict,
provider-free test result, Swallowtail PR/review/merge, and every retained
non-claim.

## Next task

If accepted, the bounded g05.029 consumer-evidence gap is closed and
Chatterbox chooses the next planning direction. No provider rerun, Card 087
range work, release, or tag follows automatically.
