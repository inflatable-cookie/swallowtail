# 084 Qwen Headless 0.21.13 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-18
Depends on: Research 127; Research 141; g03.083
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 023, 029, 032-033, 037, 039, 043-044
Planning state: cards 256-257 completed

## Problem

Research 127 ranked remaining AllowUnverified families after Pi RPC.
Qwen headless is `0.19.11..=0.20.1` and `0.21.0..=0.21.2` with
AllowUnverified. This host is already qualified `0.21.2`. Official npm
`latest` is `0.21.13`. Raising the bound is useful-newer support, not a
silent `latest` bump.

## Generation Runway Goal

Freeze Qwen host `0.21.2` / official `0.21.13` identity against the
`0.21.2` corpus, then raise the qualified ceiling only if that evidence
names a compatible extension.

## Goals

- [x] rank remaining AllowUnverified registry-newer families and pick Qwen
      headless
- [x] freeze host `0.21.2` and official `0.21.13` identity and selected
      headless source
- [x] raise the adapter claim through exact `0.21.13` on the existing
      catalogue-filter revision, keeping unpublished stable `0.20.2`
      incompatible
- [x] leave Antigravity and Gemini for later one-family work

## Non-Goals

- mapping `goal_state`, initialize `effort_status`, `--continue`, fork,
  or ACP session restore
- bulk-bumping other AllowUnverified families
- Gemini requalification
- provider prompts, live catalogue, live headless sessions, install,
  update, or publication

## Execution Plan

### Batch 84.1 — Identity Corpus

- [x] Execute card 256.
- [x] record host `0.21.2` and official `0.21.13` against `0.21.2`
- [x] name compatible-extension on the existing catalogue-filter revision
      for card 257 without changing production claims

### Batch 84.2 — Claim And Acceptance

- [x] Execute card 257 after card 256 names compatible-extension.
- [x] extend Maintained `0.21.0..=0.21.13` on
      `qwen-code.headless.v0.21.0-catalogue-filter`
- [x] keep baseline, unpublished `0.20.2`, AllowUnverified, and synthetic
      later `0.21.14`
- [x] refresh matrices, guides, contracts that name the ceiling, and
      focused Qwen proof

## Acceptance Criteria

- [x] host `0.21.2` remains Qualified
- [x] official `0.21.13` classifies as Qualified Maintained
- [x] published intermediates `0.21.3` through `0.21.12` classify as
      Qualified
- [x] unpublished stable `0.20.2` remains incompatible
- [x] `0.21.14` remains permitted UnverifiedNewer
- [x] decoder specimen remains `qwen-code-v0.19.11`
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

g03.084 is complete. Next family is Antigravity (registry newer; host
already on a qualified bound). Gemini stays deferred.
