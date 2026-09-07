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

## Auto-Continuation

No. Stop for exact-head review.
