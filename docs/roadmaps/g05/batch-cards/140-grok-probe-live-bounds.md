# 140 Grok Probe Live Bounds

Status: complete; PR 288
Owner: Tom
Created: 2026-09-08
Updated: 2026-09-08
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 137 merged (`56ea6008`); the 2026-09-08 decoupled rerun capsules (`51a2d094` on `1.0.4`, `02720b85` on `1.0.5`)

## Why

The card 137 fix worked where it was aimed: `session/new` now completes.
Neither rerun failed on session establishment, and both again recorded
`client_mcp_admitted`, `client_mcp_tools_listed`, and `echo_helper_live`
true — the sixth and seventh live observations that Grok Build admits a
client-declared MCP server, spawns it, connects, and enumerates its tools.

Both still ended `inconclusive`, and both causes are our bounds, not Grok:

- `1.0.5` → `no_turn_result`. The session was created and the prompt was
  sent, but every exchange other than `session/new` keeps `LIVE_WAIT`, which
  is eight seconds (`grok_acp_client_mcp_probe.rs:62`). A real model turn that
  reasons and calls a tool routinely exceeds that. We are timing out a live
  model as though it were a protocol round trip.
- `1.0.4` → `truncated`. `MAXIMUM_FRAMES` is 48
  (`grok_acp_client_mcp_probe.rs:54`). A real session that spawns an MCP
  server, enumerates tools, and runs a turn produces more frames than that, so
  capture stopped before the decisive ones.

## Scope

Do every bound in one pass. This is the third bound-shaped iteration and each
one has cost live attempts against a finite quota; the harness was sized for
protocol round trips and is being used against a live model, so re-derive all
of its limits against that reality now rather than discovering them one at a
time.

1. Give the prompt exchange its own generous bound, separate from ordinary
   protocol exchanges — a live turn that may reason and call a tool needs
   minutes, not seconds. Keep short bounds only where a response is genuinely
   immediate, and name each bound's justification in a comment.
2. Raise the frame capture bound enough for a full real session, and make
   truncation preserve the decisive frames rather than the first N: the
   `session/new` request, any tool-call and tool-result frames, and the turn
   result must survive truncation. `truncated` should mean "we dropped
   uninteresting middle frames", never "we lost the answer".
3. Audit every remaining constant in the module against a live run and adjust
   or justify each one.
4. Fixtures: an agent that answers slowly enough to exceed the old prompt
   bound and still scores `accepts_client_mcp`; a run that overflows the frame
   bound and still retains the decisive frames and scores correctly.
5. Update the hand-off packet with the new bounds and what `truncated` now
   means.

## Out Of Scope

Adapter, claim, matrix, or contract changes; another live attempt beyond the
single authorized rerun per segment that follows this merge; substituting
another Grok version.

## Acceptance Criteria

- [x] the prompt exchange bound is minutes-scale and separately justified
- [x] every bound in the module is justified against live use, in comments
- [x] truncation preserves session/new, tool-call, tool-result, and turn-result frames
- [x] fixtures cover a slow turn and an overflowing capture, both scoring correctly
- [x] packet updated

## Validation

- `effigy validate:focused swallowtail-testkit`
- `effigy package:verify-affected swallowtail-testkit`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the probe outlives a real model turn and never discards the frame
that carries the answer. Smallest counterexample: an `inconclusive` whose
cause is a limit we chose.

## Result — 2026-09-08 (worker lane g05-card140, provider-free)

**Bounds re-derived in one pass.** The prompt exchange has its own bound,
`LIVE_PROMPT_WAIT`, at 300 seconds: it is the only exchange that waits on a
live model reasoning, calling the echo tool, and finishing the turn, and a
protocol-scale bound there scores our own impatience as `no_turn_result`.
`LIVE_WAIT` is gone. `initialize` and `authenticate` — the only genuinely
immediate exchanges, with no inference behind them — take
`LIVE_PROTOCOL_WAIT` at 30 seconds, sized for a cold agent process still
loading its runtime rather than for a warm round trip. `session/new` rises to
120 seconds because establishment spawns every declared MCP server and
enumerates its tools; establishment succeeding inside the previous minute is
not evidence that a minute suffices on the slowest host. Child-exit grace
rises to 10 seconds so a Node agent flushing session state on shutdown is not
killed and the kill reported as cleanup evidence. The echo helper self-check
rises to 5 seconds, because a false negative there turns a real provider
result into `echo_liveness_unproven`. `LIVE_IDLE` stays at 200ms with its
role named: it ends one burst, and since `exchange` holds an absolute
deadline and re-enters the drain, a longer gap only splits a burst across two
reads. `STALE_CALLBACK_ID` and the three fixture strings are documented as
what they are — a sentinel that cannot collide with the four probe request
ids, and placeholders already in redacted spelling. Every constant in the
module now carries its justification in a comment.

