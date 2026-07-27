# 065 Kimi Local Server Acceptance And Package Closeout

Status: complete
Owner: Tom
Created: 2026-07-27
Milestone: `../020-kimi-code-local-server-route.md`

## Objective

Publish the exact Kimi route distinction and prove the local-server driver from
extracted package artifacts without releasing Swallowtail.

## Governing Refs

- cards 061-064
- Contracts 011, 029, 036-038
- provider route matrix and package gates

## Scope

1. Add `kimi-code.local-server` to the exact route and lifecycle matrices.
2. Document ACP versus local-server selection, access, topology, archive,
   restore, unsupported delete, and version posture.
3. Add prepared examples for attached, owned lifecycle, imported management,
   and interactive use.
4. Execute extracted-package protocol, lifecycle, interactive, topology,
   redaction, and cleanup proofs.
5. Re-run provider-wide and Kimi ACP regressions.
6. Record Nucleus adoption inputs without editing Nucleus.
7. Do not replace the held release candidate until card 059's canonical-source
   gate is separately satisfied.

## Acceptance Criteria

- [x] every Kimi route appears once with exact driver and transport identity
- [x] ACP remains unsupported for provider-session management
- [x] local server reports archive and restore supported, delete unsupported
- [x] examples compile from extracted public artifacts
- [x] package proof contains no token, path, session id, prompt, or payload
- [x] no provider call, publication, push, tag, or consumer edit occurs

## Validation

- Kimi adapter and package-family tests
- provider route and lifecycle matrix checks
- examples, docs, public API, and repository QA
- `git diff --check`

## Stop Conditions

- package artifacts omit a required route or prepared operation
- matrices imply Kimi deletion or ACP lifecycle support
- proof requires live Kimi authentication or provider mutation

## Auto-Continuation

No. Stop for operator review before any Nucleus adoption or release-candidate
replacement.

## Evidence

- promoted `kimi-code.local-server` as the 23rd production route without
  changing `kimi-code.acp`
- published exact ACP versus local-server selection, access, topology,
  callback, archive, restore, delete, version, and cleanup truth
- added compile-tested attached, owned-lifecycle, interactive, and ACP-binding
  import examples
- added a dedicated local-server integration guide with bounded Nucleus
  adoption inputs
- extended future candidate gates to four management adapters, 23 routes, and
  the four packaged Kimi local-server suites
- kept the held release candidate unchanged

## Validation Evidence

- extracted-package assembly passed for all 23 crates
- extracted Kimi package: 21 protocol, lifecycle, binding-import, interactive,
  topology, redaction, and cleanup tests passed
- full Kimi adapter: 56 deterministic tests passed; one live installed probe
  remained gated and ignored
- full workspace tests and strict workspace Clippy passed
- all four public Kimi local-server examples compiled locally and from the
  extracted package
- route, lifecycle, docs, Northstar, formatting, Rust, and diff checks passed
- Effigy doctor retained 37 known findings, including 9 pre-existing
  oversized-file errors; card 065 added none
- no live credential, provider call, candidate replacement, publication,
  push, tag, or consumer edit occurred
