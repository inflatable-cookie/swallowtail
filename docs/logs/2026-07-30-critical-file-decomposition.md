# Critical File Decomposition

Date: 2026-07-30

Card 151 removes the structural scanner's five critical findings.

## Changes

- Anthropic session behavior is split into private turn, attempt, history,
  control, and event source fragments.
- Provider-route validation is split behind the unchanged shell entry point
  into validation, base, classification, and assertion files.
- Claude Agent prepared-facade tests, Codex app-server tests, and OpenCode
  prepared-facade cases are split by operation family.
- Source inclusion preserves existing Rust test names and module scope. No
  provider behavior, fixture, route claim, or public declaration moved.

## Evidence

- focused tests: 98 passed across Anthropic, Claude Agent, Codex, and OpenCode
- focused warnings-denied clippy: passed
- provider route matrix: passed with 57 operations, 27 production routes, and
  four auxiliary catalogues
- doctor: 147 findings, 118 warnings, 29 high errors, zero critical findings

The full public-API gate reports only `swallowtail-core` and
`swallowtail-runtime` hash drift from the typed harness-input work already in
the working tree. The four decomposed adapter hashes match the current
baseline. That separate API work remains untouched.

## Next

Card 152 removes the five Codex and seven OpenCode high findings.
