# 130 Headless Harness Activity Projection

Status: planned
Owner: Tom
Created: 2026-07-29
Milestone: `../038-non-acp-harness-activity-coverage.md`
Depends on: card 129

## Goal

Map exact machine-readable activity from the Claude Code, Gemini CLI, Kimi
Code, and Qwen Code headless routes.

## Scope

1. Map qualified message, reasoning-summary, tool, result, search, plan, and
   terminal records.
2. Preserve completion-only fidelity when the stream supplies no stable
   start or update.
3. Publish exact route profiles for structured runs and private
   turn-scoped continuation where applicable.
4. Preserve retention, native budgets, model identity, cancellation,
   deadlines, and joined child cleanup.
5. Reject malformed, contradictory, and uncorrelatable tool records.

## Out Of Scope

- parsing terminal prose
- inventing persistent item lifecycle
- changing headless session or retention policy
- direct inference routes

## Acceptance Criteria

- [ ] tool-bearing records no longer become empty progress where qualified
- [ ] completion-only sources remain explicitly thinner
- [ ] provider-visible reasoning remains summary-only
- [ ] structured and interactive projections remain separate roles
- [ ] exact versions and unverified-newer posture remain unchanged
- [ ] every child process and stream remains joined

## Validation

- complete headless route fixture tests
- focused prepared structured and interactive suites
- `effigy check:rust`
- `effigy lint:rust`
- `effigy package:api`

## Stop Conditions

- Retain empty progress only for exact non-semantic status.
- Stop one route when machine-readable records do not support safe identity.

## Auto-Continuation

Continue to card 131 after all selected headless mappings pass.

