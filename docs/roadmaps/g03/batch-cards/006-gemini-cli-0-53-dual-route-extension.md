# 006 Gemini CLI 0.53 Dual-Route Extension

Status: planned
Owner: Tom
Created: 2026-07-31
Milestone: `../002-claude-and-gemini-acp-range-maintenance.md`
Depends on: card 005

## Goal

Qualify Gemini CLI `0.53.0` independently for ACP interactive sessions and
headless structured runs without borrowing one route's proof for the other.

## Scope

1. Extend the ACP claim from exact `0.51.0` through exact `0.53.0` using its
   unchanged selected behavior.
2. Extend the headless claim from `0.51.0..=0.52.0` through exact `0.53.0`.
3. Update discovery, protocol, activity, retention, management, prepared, and
   cross-host fixtures for each applicable route.
4. Make the separately gated installed probe parse and classify the observed
   point against both route claims.
5. Retain Plan Mode, read authority, durable headless transcript, deletion,
   callback, and sandbox-negative truth unchanged.

## Acceptance Criteria

- [ ] both Gemini claims end at exact `0.53.0`
- [ ] ACP and headless keep separate axes, behavior, roles, and lifecycle
- [ ] negotiated ACP options do not become a standalone catalogue
- [ ] headless retention and history-removal truth remain unchanged
- [ ] the ignored live probe reports both exact route assessments
- [ ] later stable and prerelease classifications remain distinct
- [ ] focused Gemini and shared ACP tests plus warnings-denied clippy pass

## Validation

- `effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-gemini`
- focused Gemini discovery, ACP, headless, retention, management, activity,
  prepared-facade, and cross-host tests
- compile the ignored live probe without executing it
- no broad workspace suite or live Gemini prompt

## Stop Conditions

- Stop if exact `0.53.0` artifacts differ from card 004's selected source.
- Stop if either route needs the other's transport or lifecycle claim.
- Do not authenticate, invoke Gemini, or delete a real transcript.

## Auto-Continuation

Yes. Continue to card 007 only after both route claims pass focused validation.
