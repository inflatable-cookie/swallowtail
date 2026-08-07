# 148 Total Version-Binding Helpers

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../050-provider-reachable-panic-closure.md`
Depends on: card 147

## Goal

Make version-binding helpers total so no adapter panics on observed provider
text, and lock it with a regression test.

## Scope

1. Change `ollama_runtime_binding` to return `Option` or `Result` and map the
   failure to a `VersionParse`-class diagnostic instead of panicking
   (`adapter-ollama/src/selection.rs:23`); update the provider-flow caller
   (`adapter-ollama/src/protocol/catalog.rs:70`).
2. Change `codex_cli_binding` to the same total shape
   (`adapter-codex/src/selection.rs:80`) and update its test and fixture
   callers.
3. Add a blank-version and whitespace-only regression test for Ollama
   discovery; verify every adapter with an observed version axis fails closed
   on blank provider text.
4. Align the two panicking helpers with the 13 `Option`-returning siblings.

## Out Of Scope

- version-range, claim, or classification changes
- public API additions beyond the helper shape fix
- provider, transport, or consumer behavior changes

## Acceptance

- [ ] blank and whitespace-only Ollama versions produce a version diagnostic,
      never a panic
- [ ] the Codex helper is total with updated callers
- [ ] focused adapter rounds pass

## Stop Conditions

- stop if the helper change alters qualified version classification

## Auto-Continuation

Yes, to card 149 after acceptance.

## Validation

- `effigy validate:focused swallowtail-adapter-ollama swallowtail-adapter-codex`
- `effigy check:examples`
