# 083 Usage-Evidence Closeout And Next Feature Selection

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../025-provider-feature-matrix-no-closure-programme.md`
Depends on: card 082

## Objective

Close the usage-evidence tranche, prove packaged truth, and select the next
feature family without losing the current 451-cell audit runway.

## Scope

1. Re-audit Claude Agent ACP, Pi RPC, OpenCode, and both unchanged Kimi cells.
2. Run provider-focused, workspace, docs, route, API, and package checks.
3. Record exact remaining `No` and `Not applicable` counts.
4. Update front-door counts and research disposition.
5. Select generation controls next unless evidence changes the ranking:
   output-token limit, reasoning selection, then structured output.
6. Preserve input/callback, session-continuity, provider-retention,
   retained-execution, workspace/runtime, and realtime families on the runway.

## Acceptance Criteria

- [x] usage evidence has two honest Kimi `No` cells and no false negatives
- [x] matrix and realized capabilities agree
- [x] packaged paths execute without live access
- [x] current counts are CSV-aware and machine-enforced
- [x] remaining absences retain exact evidence classifications
- [x] one next feature-family task or operator decision remains

## Stop Conditions

- a changed matrix claim lacks packaged proof
- closeout needs consumer edits or release mutation
- selecting the next family establishes product policy

## Auto-Continuation

Continue to card 084 after package-snapshot, workspace, docs, and matrix
validation pass and roadmap 026 is compiled.
