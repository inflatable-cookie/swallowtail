# 102 Qwen Headless 0.21.14 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-19
Depends on: Research 159; Research 164; g03.101
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 023, 029, 032-033, 037, 039, 043-044
Planning state: cards 316-317 completed

## Problem

Research 159 ranked remaining AllowUnverified families after Grok.
Qwen headless is `0.19.11..=0.20.1` and `0.21.0..=0.21.13` with
AllowUnverified. This host is already qualified `0.21.2`. Official npm
`latest` is `0.21.14`. Raising the bound is useful-newer support, not a
silent `latest` bump.

## Generation Runway Goal

Freeze Qwen host `0.21.2` / official `0.21.14` identity against the
`0.21.13` corpus, then raise the qualified ceiling only if that evidence
names a compatible extension of
`qwen-code.headless.v0.21.0-catalogue-filter`.

## Goals

- [x] rank remaining AllowUnverified registry-newer families and pick Qwen
      `0.21.14`
- [x] freeze host `0.21.2` and official `0.21.14` identity and selected
      headless source
- [x] raise the adapter claim through exact `0.21.14` on the existing
      catalogue-filter revision, keeping unpublished stable `0.20.2`
      incompatible
- [x] leave Kimi and later 159 families for later one-family work

## Non-Goals

- mapping `goal_state`, initialize `effort_status`, `--continue`, fork,
  ACP session restore, `qwen sessions ps`, `/advisor`, or live-session
  registry
- qualifying preview `0.21.14-preview.0`
- Gemini requalification
- provider prompts, live catalogue, live headless sessions, install,
  update, or publication

## Execution Plan

### Batch 102.1 — Identity Corpus

- [x] Execute card 316.
- [x] record host `0.21.2` and official `0.21.14` against `0.21.13`
- [x] name compatible-extension on the existing catalogue-filter revision
      for card 317 without changing production claims

### Batch 102.2 — Claim And Acceptance

- [x] Execute card 317 after card 316 names compatible-extension.
- [x] extend Maintained `0.21.0..=0.21.14` on
      `qwen-code.headless.v0.21.0-catalogue-filter`
- [x] keep baseline, unpublished `0.20.2`, AllowUnverified, and synthetic
      later `0.21.15`
- [x] refresh matrices, Qwen guide, architecture, and focused Qwen proof

## Acceptance Criteria

- [x] host `0.21.2` remains Qualified
- [x] official `0.21.14` classifies as Qualified Maintained
- [x] exact `0.21.13` remains Qualified
- [x] unpublished stable `0.20.2` remains incompatible
- [x] `0.21.15` remains permitted UnverifiedNewer
- [x] decoder and `0.21.13` specimens remain
- [x] focused Qwen proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Decision Gates

- Stop if exact artifact identity cannot be corroborated from official
  sources.
- Stop if selected mapped headless protocol differs from the recorded
  evidence.
- Stop if qualifying the current official point would require a provider
  prompt or live session.

## Closeout

g03.102 is complete. Next Upgrade Workflow family is Kimi `0.37.2`.
Gemini stays deferred.
