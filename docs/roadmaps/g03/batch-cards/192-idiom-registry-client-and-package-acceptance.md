# 192 Idiom Registry Client And Package Acceptance

Status: completed
Owner: Tom
Updated: 2026-08-09

## Goal

Realize the registry-client merge surface without transport authority, then
close package acceptance for the lane under Contracts 036 and 055.

## Scope

- portable registry records: package references and namespaces
- pull and push merge semantics following the confidence merge outcomes
- bounded typed responses
- Contract 036 architecture/package review for `swallowtail-idioms` entry
  into the workspace release set
- guide, example, route matrix, and architecture notes; release-baseline
  handling

## Out Of Scope

- HTTP client, transport, or registry service
- learned backend and the Soundcheck correction-loop proxy (later
  checkpoint)
- version bump, tag, GitHub Release, or registry mutation

## Acceptance Criteria

- [x] registry merge fixtures pass without transport authority
- [x] Contract 036 package review passes for the new package
- [x] guide, example, matrix, and architecture stay mutually honest
- [x] focused and extracted-package validation pass

## Validation

- [x] `effigy validate:focused swallowtail-idioms swallowtail-testkit` —
      118 tests pass
- [x] `effigy package:verify-affected swallowtail-idioms swallowtail-testkit`
      — extracted package proof passes
- [x] `effigy qa:docs` passes: indexes, next-action, consumer front door, and
      integration guide coverage (idioms stays out of the pinned 34-column
      provider feature inventory, matching the debug-observation precedent)
- [x] workspace `cargo fmt --check` and warnings-denied clippy pass
- [x] example `prepared_session.rs` compiles and runs the delivery and pull
      path
- [x] release-baseline handling: `scripts/release-package-set.sh` internal
      patch set carries `swallowtail-idioms`; release topology and system
      architecture record the 30-package current source; guide registered in
      `docs/guides/README.md`; guide map and feature inventory untouched
- [x] repaired one pre-existing format drift line in
      `swallowtail-adapter-command-code` activity code left by the prior
      lane commit
