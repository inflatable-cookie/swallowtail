# Papercuts llama.cpp context-size god-file closeout

Date: 2026-09-01
Base: `0349b90f212ad7c95037e83c3e22e8087a641fd7`
Worker: `worker/papercuts-llama-context-god-files`

## Outcome

- Closed `llama.cpp context-size proofs widen the god-file warning baseline` in
  `PAPERCUTS.md`.
- Split the remaining context-size prepared-facade proof grouping, and the one
  other still-live named target, into focused test modules. Production code,
  public API, diagnostics, and lifecycle coverage are unchanged.
- Re-measured the two paths named by the papercut: `prepared_facades.rs` (381
  code lines) and `owned_driver.rs` (260) were still findings.
  `connection_lifecycle.rs`, `prepared/attached.rs`, and `protocol.rs` were
  already outside this lane and stayed untouched.
- God-file findings fell from 383 (7 critical / 42 high / 334 warning) to 381
  (7 critical / 42 high / 332 warning).

## Structural proof

- Prepared-facade selection proofs and shared `owned_start` helpers moved into
  `tests/prepared_facades/{selections,support}.rs` via `include!`, so the seven
  tests keep their crate-root names under the existing `prepared_facades`
  target. Bodies match `HEAD` byte-for-byte.
- Remaining owned-driver startup-failure proofs moved into the existing
  `tests/owned_driver/failures.rs` module. Bodies match `HEAD`; discovery paths
  gain the `failures::` prefix already used by that module. Count stays 12.
- New and remaining files stay below the 250-code-line warning threshold:
  `prepared_facades.rs` 194, `selections.rs` 106, `support.rs` 84,
  `owned_driver.rs` 154, `failures.rs` 159.

## Historical closeout

g04.057 compilation (`a40cefd5`) already replaced the inherited-376 sentence in
the g04.056 closeout with 378 findings (332 warnings / 46 errors). That is the
then-current doctor taxonomy. Today's checker reports critical/high/warning, so
rewriting that 2026-08-24 paragraph would mix checker generations. The
historical record was left unchanged.

## Validation

- `cargo fmt -p swallowtail-adapter-llama-cpp -- --check`
- `cargo test -p swallowtail-adapter-llama-cpp --test prepared_facades --test owned_driver`
- `effigy validate:focused swallowtail-adapter-llama-cpp`
- `effigy package:verify-affected swallowtail-adapter-llama-cpp`
- `effigy --json scan god-files`
- `git diff --check`

All accepting checks passed. No provider command, live probe, install,
authentication, or broad workspace QA was run.

## Scope and next

- No public or semantic API change; no tests were weakened or rewritten.
- No roadmap, contract, architecture, research, feature-matrix, or route
  changes.
- Next open Swallowtail papercut remains the next unchecked entry in
  `PAPERCUTS.md`; this lane does not select another papercut.
