# 150 Provider-Reachable Expect Sweep

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../050-provider-reachable-panic-closure.md`
Depends on: card 149

## Goal

Sweep adapter `.expect()` sites that can see provider-observed input and
convert every provider-reachable one to a fail-closed result.

## Scope

1. Inventory the 1,816 `.expect()` sites across adapters; classify each as
   static-constant construction, lock-poisoning, or invariant/state.
2. Convert every invariant/state expect whose input can originate from
   provider wire data (parsing, version text, headers, envelope fields,
   notification phases) to `Result`/`Option` handling with a safe diagnostic.
3. Fix latent single-site risks found by the audit, including the Ollama
   activity profile binding expect
   (`adapter-ollama/src/activity/profile.rs:13`) and guarded-but-unclear
   expects such as `adapter-anthropic/src/driver/session/turn.rs:157`.
4. Record the remaining invariant expects as deliberate with a local comment
   stating the guard.

## Out Of Scope

- static-constant and lock-poisoning expects (no change)
- public API or diagnostic-code changes
- provider, transport, or consumer behavior changes

## Acceptance

- [ ] no adapter `expect` is reachable from provider-observed input
- [ ] invariant expects carry comments stating their guard
- [ ] focused rounds pass for every adapter touched

## Stop Conditions

- stop if a conversion changes classified failure output for a qualified
  route

## Auto-Continuation

Yes, to card 151 after acceptance.

## Validation

- focused validation per touched adapter; `effigy check:examples`
- `effigy qa:routes` after any failure-mapping touch
