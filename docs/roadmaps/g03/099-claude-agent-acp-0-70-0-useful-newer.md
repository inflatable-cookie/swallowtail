# 099 Claude Agent ACP 0.70.0 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-19
Depends on: Research 159; Research 161; g03.098
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 015, 029, 032, 037-039, 041, 044
Planning state: cards 310-311 completed

## Problem

Research 159 ranked remaining AllowUnverified families after Codex.
Claude Agent ACP is `0.53.0..=0.69.0` excluding `0.58.0` with
AllowUnverified. This host is already qualified `0.63.0`. Official npm
`latest` is `0.70.0`. Raising the bound is useful-newer support, not a
silent `latest` bump and not a flatten onto Claude Code.

## Generation Runway Goal

Freeze Claude Agent ACP `0.63.0` / `0.70.0` identity against the `0.69.0`
corpus, then raise the qualified ceiling only if that evidence names a
compatible extension.

## Goals

- [x] rank remaining AllowUnverified registry-newer families and pick
      Claude Agent ACP
- [x] freeze host `0.63.0` and official `0.70.0` identity and selected
      ACP source
- [x] raise the adapter claim through exact `0.70.0` on existing v7
- [x] leave Providers API, goal, Air, and file-change unmapped
- [x] leave Claude Code and later 159 families for later one-family work

## Non-Goals

- flattening Claude Code headless or response-only onto this axis
- mapping advertised Providers API, goal, Air, file-change, nested
  transcript, or host-owned steering fallback
- bulk-bumping other AllowUnverified families
- Gemini requalification
- provider prompts, live ACP initialize, install, update, or publication

## Execution Plan

### Batch 99.1 — Identity Corpus

- [x] Execute card 310.
- [x] record host `0.63.0` and official `0.70.0` against exact `0.69.0`
- [x] name compatible-extension on existing v7 for card 311 without
      changing production claims

### Batch 99.2 — Claim And Acceptance

- [x] Execute card 311 after card 310 names compatible-extension.
- [x] extend Maintained `0.66.0..=0.70.0` initialize-meta-extensions-v7
- [x] keep baseline, `0.58.0` exclusion, AllowUnverified, and synthetic
      later `0.70.1`
- [x] refresh matrices, guides, contracts that name the ceiling, and
      focused Claude Agent proof

## Acceptance Criteria

- [x] host `0.63.0` remains Qualified
- [x] official `0.70.0` classifies as Qualified Maintained
- [x] `0.58.0` remains incompatible
- [x] `0.70.1` remains permitted UnverifiedNewer
- [x] decoder specimens remain the existing ACP corpora
- [x] focused Claude Agent proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Decision Gates

- Stop if exact artifact identity cannot be corroborated from official
  sources.
- Stop if selected mapped ACP protocol differs from the recorded
  evidence.
- Stop if qualifying the current official point would require a provider
  prompt or live authenticated initialize.

## Closeout

g03.099 is complete. Next Upgrade Workflow family is Claude Code
`2.1.235` (headless and response-only stay one family). Gemini stays
deferred.
