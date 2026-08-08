# 148 Total Version-Binding Helpers

Status: completed
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

- [x] blank and whitespace-only Ollama versions produce a version diagnostic,
      never a panic
- [x] the Codex helper is total with updated callers
- [x] focused adapter rounds pass

## Stop Conditions

- stop if the helper change alters qualified version classification

## Auto-Continuation

Yes, to card 149 after acceptance.

## Validation

- `effigy validate:focused swallowtail-adapter-ollama swallowtail-adapter-codex`
- `effigy check:examples`

## Completion Evidence

- `ollama_runtime_binding` returns `Option<InterfaceVersionBinding>` and
  rejects blank, oversized, trimmed, or control-character text; the
  provider-flow caller `parse_version` maps `None` to the new
  `swallowtail.ollama.version_parse_failed` diagnostic instead of panicking
  (`adapter-ollama/src/selection.rs`, `protocol/catalog.rs`)
- `codex_cli_binding` returns `Option<InterfaceVersionBinding>` with the
  full sibling shape (empty, length, trim, control, semver) matching the
  13 `Option`-returning adapters (`adapter-codex/src/selection.rs`)
- all Codex test and fixture callers updated through file-local `binding`
  helpers; all Ollama callers updated with fixture expects
- a crate-wide sweep confirms no remaining provider-flow
  `InterfaceVersion::new(...).expect` on non-literal input; remaining
  expects are static-constant or fixture construction
- regression tests: blank and whitespace-only Ollama versions fail with
  `version_parse_failed`; Codex blank, whitespace, non-semantic, and
  prefixed text all return `None`
- qualified classification is unchanged: valid versions bind identically,
  and well-formed out-of-window text (including the existing
  `version-malformed.json` "current" case) still reports
  `version_unsupported`
- focused rounds (202 tests, warnings-denied clippy), workspace nextest
  (1,495 passed), examples, and format all pass
