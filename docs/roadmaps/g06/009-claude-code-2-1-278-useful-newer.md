# g06.009 Claude Code 2.1.278 Useful Newer

Owner: Tom
Created: 2026-09-21
Depends on: Contract 029; Research 307, 331; completed g05.058
Vision tags: route currentness, Claude Code, useful-newer

## Outcome

Qualify the Claude Code family (native headless + response-only stream-JSON)
from the current qualified ceiling `2.1.270` through official npm/GitHub
stable `2.1.278`. Identity names the segment shape first. The claim raises
both axes together.

## Why It Matters

Research 307 left both axes AllowUnverified at `2.1.270`. Official stable
moved to `2.1.278`. AllowUnverified already admits later points; it is not
permission to leave the current official stable UnverifiedNewer.

## Ready-State Rubric

- [x] Official npm `@anthropic-ai/claude-code@latest` and GitHub `v2.1.278`
      agree.
- [x] Every published hop `2.1.271` through `2.1.278` was retrieved.
- [x] Research 331 froze identity and named compatible-extension.
- [x] Claim raises both `LATEST_QUALIFIED` values to official latest.

## Decisions

- One family: headless and response-only rise together.
- Compatible-extension: keep baselines, claim ids, behavior revisions,
  AllowUnverified, historical unpublished gaps, watcher exact `2.1.251`,
  and feature-specific exact sets on `2.1.220..=2.1.241`.
- Later UnverifiedNewer is unpublished `2.1.279`.
- Host missing is not a gap. Do not install. Do not execute downloaded
  binaries. No provider prompt or live session.
- Do not rebind Claude Agent SDK native `2.1.270`.
- Do not edit Next Task, generation status, handoffs, or northstar loop
  state.

## Dispatch manifest

- **State:** ready; one family useful-newer lane.
- **Owned mutable paths:** Claude Code family fixtures, identity tests,
  selection claims, prepared guide and route/feature matrix rows that name
  this ceiling, CHANGELOG Unreleased, Research 331, this task, and the
  family logs/indexes.
- **Excluded:** other families, Gemini, SDK native pin, watcher
  authorization, Next Task, `docs/roadmaps/README.md`, generation runway,
  handoffs, lifecycle projection.

## Work

1. Freeze Research 331 identity fixtures and tests. No production claim.
2. Recheck official latest. If it moved before identity landed, extend hops
   under Contract 029 In-Run Latest Movement.
3. Apply the compatible-extension claim on both axes.
4. Run focused adapter validation and the named docs gates.

## Acceptance and review oracle

- Identity names compatible-extension with frozen hop inventory.
- Both axes qualify `2.1.220..=2.1.278` / `2.1.227..=2.1.278`.
- `2.1.279` is UnverifiedNewer. Historical gaps stay incompatible.
- Focused `validate:focused` and `package:verify-affected` for
  `swallowtail-adapter-claude-agent` pass.
- No workspace `qa`. No other family edited.

## Stop conditions

Stop and ask if a hop changes selected argv, stream-JSON shape, capability,
or authority; if channels disagree; if official latest is a major-line
reset; or if one axis must stay behind.

## Evidence

Research 331. Fixtures under
`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.278/`.
Probe artifacts lived in `/tmp/g06-009` and are not in the repo.

## External PR Review Entry Authority

Tom authorized the Queue Oracle on 2026-09-21 to prepare and take over existing
PR #349 through review entry. The external work and its evidence above are
submitted claims, not independent acceptance. Tom declared Grok 4.6 as its
external author model; canonical declaration: `xai/grok-4.6`, GitHub author
`betterthanclay`, authoring system Cursor cloud driven by Grok Bot.

Queue starts an independent reviewer, not a worker. Changes requested authorize
one ordinary revision loop in the same workspace and branch, scoped to this
Claude Code qualification and its existing changed surfaces. Queue owns merge
and lifecycle closeout. The intake preparation may publish this planning and
merge it into the existing branch; the original no-handoff/no-frontier-edit
boundary above applies to implementation workers, not this operator-approved
planning preparation or hook-owned closeout.

Canonical handoff: `docs/handoffs/20260921-g06-009-external-pr-review-entry.md`.
Preserve external branch and workspace after completion. Other external PRs are
not dispatched by this approval. Research 331 remains this lane's reference;
other PRs must reconcile their collisions before separate admission.
