# 002 Claude And Gemini ACP Range Maintenance

Status: active
Owner: Tom
Created: 2026-07-31
Depends on: g03.001
Vision tags: maintained compatibility, installed harnesses, consumer stability
Contract refs: 011, 015, 020, 029, 032, 037-045
Planning state: card 004 ready; cards 005-007 planned

## Problem

Claude Agent `0.62.0..=0.64.0` and Gemini CLI `0.53.0` are stable releases
above Swallowtail's guaranteed bounds. They already run as visible
unverified-newer points. Current exact evidence supports fixture-first range
qualification, but the two harnesses have different behavior:

- Claude adds tool, subagent, steering, and form-elicitation milestones
- Gemini's selected ACP and headless sources are unchanged

The shared ACP wire must remain stable evidence, not a shortcut from one
harness claim to another.

## Generation Runway Goal

Extend exact compatibility only where current source and conformance support
it. Prove one milestone-bearing wrapper range beside one unchanged dual-route
extension without moving either baseline.

## Goals

- [ ] freeze exact current artifacts and behavior groups before claim changes
- [ ] qualify Claude Agent through `0.64.0` with explicit milestones
- [ ] qualify Gemini ACP and headless separately through `0.53.0`
- [ ] retain every existing baseline, exclusion, access, and lifecycle rule
- [ ] keep later stable releases visible unverified newer
- [ ] repair the stale optional OpenCode selector without reopening its range
- [ ] close through focused cross-host and extracted-package evidence

## Non-Goals

- changing stable ACP wire or schema claims
- qualifying a release from semver, release notes, or discovery alone
- moving a supported baseline
- Pi or Qwen qualification
- Pi load or resume
- installed harness updates, authentication, provider prompts, or model calls
- consumer repository edits, candidate replacement, or publication
- provider, model, endpoint, credential, or transport fallback

## Execution Plan

### Batch 2.1 — Exact Corpus And Behavior Selection

- [ ] Execute card 004.
- [ ] freeze publication, tag, dependency, source, and selected-surface records
- [ ] name exact Claude behavior groups and Gemini unchanged-source evidence
- [ ] prove stable ACP itself requires no movement

### Batch 2.2 — Claude Agent Range Extension

- [ ] Execute card 005 after card 004 passes.
- [ ] extend the wrapper claim through exact `0.64.0`
- [ ] preserve lifecycle, access, form, activity, and steering truth per segment
- [ ] update the managed local evidence package without live provider work

### Batch 2.3 — Gemini Dual-Route Extension

- [ ] Execute card 006 after Claude focused validation passes.
- [ ] extend the separate ACP and headless claims through exact `0.53.0`
- [ ] improve the ignored installed probe to classify both route axes
- [ ] retain route-specific lifecycle, retention, and activity profiles

### Batch 2.4 — Cross-Adapter Acceptance

- [ ] Execute card 007.
- [ ] repair OpenCode's stale optional live-selector assertion
- [ ] run focused ACP, Claude, Gemini, and OpenCode validation
- [ ] assemble and compile affected packages
- [ ] refresh public route, roadmap, and log truth

## Acceptance Criteria

- [ ] Claude `0.62.0..=0.64.0` membership and milestones are exact
- [ ] Gemini `0.53.0` is independently qualified on ACP and headless axes
- [ ] stable ACP v1/schema `v1.20.0` remains unchanged
- [ ] existing baselines and Claude `0.58.0` exclusion remain unchanged
- [ ] later stable points remain visible unverified newer
- [ ] OpenCode's ignored probe agrees with its claim instead of one old release
- [ ] no selected authority, access, lifecycle, retention, activity, or cleanup
  claim widens without exact evidence
- [ ] focused and affected-package evidence passes without a broad workspace
  suite or live provider prompt
- [ ] one clear next compatibility checkpoint remains

## Decision Gates

- Stop if tagged source changes a selected authority or lifecycle boundary not
  covered by current contracts.
- Stop if a claimed interval contains an uninspected stable point or gap.
- Stop if qualification needs prompt replay, fallback, hidden cleanup, or a
  common operation change.
- Do not install, update, authenticate, invoke a model, or mutate consumer or
  attached-provider state.

## Next Planning Checkpoint

After card 007, reassess Qwen and Pi as separate provider-specific tranches.
Keep Pi continuity in backlog until its cwd-binding gate changes. Consumer-
reproduced defects may preempt range maintenance at that checkpoint.
