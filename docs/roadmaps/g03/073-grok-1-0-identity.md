# 073 Grok 1.0.x Identity

Status: completed
Owner: Tom
Created: 2026-08-17
Depends on: Research 127; g03.072
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 029, 032, 036-037, 039, 044, 052
Planning state: cards 233-234 completed

## Problem

Research 127 found Grok's official line reset from `0.2.x` to `1.0.x`.
Swallowtail's qualified window is `grok-build.executable` `0.2.114..=0.2.117`
with AllowUnverified. Classifying `1.0.4` as UnverifiedNewer on the `0.2`
axis would treat a major identity reset as a compatible later stable.
Fail-closing `1.0.x` is also wrong: the route must support both windows
cleanly through Contract 029 segments.

## Generation Runway Goal

Freeze exact Grok `1.0.x` package and protocol identity, then qualify a
same-axis milestone segment so `0.2` and `1.0` installs are both honest
mappings.

## Goals

- [x] freeze npm `@xai-official/grok@1.0.4` and local `grok 1.0.4` identity
      against the `0.2.114..=0.2.117` claim
- [x] name `1.0.x` as a same-axis milestone to qualify, not UnverifiedNewer
      and not fail-closed
- [x] qualify `1.0.4` with handshake evidence and claim membership
- [x] leave exact-pin families, Gemini, and other 127 rows for later

## Non-Goals

- flattening `1.0.x` onto Grok Build `0.2` UnverifiedNewer behavior
- refusing `1.0.x` after identity is known
- inventing a second Grok axis without evidence
- Codex, exact-pin, Gemini, or hosted facade work
- provider prompts, install, update, or publication

## Execution Plan

### Batch 73.1 — Identity Corpus

- [x] Execute card 233.
- [x] record official and local `1.0.4` identity against the `0.2` claim
- [x] name the next move without changing production claims

### Batch 73.2 — Milestone Claim

- [x] Execute card 234 after card 233 names same-axis milestone support.
- [x] collect ACP handshake evidence for `1.0.4` (no provider prompt)
- [x] add a `1.0` segment; keep `0.2.114..=0.2.117`; raise latest qualified

## Acceptance Criteria

- [x] `1.0.x` is classified as a distinct identity problem, not `0.2`
      UnverifiedNewer
- [x] production Grok claims stay unchanged in this milestone's first card
- [x] `1.0.4` becomes a Qualified mapping on the same axis
- [x] later 127 families remain untouched

## Decision Gates

- Stop if identity evidence requires a new public operation or contract
  beyond 029 currentness.
- Stop if classification depends on a provider prompt or harness install.
- Do not compile the next 127 family inside card 233 or 234.

## Next Planning Checkpoint

After card 234 qualifies `1.0.4`, reassess remaining Research 127 families
one at a time. Rank after Grok: exact-pin host drift, then AllowUnverified
cluster. Gemini stays deferred.