**Truncation cannot lose the answer.** Capture capacity rises from 48 to 512
and is spent on chatter only. `FrameCapture::push` evicts the oldest
non-decisive frame to make room for an incoming one, so the outbound
`session/new` and `session/prompt` requests, every correlated response, every
inbound agent request with the probe's recorded answer, `tool_call` and
`tool_call_update` updates, and the turn result all survive overflow.
`truncated` on a capsule now means "uninteresting middle frames were elided"
and no longer decides anything: the verdict is scored normally. The
`truncated` inconclusive cause survives for the one case that can still lose
an answer — a capacity filled entirely by decisive frames — and is driven by
a separate `decisive_frame_lost` signal.

**Fixtures.** `SlowTurnPeer` releases its turn frames only when the exchange
offers at least 45 seconds and otherwise returns the empty drain a real
timeout produces; it scores `accepts_client_mcp` and spends no wall clock,
and shrinking the prompt bound back to protocol scale reproduces the
2026-09-08 `1.0.5` `no_turn_result`. The chatty fixture now streams past the
capture bound on both sides of a real echo tool call and scores
`accepts_client_mcp` while `truncated` is true, asserting each decisive frame
by name. Two `FrameCapture` unit tests pin the eviction order and the
decisive-only overflow that still names `truncated`. A bound-ordering test
pins `LIVE_PROMPT_WAIT` minutes-scale and above every protocol bound.

**Live observations (no new spend).** The 2026-09-08 decoupled reruns
recorded `client_mcp_admitted`, `client_mcp_tools_listed`, and
`echo_helper_live` true on both segments. Those are the sixth and seventh
consecutive live admission observations that Grok Build admits a
client-declared MCP server, spawns it, connects, and enumerates its tools.
Invocation remains unproven. The four `grok-build.acp` cells stay
`evidence_pending`, no claim moves, and no matrix, contract, adapter, or
release action follows. The packet carries the new bounds and the new meaning
of `truncated`.

**Independent review (codex/gpt-5.6-terra, exact head `97ecab4e`).** Three
findings, two real and both fixed:

1. The exchange deadline was not absolute. `take_inbound_within` drained
   while frames kept arriving inside `LIVE_IDLE` and never consulted the
   `remaining` budget `exchange` passed it, so an agent streaming without
   pause could outlive the 300-second prompt bound indefinitely: probe wall
   clock was unbounded, which is the same class of defect as the bounds this
   card was opened to fix. The drain is now `drain_within`, a free function
   holding its own absolute deadline and capping each idle wait at the
   remaining budget. Worst-case probe wall clock is now exactly the sum of
   the exchange bounds, about eight and a half minutes.
2. A capacity filled entirely by decisive frames still dropped the answer.
   ACP lets an agent refine one tool call many times, so 505
   `tool_call_update` frames plus the setup frames could evict nothing, and
   the terminal turn result was lost to a harness-caused `truncated` —
   directly against the review oracle. Eviction is now ordered: chatter
   first, then a `tool_call_update` that a later update for the same
   `toolCallId` has superseded, and never the opening `tool_call`, the newest
   update for a call, a correlated response, an agent request, or an outbound
   frame. `truncated` as a cause now needs hundreds of *distinct* requests
   and tool calls in one turn, and the packet says so instead of claiming no
   session shape reaches it.
3. The reviewer noted `SlowTurnPeer` derives its behaviour from the bound it
   is offered rather than from wall clock, so it could not have caught
   finding 1. Accepted as stated. It is kept for what it does pin — which
   bound reaches the prompt exchange, which is exactly what regressed — and
   its doc comment now says that plainly. The clock-shaped gap is closed by
   `drain_within_returns_inside_its_budget_under_a_continuous_stream`, which
   runs a real 20ms-cadence stream against a 100ms budget and fails on the
   pre-fix drain. The reviewer also observed that the chatty fixture's echo
   transcript is injected by the fake peer rather than earned; that is the
   module's existing offline design, where the transcript is a file the live
   helper writes, and is unchanged by this card.

The reviewer found no droppable frame class the scorers read, and confirmed
middle eviction preserves the relative ordering the indexed request/response
correlations depend on.

**Validation.** `effigy validate:focused swallowtail-testkit`,
`effigy package:verify-affected swallowtail-testkit`, `effigy qa:northstar`,
`scripts/check-public-api.sh` (one added const, `LIVE_PROMPT_WAIT`, absorbed
into the working baseline), and `git diff --check` all pass. No Grok process,
no live probe, no quota spend.

## Auto-Continuation

No. Stop for exact-head review.
