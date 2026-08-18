# 074 Muse Code 0.2.1 Signed-Payload Pin Move

Status: completed
Owner: Tom
Created: 2026-08-18
Depends on: Research 127; Research 131; g03.073
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 029, 032, 036-037, 039, 044, 052
Planning state: cards 235-236 completed

## Problem

Research 127 ranked exact-pin host drift after Codex and Grok. This host now
runs Muse Code payload `0.2.1-R1215.1` while Swallowtail pins opaque
`0.1.0-R708.1` under QualifiedOnly. Opaque claims cannot keep two exact
segments, so support means moving the pin after corpus evidence.

## Generation Runway Goal

Freeze Muse `0.2.1-R1215.1` identity against the `0.1.0-R708.1` corpus, then
move the opaque signed-payload pin without inventing UnverifiedNewer for
opaque revisions.

## Goals

- [x] rank exact-pin host-drift families and pick Muse first
- [x] freeze payload, help, and deterministic echo identity for
      `0.2.1-R1215.1`
- [x] move the opaque pin, discovery parser, and public matrices to exact
      `0.2.1-R1215.1`
- [x] leave Command Code, DeepSeek, llama.cpp, Claude AllowUnverified, and
      Gemini for later one-family work

## Non-Goals

- keeping both opaque Muse pins
- Meta-provider requalification in the identity card
- changing Claude Code, Command Code, or other 127 families
- provider prompts, install, update, or publication

## Execution Plan

### Batch 74.1 — Identity Corpus

- [x] Execute card 235.
- [x] record host payload identity and echo protocol comparison
- [x] name opaque pin-move for card 236 without changing production claims

### Batch 74.2 — Pin Move And Acceptance

- [x] Execute card 236 after card 235 names pin-move.
- [x] replace `0.1.0-R708.1` with exact `0.2.1-R1215.1`
- [x] update discovery version parsing and payload basename checks
- [x] refresh matrices, guides, and focused Muse proof

## Acceptance Criteria

- [x] Muse is ranked first among exact-pin host-drift families
- [x] `0.2.1-R1215.1` identity is frozen and the pin-move shape is named
- [x] production Muse claim admits exact `0.2.1-R1215.1` and rejects the old
      pin
- [x] later 127 families remain untouched

## Decision Gates

- Stop if Meta authentication is required before the pin can move.
- Stop if echo protocol evidence requires a new public operation.
- Do not compile Claude or other AllowUnverified families inside this
  milestone.

## Next Planning Checkpoint

After card 236, reassess remaining Research 127 families one at a time.
Next rank: AllowUnverified cluster (including Claude Code headless useful
newer support). Gemini stays deferred.
