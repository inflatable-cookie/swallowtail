# 098 Codex 0.148.0 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-19
Depends on: Research 159; Research 160; g03.097
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 029, 032, 036-037, 039, 044, 048, 052
Planning state: cards 308-309 completed

## Problem

Research 159 ranked remaining AllowUnverified families after harness-route
expansion. Codex is qualified through `0.147.0` with AllowUnverified. This
host is already qualified `0.147.0`. Official npm `@openai/codex` latest is
now `0.148.0`. Raising the bound is useful-newer support, not a silent
`latest` bump.

## Generation Runway Goal

Freeze Codex host `0.147.0` / official `0.148.0` identity against the
`0.147.0` corpus, then raise the qualified ceiling only if that evidence
names a compatible extension.

## Goals

- [x] rank remaining AllowUnverified registry-newer families and pick Codex
      `0.148.0`
- [x] freeze host `0.147.0` and official `0.148.0` identity, schema, and
      selected exec/app-server surfaces
- [x] raise exec, app-server, lifecycle, and thread-catalogue claims through
      `0.148.0` on the existing behavior revisions
- [x] leave `fork` / `thread/fork` / Bedrock unmapped
- [x] leave Claude Agent and later 159 families for later one-family cards

## Non-Goals

- mapping session fork or Bedrock
- replacing the host Codex install
- live provider prompts or sessions
- bulk-bumping other families
- Gemini requalification
- install, update, or publication

## Execution Plan

### Batch 98.1 — Identity Corpus

- [x] Execute card 308.
- [x] record host `0.147.0` and official `0.148.0` against exact `0.147.0`
- [x] name compatible-extension on the existing exec, app-server, lifecycle,
      and thread-catalogue revisions for card 309 without changing
      production claims

### Batch 98.2 — Claim And Acceptance

- [x] Execute card 309 after card 308 names compatible-extension.
- [x] raise latest qualified to `0.148.0`
- [x] keep AllowUnverified, gaps, and synthetic later `0.148.1`
- [x] refresh matrices, guides, Contract 048's moving ceiling, and focused
      Codex proof

## Acceptance Criteria

- [x] host `0.147.0` remains Qualified
- [x] official `0.148.0` classifies as Qualified Maintained
- [x] `0.148.1` remains permitted UnverifiedNewer
- [x] existing gaps stay incompatible
- [x] decoder specimens stay
- [x] focused Codex proof and package verify pass
- [x] matrices and guides name the new release ceiling

## Decision Gates

- Stop if exact artifact identity cannot be corroborated from official
  sources.
- Stop if selected mapped exec flags or app-server methods differ from the
  recorded evidence.
- Stop if qualifying the current official point would require a provider
  prompt or live session.

## Closeout

g03.098 is complete. Next Upgrade Workflow family is Claude Agent ACP
`0.70.0`. Gemini stays deferred. Exact-pin drift stays closed.
`aider.headless` and `kiro.headless` stay deferred.
