# 260 Primary Wave Source And Route Gate

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../097-harness-route-expansion-intake-and-watchlist.md`
Depends on: Research 143; g03.085

## Goal

Freeze the primary source pack and turn the external scan into candidate-specific route dispositions without changing production claims.

## Scope

Reconcile Research 143 with the current route matrix and package inventory. Record official identity, route candidates, version/package axis, access posture, topology, likely cleanup boundary, and stop conditions for Cline, Goose, Copilot CLI, Mistral Vibe, Qoder CLI, and Pi ACP.

## Out Of Scope

driver code, new packages, production descriptors, provider prompts, installation, live inference, and consumer changes

## Acceptance Criteria

- [x] Every candidate has a source-backed route disposition.
- [x] No candidate is promoted solely from ACP registry membership.
- [x] Proposed package and route identities are explicit but provisional.
- [x] Cards 261-285 have candidate-specific dependencies and stop conditions.

## Validation

`effigy qa:northstar`; read-only route, package, and source inspection.

## Stop Conditions

Stop if official evidence no longer supports a named route, if two candidates collapse onto an existing route without new pressure, or if the wire shape requires hidden credential state.

## Auto-Continuation

Continue to card 261 once the source pack and candidate order are frozen.

## Evidence

Research 143; `docs/guides/provider-route-matrix.md`; `Cargo.toml`
