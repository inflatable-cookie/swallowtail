# 072 Recurring Version Currentness And Codex 0.147.0

Status: completed
Owner: Tom
Created: 2026-08-17
Depends on: Research 127; g03.071
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 029, 032, 036-037, 039, 044, 052
Planning state: cards 230-232 completed

## Problem

Installed harnesses and shared protocols move independently. Research 127
shows most production families now sit above their qualified ceiling.
Swallowtail already has UnverifiedNewer and an Upgrade Workflow, but the
all-route sweep was an ad-hoc research pass, not a named recurring process.

Raising every bound to registry `latest` would violate Contract 029.
Leaving the sweep as a one-off would let the same drift accumulate again.

Codex `0.147.0` is the first material candidate: it is npm `latest`, this
host's CLI, and already seen by Nucleus. g03.047/048 repaired live 0.147
behavior while the qualified bound stayed `0.146.0`.

## Generation Runway Goal

Make all-route currentness a named Contract 029 checkpoint, then extend
exact compatibility only where current evidence finds material drift or
useful newer support, starting with Codex `0.147.0`.

## Goals

- [x] name the recurring checkpoint in Contract 029, working rules,
      architecture, and an operator runbook
- [x] freeze Codex `0.147.0` artifact and protocol evidence against `0.146.0`
- [x] raise the qualified `codex.cli` bound only if corpus and conformance
      support it, with an explicit milestone if mapping changed
- [x] keep later stables visible unverified newer
- [x] leave Grok `1.0.x`, exact-pin families, Gemini, and other 127 rows
      for later one-family cards

## Non-Goals

- bulk-bumping every Research 127 family in this milestone
- treating Grok `1.0.x` as compatible `0.2` UnverifiedNewer
- silently adding unverified-newer to exact-pin routes
- Gemini requalification
- ZCode later gates, install/update, provider prompts, or publication
- calendar CI for registry polling

## Execution Plan

### Batch 72.1 — Named Currentness Process

- [x] Execute card 230.
- [x] record the checkpoint method, cadence, classification vocabulary, and
      one-family upgrade rule
- [x] keep claim numbers unchanged in the process card

### Batch 72.2 — Codex 0.147.0 Corpus

- [x] Execute card 231.
- [x] freeze official `0.147.0` identity against the current `0.146.0` claim
- [x] classify compatible extension versus behavior milestone using existing
      Nucleus, schema, and g03.047/048 evidence
- [x] keep the production claim at `0.146.0` until card 232

### Batch 72.3 — Codex Claim And Acceptance

- [x] Execute card 232 after card 231 names the segment shape.
- [x] extend or milestone the `codex.cli` claims through exact `0.147.0`
- [x] reconcile matrices, guides, probes, and closeout evidence
- [x] return to the currentness checkpoint for the next 127 family

## Acceptance Criteria

- [x] Contract 029 names the recurring checkpoint; the runbook matches it
- [x] the process card does not change any qualified bound
- [x] `0.147.0` is either a qualified Codex point with named behavior or an
      explicit stop with the bound left at `0.146.0`
- [x] later Codex stables remain permitted and visibly unverified
- [x] focused and extracted-package Codex proof pass if the claim moves
- [x] Grok, exact-pin, Gemini, and hosted facade bounds are untouched

## Decision Gates

- Stop if Codex `0.147.0` protocol evidence requires a new public operation
  or contract change beyond 029 currentness.
- Stop if qualification depends on a provider prompt or harness install.
- Do not compile the next 127 family inside this milestone.

## Next Planning Checkpoint

After card 232, g03.073 card 233 maps Grok `1.0.x` identity. Rank after
that: exact-pin host drift, then AllowUnverified visible newer. Gemini
stays deferred.
