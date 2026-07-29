# 124 Codex Exec Activity And Closeout

Status: completed
Owner: Tom
Created: 2026-07-29
Milestone: `../036-codex-observable-activity-fidelity.md`
Depends on: card 123

## Goal

Map the exact completion-oriented `codex exec` activity surface and close
Codex provider-wide fidelity.

## Scope

1. Map completed assistant, reasoning-summary, search, command, file, tool,
   and other qualified JSONL items.
2. Declare completion-only fidelity where no start or update exists.
3. Preserve structured output and final output semantics.
4. Preserve non-zero discovery diagnostics, cancellation, deadlines,
   installed-version range truth, and joined process cleanup.
5. Run the complete Codex adapter, prepared facade, compatibility, and package
   regression.
6. Update the activity matrix draft and close g02.036.

## Out Of Scope

- fabricating app-server lifecycle
- parsing human terminal output
- consumer changes
- live authentication by default

## Acceptance Criteria

- [x] exec activity never claims app-server lifecycle
- [x] selected completed items retain type and content
- [x] final structured output remains authoritative
- [x] unknown semantic items remain bounded and visible
- [x] all guaranteed and unverified-newer version postures remain unchanged
- [x] the complete Codex deterministic suite passes

## Result

- Added a separate exec JSONL activity projector bound to the operation run id
  and exact qualified executable segment.
- Preserved completion-only assistant, reasoning-summary, file-change, and
  warning activity.
- Preserved started/completed command, MCP-tool, and search activity plus
  started/replacement-updated/completed todo activity.
- Added collaboration activity only from the qualified `0.92.0` milestone.
- Kept unknown semantic items bounded and namespaced. Missing identity,
  lifecycle widening, kind drift, and malformed content fail closed.
- Prepared structured exec now publishes exact streaming and observable
  activity requirements beside its immutable route profile.
- Preserved legacy search, reasoning, structured output, final output, usage,
  cancellation, deadline, discovery, compatibility, and joined-cleanup truth.
- Kept `0.146.0` executable as unverified newer on the exact `0.145.0`
  guarantee.

## Validation Evidence

- complete Codex adapter suite — 124 passed
- prepared exec profile range — `0.80.0`, `0.91.0`, `0.92.0`, `0.145.0`,
  and unverified-newer `0.146.0`
- `effigy check:rust`
- `effigy lint:rust`
- `effigy package:api`
- `effigy package:verify-local` — all 23 local archives assembled; extracted
  workspace compiled; packaged Codex prepared profile suite passed

## Validation

- focused Codex exec fixture tests
- complete `swallowtail-adapter-codex` tests
- `effigy check:rust`
- `effigy lint:rust`
- `effigy package:api`
- selected extracted-package Codex proof

## Stop Conditions

- Stop if exec output cannot distinguish semantic items from transport noise.
- Retain generic progress only for exact non-semantic status.

## Auto-Continuation

Continue to card 125 only after roadmap g02.036 closes.
