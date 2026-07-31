# 009 Qwen Code Installed Range Closure

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g03.006
Vision tags: maintained compatibility, installed harnesses, structured execution
Contract refs: 011, 020, 023, 029, 032, 037, 039, 043-045
Planning state: cards 021-023 completed

## Problem

Swallowtail guarantees only exact Qwen Code `0.19.11`. The installed executable
is `0.21.2`, so consumers see a permitted but unverified-newer route even though
current source evidence retains the selected stream protocol.

Invocation-affecting safe-mode, tool-registry, and catalogue changes occurred
inside the interval. The maintained window must classify those changes and
freeze milestones rather than promote the range from semver.

## Goal

Guarantee the useful stable Qwen Code window from baseline `0.19.11` through
installed boundary `0.21.2`, preserving exact behavior milestones and the
existing unverified-newer posture above that boundary.

## Execution

### Batch 9.1 — Exact Range Corpus

- [x] Execute card 021.
- [x] classify every stable release from `0.19.12` through `0.21.2`
- [x] freeze selected invocation, stream, catalogue, activity, and continuation
  evidence at the baseline, latest boundary, and every material milestone

### Batch 9.2 — Claim And Conformance

- [x] Execute card 022.
- [x] extend the maintained claim with explicit segments and exclusions
- [x] keep exact discovery, preflight, activity, catalogue, run, and session
  behavior aligned with each segment

### Batch 9.3 — Installed Acceptance

- [x] Execute card 023.
- [x] prove installed discovery and package extraction deterministically
- [x] run authenticated catalogue and one bounded read-only prompt only when
  local harness access is ready
- [x] reconcile route truth and close the milestone

## Boundaries

- no Qwen installation, update, or authentication mutation
- no credential or provider-config inspection
- no Qwen OAuth claim; that route is discontinued
- no implicit provider, model, endpoint, entitlement, or billing fallback
- no ambient `--continue`; restarted continuity uses an exact session id
- no permissive approval or implicit sandbox requirement
- no prerelease, nightly, or experimental package qualification
- no Claude, Gemini, consumer, or registry-publication work

## Acceptance

- [x] baseline `0.19.11` remains maintained
- [x] every stable point through `0.21.2` is classified
- [x] selected command and protocol changes map to explicit behavior revisions
- [x] installed exact `0.21.2` discovers as qualified
- [x] later stable versions remain visible and permitted as unverified newer
- [x] malformed, older, prerelease, excluded, and protocol-drift evidence fails
  safely
- [x] live access remains separately gated and cannot weaken deterministic proof
- [x] focused Qwen and extracted-package validation pass
- [x] public currentness and the sole next-task pointer are reconciled

## Next

Return to the g03 provider-maintenance checkpoint. Compare installed Pi RPC
range value with the paused standalone Claude ACP extension.
