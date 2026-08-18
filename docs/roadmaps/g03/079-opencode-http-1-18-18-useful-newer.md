# 079 OpenCode HTTP 1.18.18 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-18
Depends on: Research 127; Research 136; g03.078
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 029, 036-039, 041, 044, 048-049, 052
Planning state: cards 246-247 completed

## Problem

Research 127 ranked AllowUnverified host drift after Cursor Agent. OpenCode
HTTP is published segments through `1.18.10` with AllowUnverified. This host
and official npm `latest` are `1.18.18`. Raising the bound is useful-newer
support, not a silent `latest` bump and not one closed interval.

## Generation Runway Goal

Freeze OpenCode `1.18.18` identity against the `1.14.48..=1.18.10` corpus,
then raise the qualified ceiling only if that evidence names a compatible
extension.

## Goals

- [x] rank remaining AllowUnverified host-drift families and pick OpenCode
      HTTP
- [x] freeze npm/host `1.18.18` identity and selected OpenAPI closures
- [x] raise the server claim through exact `1.18.18` with private
      `surface-19`
- [x] leave Kimi, Gemini, and other 127 families for later one-family work

## Non-Goals

- flattening unpublished gaps into one closed interval
- mixing OpenCode ACP into this HTTP/SSE claim
- bulk-bumping other AllowUnverified families
- Gemini requalification
- provider prompts, install, update, server start, or publication

## Execution Plan

### Batch 79.1 — Identity Corpus

- [x] Execute card 246.
- [x] record host and npm `1.18.18` against exact `1.18.10`
- [x] name compatible-extension plus private `surface-19` for card 247
      without changing production claims

### Batch 79.2 — Claim And Acceptance

- [x] Execute card 247 after card 246 names compatible-extension.
- [x] keep `1.18.0..=1.18.10` on `surface-18`; add `1.18.11..=1.18.18` on
      `surface-19`
- [x] keep baseline `1.14.48`, AllowUnverified, and unpublished gaps
- [x] refresh matrices, guides, contracts that name the ceiling, and
      focused OpenCode proof

## Acceptance Criteria

- [x] OpenCode is ranked first among remaining AllowUnverified host-drift
      families
- [x] `1.18.18` identity is frozen and the segment shape is named
- [x] production claim admits published segments through `1.18.18`
- [x] later stables remain visible UnverifiedNewer
- [x] other 127 families remain untouched

## Decision Gates

- Stop if selected HTTP evidence requires a new public operation.
- Stop if qualification depends on a provider prompt or starting the
  attached server.
- Do not compile Kimi or the next 127 family inside this milestone.

## Next Planning Checkpoint

After card 247, reassess remaining Research 127 families one at a time
and qualify useful-newer support. Next rank: Kimi. Gemini stays deferred.
