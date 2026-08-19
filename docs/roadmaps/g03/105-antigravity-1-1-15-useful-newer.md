# 105 Antigravity 1.1.15 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-19
Depends on: Research 159; Research 167; g03.104
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 023, 029, 032-033, 037, 039, 043-044
Planning state: cards 322-323 completed

## Problem

Research 159 ranked remaining AllowUnverified families after Oh My Pi.
Antigravity is qualified `1.1.9..=1.1.14` with AllowUnverified. This host
is already qualified `1.1.9`. Official GitHub `latest` is `1.1.15`.
Raising the bound is useful-newer support, not a silent `latest` bump and
not a flatten onto Gemini.

## Generation Runway Goal

Freeze Antigravity host `1.1.9` / official `1.1.15` identity against the
`1.1.14` corpus, then raise the qualified ceiling only if that evidence
names a compatible extension.

## Goals

- [x] rank remaining AllowUnverified registry-newer families and pick
      Antigravity `1.1.15`
- [x] freeze host `1.1.9` and official `1.1.15` identity, changelog, and
      selected help
- [x] raise catalogue and headless claims through `1.1.9..=1.1.15` on the
      existing behavior revisions, keeping `1.1.8` incompatible
- [x] leave Gemini requalification deferred

## Non-Goals

- mapping `--input-format` stdin NDJSON turns
- flattening onto Gemini API-key or enterprise sign-in
- live `agy models` or print prompts
- bulk-bumping other families
- install, update, or publication

## Execution Plan

### Batch 105.1 — Identity Corpus

- [x] Execute card 322.
- [x] record host `1.1.9` and official `1.1.15` against `1.1.14`
- [x] name compatible-extension on the existing catalogue and stream-json
      revisions for card 323 without changing production claims

### Batch 105.2 — Claim And Acceptance

- [x] Execute card 323 after card 322 names compatible-extension.
- [x] extend Maintained `1.1.9..=1.1.15` on both claims
- [x] keep `1.1.8` incompatible, AllowUnverified, and synthetic later
      `1.1.16`
- [x] refresh matrices, guides, architecture, and focused Antigravity
      proof

## Acceptance Criteria

- [x] host `1.1.9` remains Qualified
- [x] official `1.1.15` classifies as Qualified Maintained
- [x] `1.1.8` remains incompatible
- [x] `1.1.16` remains permitted UnverifiedNewer
- [x] decoder specimen remains `antigravity-cli-1.1.9`
- [x] focused Antigravity proof and package verify pass
- [x] matrices and guides name the new release ceiling

## Decision Gates

- Stop if exact artifact identity cannot be corroborated from official
  sources.
- Stop if selected mapped help or catalogue/headless protocol differs
  from the recorded evidence.
- Stop if qualifying the current official point would require a provider
  prompt or live session.

## Closeout

g03.105 is complete. Research 159 AllowUnverified families except deferred
Gemini now sit on current official stables. Gemini stays deferred.
