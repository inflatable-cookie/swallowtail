# 130 Headless Harness Activity Projection

Status: completed
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

- [x] tool-bearing records no longer become empty progress where qualified
- [x] completion-only sources remain explicitly thinner
- [x] provider-visible reasoning remains summary-only
- [x] structured and interactive projections remain separate roles
- [x] exact versions and unverified-newer posture remain unchanged
- [x] every child process and stream remains joined

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

## Completion Evidence

- Qwen Code enables its qualified partial-message stream and projects exact
  message, text, readable-thinking, and provider-tool lifecycle for structured
  runs and private turn-scoped continuation. Raw partial tool input and result
  bodies remain excluded.
- Gemini CLI projects assistant deltas on one operation-local identity and
  completion-only correlated tool-use and tool-result records. Parameters,
  results, and provider error bodies remain excluded.
- Claude Code preserves its qualified completion-only surface. End-turn
  assistant display, provider-tool identity, and correlated result status are
  visible without claiming partial messages or readable reasoning.
- Kimi Code projects completion-only assistant and correlated provider-tool
  records. Retry metadata and safe future records remain bounded namespaced
  activity; resume hints remain session metadata.
- Every prepared route publishes an exact observable-activity profile bound
  to its qualified behavior revision. Permitted newer versions inherit the
  last guarantee without widening it.
- Complete Qwen, Gemini, Claude Agent, and Kimi adapter suites pass. Rust
  check, lint, public-API, and formatting gates pass.
- No executable, credential, account, model request, paid inference, or
  consumer repository was used.
