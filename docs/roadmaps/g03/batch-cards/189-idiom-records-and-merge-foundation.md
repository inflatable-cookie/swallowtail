# 189 Idiom Records And Merge Foundation

Status: completed
Owner: Tom
Updated: 2026-08-09

## Goal

Realize the `swallowtail-idioms` crate records and pure functions under
Contract 055.

## Scope

- new `swallowtail-idioms` crate depending on `swallowtail-core` only
- portable records: `IdiomId`, `IdiomScope`, `IdiomConstraint`,
  `Provenance`, `Confidence`, `IdiomSignal`
- time-based confidence decay as a pure function of stored value,
  `as-of` monotonic time, and one fixed decay rate; fixture-clock
  determinism
- merge outcomes: new, raised, lowered, unchanged (on effective confidence)
- lint bounds and malformed-record rejection

## Out Of Scope

- engine trait and backends
- registry client and transport
- prompt composition and permission authority
- learned models and Monkey

## Acceptance Criteria

- [x] records compile in the extracted package with core-only dependencies
- [x] decay determinism fixtures pass under fixture clocks
- [x] merge fixture matrix covers all four outcomes
- [x] lint rejects malformed records and out-of-range confidence

## Validation

- [x] `effigy validate:focused swallowtail-idioms` — 19 tests pass
- [x] `effigy package:verify-affected swallowtail-idioms` — extracted package
      proof passes
- [x] `cargo fmt --check` and warnings-denied clippy pass
- [x] `effigy doctor` shows no finding from the new package
