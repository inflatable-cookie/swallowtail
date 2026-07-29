# 124 Codex Exec Activity And Closeout

Status: planned
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

- [ ] exec activity never claims app-server lifecycle
- [ ] selected completed items retain type and content
- [ ] final structured output remains authoritative
- [ ] unknown semantic items remain bounded and visible
- [ ] all guaranteed and unverified-newer version postures remain unchanged
- [ ] the complete Codex deterministic suite passes

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

