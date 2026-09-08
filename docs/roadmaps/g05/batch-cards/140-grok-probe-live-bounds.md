# 140 Grok Probe Live Bounds

Status: ready
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

- [ ] the prompt exchange bound is minutes-scale and separately justified
- [ ] every bound in the module is justified against live use, in comments
- [ ] truncation preserves session/new, tool-call, tool-result, and turn-result frames
- [ ] fixtures cover a slow turn and an overflowing capture, both scoring correctly
- [ ] packet updated

## Validation

- `effigy validate:focused swallowtail-testkit`
- `effigy package:verify-affected swallowtail-testkit`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the probe outlives a real model turn and never discards the frame
that carries the answer. Smallest counterexample: an `inconclusive` whose
cause is a limit we chose.

## Auto-Continuation

No. Stop for exact-head review.
