# 005 Claude Agent 0.64 Range Extension

Status: paused
Owner: Tom
Created: 2026-07-31
Milestone: `../002-claude-and-gemini-acp-range-maintenance.md`
Depends on: card 004

Paused: 2026-07-31 with the coupled Claude/Gemini tranche. Recompile Claude
independently before resuming.

## Goal

Extend the Claude Agent ACP guarantee through exact `0.64.0` while preserving
the wrapper's version-specific lifecycle and capability truth.

## Scope

1. Extend the final existing behavior through `0.62.0`.
2. Add exact `0.63.0` and `0.64.0` private behavior revisions from card 004.
3. Update discovery, protocol, lifecycle, permissions, elicitation, activity,
   prepared-facade, and cross-host fixtures.
4. Retain baseline `0.53.0`, unpublished `0.58.0` exclusion, access profiles,
   native close/delete truth, and visible unverified-newer posture.
5. Update the managed wrapper evidence package and lock to exact `0.64.0`.

## Acceptance Criteria

- [ ] the qualified upper bound is exact `0.64.0`
- [ ] `0.62.0`, `0.63.0`, and `0.64.0` dispatch through their frozen behavior
- [ ] tool, subagent, steering, and elicitation differences are not flattened
- [ ] existing form questions remain lossless and richer forms still decline
- [ ] lifecycle deletion strength and access authority remain unchanged
- [ ] baseline, exclusion, prerelease, and later-stable classifications pass
- [ ] focused Claude and shared ACP tests plus warnings-denied clippy pass

## Validation

- `effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-claude-agent`
- focused Claude compatibility, discovery, protocol, lifecycle, permission,
  elicitation, activity, and prepared-facade tests
- warnings-denied Claude adapter clippy through the focused selector
- no broad workspace suite or live Claude prompt

## Stop Conditions

- Stop if a newer wrapper changes selected authentication, session ownership,
  deletion strength, or form-response semantics beyond current contracts.
- Stop if qualification needs a fallback or public compatibility shim.
- Do not authenticate or invoke Claude.

## Auto-Continuation

Yes. Continue to card 006 only after focused Claude and ACP validation passes.
