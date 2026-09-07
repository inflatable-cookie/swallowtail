# 137 Grok Probe Conforming ACP Client

Status: ready
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 133 merged (`0aeefa15`); the 2026-09-07 rerun capsules (`21c31e50…`, `d4da8816…`), both `inconclusive` with cause `session_new_unanswered`

## Defect

Both reruns proved more admission evidence than before —
`client_mcp_admitted`, `client_mcp_tools_listed`, and `echo_helper_live` all
true on `1.0.4` and `1.0.5` — and then failed with
`session_new_unanswered` after three and four seconds.

The probe is not a conforming ACP client. `exchange` writes its outbound
request and then drains inbound messages into the capture without answering
any of them (`grok_acp_client_mcp_probe.rs` around the `take_inbound` loop),
and `answer_callbacks`, which is the only code that replies to a client
request, runs solely after `session/prompt`. So any client request Grok
issues while establishing the session — a permission request, a filesystem
read, a terminal or capability query — is recorded and never answered. Grok
then waits, `session/new` never resolves, and the probe bounds out in a few
seconds.

That is consistent with every observed field: the echo server was spawned,
initialized, and enumerated (so Grok honoured the declaration), yet no
session id was ever returned (so no prompt could run and no `tools/call`
could occur). It is a harness gap, not a provider finding.

## Scope

1. Diagnose first, offline and at no quota cost: read the inbound frames
   already captured in the two rerun capsules and name the exact request
   method Grok sent during `session/new` that went unanswered. Record it in
   the card Result. If no inbound request is present, say so and treat the
   bound as the cause instead.
2. Answer client requests during every exchange, not only after the prompt:
   a bounded, safe responder for the methods the frames show, refusing
   anything outside a small allowlist and recording every answer in the
   capsule. No filesystem write, no shell, no network.
3. Separate the causes: `session_new_unanswered` must distinguish "an inbound
   client request went unanswered" from "no response within the bound", and
   the response bound for `session/new` must be generous enough for real
   session establishment with MCP servers (it is currently a few seconds).
4. Extend the offline fixtures to cover an agent that requires a client
   answer during `session/new`, proving the probe now completes it.
5. Update the hand-off packet.

## Out Of Scope

Adapter, claim, matrix, or contract changes; a further live attempt beyond
the single authorized rerun per segment that follows this merge;
substituting another Grok version.

## Acceptance Criteria

- [ ] the unanswered method is named from the existing capsules, or their absence recorded
- [ ] the probe answers bounded client requests during every exchange
- [ ] the two causes are distinguishable and the `session/new` bound is realistic
- [ ] offline fixtures cover the answer-during-session/new path and pass
- [ ] packet updated

## Validation

- `effigy validate:focused swallowtail-testkit`
- `effigy package:verify-affected swallowtail-testkit`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the probe fails only where the provider fails. Smallest
counterexample: a capsule that reports a provider-shaped cause for a request
the probe declined to answer.

## Result — 2026-09-08 (worker lane g05-card137, provider-free)

**Capsule inspection (Scope 1).** The two rerun capsules are referenced in
this repository only by ID prefix. Their frame payloads are Desktop-held
artifacts: they are not committed here and were not retrievable from any
Swallowtail-owned surface in this provider-free lane (repo, full git history,
hand-off packet, logs searched). No inbound request method can be named from
frames this lane could not read, so per Scope 1's fallback the bound is
treated as the cause. The defect analysis above stands on the recorded
`session_new_unanswered` cause plus the code path: `exchange` drained inbound
frames without answering, and the only answering code ran after
`session/prompt`. The responder therefore covers the standard ACP client
request surface rather than only the observed methods.

**Implemented.** `grok_acp_client_request_reply` answers inbound client
requests during every exchange: `session/request_permission` selects the
agent's own `allow_once` option; every other method is refused with a
recorded `-32601` error. No filesystem write, no shell, no network; every
answer is captured on the capsule. `answer_callbacks` is removed; the
drain-and-answer loop lives inside `exchange`.

**Causes and bound.** `session_new_unanswered` now means exactly "an inbound
client request in the `session/new` exchange was left unanswered"
(harness-shaped). The provider-shaped silence cause is new:
`session_new_bound_exceeded`. `session/new` waits up to
`LIVE_SESSION_NEW_WAIT` (60s), realistic for MCP server spawn and
enumeration; other exchanges keep the 8s bound.

**Fixtures.** Two public oracle shapes:
`SessionNewAnswerRequired` (agent completes the session only after the
probe's answer; scores `accepts_client_mcp`) and
`SessionNewSilentAfterAnswer` (probe answers are recorded, agent never
responds; scores `inconclusive`/`session_new_bound_exceeded`). A private
foreign-request fixture proves an out-of-allowlist refusal is recorded and
non-blocking. Packet updated.

**Independent review (codex/gpt-5.6-terra).** Two findings, both fixed:

1. First head: the drain loop exited after a burst carrying nothing
   answerable, so an agent that emitted a notification and then sent its
   request one drain later was never read — a provider-shaped
   `session_new_bound_exceeded` for probe silence, violating the review
   oracle. Fixed by giving each exchange one absolute deadline
   (`session/new`: 60s) that answering never extends, and draining until the
   response arrives or the bound expires.
2. Second head: the regression fixture's steps were not gated on the
   outbound `session/new`, so `SteppedPeer` served them during `initialize`
   and whole-capture response lookup misattributed the early id-3 result —
   a false proof. Fixed by activating the steps only on `session/new`,
   asserting the request frame follows the outbound `session/new`, and
   correlating both `session_id_from` and the verdict's response search to
   frames after the outbound request only.

`delayed_request_after_notification_is_still_answered` now replays the exact
review-oracle timing shape behind the gate. All selectors re-run green.

**Validation.** `effigy validate:focused swallowtail-testkit`,
`effigy package:verify-affected swallowtail-testkit`,
`effigy qa:northstar`, `scripts/check-public-api.sh` (baseline absorbed),
and `git diff --check` all pass. No Grok process, no live probe, no quota
spend. The exact one-rerun-per-segment gate, the four `evidence_pending`
matrix cells, and the no-support-claim posture are unchanged.

**Correlation completion.** The reviewer's follow-up asked that a response be
accepted only after its matching outbound request, not only for
`session/new`. `response_result` and `response_error` now locate the outbound
request by method and id and correlate only later frames; they gate
`initialize`/`authenticate` and resolve `session_id_from`, whose bespoke
search is removed for one convention. `request_was_answered` and
`permission_response_rejected` require the probe's answer to follow the
inbound request it answers. Public API, capsule shape, and verdicts for
conforming captures are unchanged.

**Hosted CI diagnosis (head `3a08560b`).** The `Stable process-spawning
nextest` job failed once on
`claude_agent_sdk_driver::registered_tool_route::close_joins_the_registered_listener`,
panicking at sidecar fixture session-open (`fixture.claude_agent_sdk.failed`,
classification `Unknown`). Unrelated flake, not a branch defect: the branch
touches no adapter code, and the identical job passed on first head
`63886503` and on `026d1d3a`, which carries every branch change plus the same
adapter sources. No card action follows; the job is green on the pushed head.

## Auto-Continuation

No. Stop for exact-head review.
