# 076 Claude Code Response-Only 2.1.234 Provisional Keep

Status: completed
Owner: Tom
Created: 2026-08-18
Depends on: Research 127; Research 133; g03.068; g03.075
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 029, 032, 036-037, 039, 044, 052
Planning state: cards 239-241 completed

## Problem

g03.075 qualified Claude Code headless through `2.1.234` on a different
axis. Response-only still qualifies `2.1.227..=2.1.228` and already admits
later stables as UnverifiedNewer under g03.068's fail-closed validator.
Raising that ceiling from the headless move, or from npm `latest`, would
flatten route identity.

## Generation Runway Goal

Freeze response-only `2.1.234` identity against the `2.1.227`/`2.1.228`
protocol specimens, then either raise the qualified ceiling or keep
provisional UnverifiedNewer.

## Goals

- [x] rank remaining AllowUnverified host-drift families and pick
      response-only first
- [x] freeze selected-help protocol for `2.1.234` on this axis
- [x] keep or raise the qualified bound only as Research 133 names
      (operator-corrected: raise through `2.1.234`)
- [x] leave headless, Gemini, and other 127 families untouched

## Non-Goals

- mixing `claude-code.headless-stream-json` into this milestone
- a live response-only prompt
- bulk-bumping other AllowUnverified families
- Gemini requalification
- install, update, or publication

## Execution Plan

### Batch 76.1 — Identity Corpus

- [x] Execute card 239.
- [x] record `2.1.234` response-only identity against `2.1.227..=2.1.228`
- [x] name keep-provisional for card 240 without changing production claims

### Batch 76.2 — Keep-Bound Closeout

- [x] Execute card 240 after card 239 names keep-provisional.
- [x] operator rejected that closeout: leaving `2.1.228` as latest
      qualified skips the currentness lane

### Batch 76.3 — Claim And Acceptance

- [x] Execute card 241 after operator correction.
- [x] raise `claude-code.response-only-stream-json` through exact `2.1.234`
- [x] keep baseline `2.1.227`, AllowUnverified, empty deny-list, and
      fail-closed protocol validation
- [x] refresh matrices, guides, Contract 039 ceiling text, and focused
      proof

## Acceptance Criteria

- [x] response-only is ranked first among remaining AllowUnverified
      host-drift families
- [x] `2.1.234` identity is frozen and the segment shape is named
- [x] production response-only claim admits `2.1.227..=2.1.234` as
      Maintained
- [x] later stables remain visible UnverifiedNewer
- [x] headless `2.1.220..=2.1.234` remains a separate axis

## Decision Gates

- Stop if selected protocol evidence requires a new public operation.
- Stop if qualification depends on a provider prompt.
- Do not compile Oh My Pi or other 127 families inside this milestone.

## Next Planning Checkpoint

After card 241, reassess remaining Research 127 families one at a time and
qualify useful-newer support. Next rank: Oh My Pi. Gemini stays deferred.
