# 004 Provider Request Reference Representation

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g03.003
Vision tags: portable identity, maintained compatibility, exact correlation
Contract refs: 014, 041, 044
Planning state: card 009 completed; g03.002 resumed at card 004

## Problem

The Codex numeric request-id repair preserves string versus signed-integer
identity privately inside its adapter, but the portable `ProviderRequestRef`
retains only visible text. A consumer receiving string `"900"` and integer
`900` cannot distinguish them without provider-native parsing.

The omission is now proven by a qualified provider boundary. Leaving it as a
private adapter detail risks weaker persisted correlation and duplicated
consumer rules.

## Contract Delta

Contracts 014, 041, and 044 now require a provider request reference to retain
the qualified scalar representation beside its opaque value. Text and signed
integer with the same visible content are distinct portable references.

Existing text-only providers remain text. No adapter infers a representation
not supplied by its qualified interface.

## Goal

Make `ProviderRequestRef` representation-aware without exposing provider
payloads or importing JSON-RPC vocabulary into the common type. Use the common
reference directly for Codex callback and activity correlation.

## Execution

- [x] Execute card 009.
- [x] add text and signed-integer portable representation metadata
- [x] retain existing opaque value and redacted formatting behavior
- [x] replace Codex's duplicate private request key with the common reference
- [x] prove equal visible forms remain distinct end to end
- [x] run focused core, runtime, and Codex validation without live effects
- [x] restore g03 card 004 as the sole next task

## Boundaries

- no arbitrary JSON value support or stringification
- no provider-native payload exposure
- no consumer persistence or wire codec
- no representation inference on existing text-only routes
- no change to callback exact-once handling
- no provider or consumer effect

## Acceptance

- [x] the common type identifies text and signed-integer representation
- [x] equal visible forms compare and hash as distinct references
- [x] default formatting remains redacted
- [x] Codex admission and resolution use the same common reference
- [x] all other adapters retain their existing text representation
- [x] focused and affected-package validation pass

## Next

After closeout, resume roadmap g03.002 at card 004. This contract completion
does not alter the Claude/Gemini tranche or generation boundary.
