# 011 Cursor Discovery, Catalogue, And Package

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../005-cursor-installed-dual-route-foundation.md`
Depends on: card 010

## Goal

Add the Cursor adapter foundation, identity-safe executable discovery, exact
version posture, delegated local access, and auth-aware model catalogue.

## Scope

1. Create `swallowtail-adapter-cursor` with one Cursor integration family.
2. Prefer `cursor-agent` for automatic discovery; admit an explicitly
   host-approved path only after Cursor identity and version verification.
3. Parse exact calendar-plus-build identities without inventing continuous
   ordering across opaque hashes.
4. Implement `cursor-agent.catalogue` from the frozen model-list corpus.
5. Add provider-neutral descriptors, access posture, preflight, safe
   diagnostics, host fixtures, and package metadata.

## Acceptance Criteria

- [x] Grok's generic `agent` executable cannot be selected as Cursor
- [x] host-approved paths are assessed by identity, not filename rejection
- [x] exact qualified and unverified observations remain visible
- [x] local authentication stays provider-owned and opaque
- [x] catalogue entries preserve stable ids and available metadata
- [x] catalogue discovery does not claim model invocation success
- [x] focused adapter tests and warnings-denied clippy pass

## Validation

- `effigy validate:focused swallowtail-adapter-cursor`
- focused discovery, compatibility, catalogue, preflight, diagnostic, and
  cross-host tests
- no broad workspace suite or live prompt

## Stop Conditions

- Stop if identity discovery depends on account prose or credential access.
- Stop if the model list cannot be parsed without retaining private payloads.
- Do not invoke a model or change account state.

## Auto-Continuation

Completed. Card 019 retains the active consumer-defect preemption. Continue to
card 012 after that deterministic closeout.
