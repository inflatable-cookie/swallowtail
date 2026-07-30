# 151 Critical File Decomposition

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../045-error-level-structural-health-stabilization.md`

## Goal

Remove all five critical structural findings through private source extraction
without changing behavior or public declarations.

## Scope

1. Split `anthropic/src/driver/session.rs` by preparation, turn, and cleanup.
2. Split the provider-route matrix script by lifecycle, feature, and activity
   validation.
3. Split Claude Agent prepared-facade cases by operation family.
4. Split Codex app-server cases by protocol and lifecycle concern.
5. Split OpenCode prepared cases by catalogue, session, structured-run, and
   generation-control concern.

## Acceptance Criteria

- [x] no critical structural finding remains
- [x] existing test names and fixtures remain discoverable
- [x] route-matrix output and failure codes remain unchanged
- [x] no public declaration hash changes in the decomposition scope
- [x] focused Anthropic, Claude Agent, Codex, OpenCode, and route checks pass

## Validation

- focused tests for the four adapters
- focused warnings-denied clippy for touched crates
- `bash scripts/check-provider-route-matrix.sh`
- `effigy package:api`
- `effigy doctor`
- `git diff --check`

## Stop Conditions

- Stop on concurrent functional edits to a target file.
- Stop if extraction changes test coverage, provider behavior, or script
  diagnostics.
- Do not touch high or warning-only files outside a required private seam.

## Auto-Continuation

Yes. Continue to card 152 only after all critical findings are gone.

## Evidence

- Anthropic session behavior now lives in five private source fragments; the
  public driver file is 342 code lines and its largest fragment is 384.
- The route-matrix wrapper is 149 total lines. Its existing validation logic
  is split across four private Python files and retains the same command,
  output, and failure path.
- Claude Agent, Codex, and OpenCode critical test files are split by operation
  family through source inclusion. Existing top-level test names remain
  unchanged.
- Focused tests passed: Anthropic 11, Claude Agent 13, Codex 30, and OpenCode
  44.
- Warnings-denied clippy passed for all four adapters. The route matrix passed
  with 57 operations, 27 production routes, and four auxiliary catalogues.
- Doctor now reports 147 findings: 118 warnings, 29 high errors, and no
  critical finding.
- The public-API gate reports only concurrent `swallowtail-core` and
  `swallowtail-runtime` hash drift from the typed harness-input work already in
  the working tree. All four decomposed adapter hashes match the current
  baseline.
