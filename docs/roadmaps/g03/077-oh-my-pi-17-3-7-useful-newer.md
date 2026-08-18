# 077 Oh My Pi 17.3.7 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-18
Depends on: Research 127; Research 134; g03.076
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 029, 032, 036-037, 039, 044, 052
Planning state: cards 242-243 completed

## Problem

Research 127 ranked AllowUnverified host drift after the Claude Code
family. Oh My Pi RPC is exact qualified `17.2.9` with AllowUnverified.
This host runs `omp/17.2.15`. Official npm `latest` is now `17.3.7`.
Raising the bound is useful-newer support, not a silent `latest` bump.

`pi.package` is a different axis. Mixing the two would flatten route
identity.

## Generation Runway Goal

Freeze Oh My Pi `17.3.7` identity against the `17.2.9` corpus, then raise
the qualified ceiling only if that evidence names a compatible extension.

## Goals

- [x] rank remaining AllowUnverified host-drift families and pick Oh My Pi
- [x] freeze npm `17.3.7` identity, host `17.2.15` help, and selected RPC
      protocol
- [x] raise the package claim through exact `17.3.7` on the existing
      behavior revision
- [x] leave Cursor Agent, Gemini, and other 127 families for later
      one-family work

## Non-Goals

- mixing `pi.package` into this milestone
- bulk-bumping other AllowUnverified families
- Gemini requalification
- provider prompts, install, update, or publication

## Execution Plan

### Batch 77.1 — Identity Corpus

- [x] Execute card 242.
- [x] record host `17.2.15` and npm `17.3.7` identity against exact
      `17.2.9`
- [x] name compatible-extension for card 243 without changing production
      claims

### Batch 77.2 — Claim And Acceptance

- [x] Execute card 243 after card 242 names compatible-extension.
- [x] raise `oh-my-pi.package` through exact `17.3.7`
- [x] keep baseline `17.2.9`, AllowUnverified, and behavior
      `oh-my-pi.rpc-v2-v17.2.9`
- [x] refresh matrices, guides, and focused Oh My Pi proof

## Acceptance Criteria

- [x] Oh My Pi is ranked first among remaining AllowUnverified
      host-drift families
- [x] `17.3.7` identity is frozen and the segment shape is named
- [x] production claim admits `17.2.9..=17.3.7` as Maintained
- [x] later stables remain visible UnverifiedNewer
- [x] `pi.package` and other 127 families remain untouched

## Decision Gates

- Stop if selected RPC evidence requires a new public operation.
- Stop if qualification depends on a provider prompt or harness install.
- Do not compile Cursor Agent or the next 127 family inside this
  milestone.

## Next Planning Checkpoint

After card 243, reassess remaining Research 127 families one at a time
and qualify useful-newer support. Next rank: Cursor Agent. Gemini stays
deferred.
