# 040 Cross-Harness Compatibility Range Expansion

Status: active
Owner: Tom
Updated: 2026-07-23

## Purpose

Select and prove a second installed-harness compatibility range. Test whether
the Codex version-window architecture transfers to a materially different
transport without flattening its observation, protocol, or lifecycle.

## Authority

- Research 025: Installed Harness Compatibility And Codex Range Selection
- Contract 029: Interface Version Qualification And Compatibility
- Contract 032: Installed Executable Observation And Discovery
- Contract 033: Harness Configuration Posture
- Roadmap 039: Installed Harness Compatibility Range Audit

## Generation Runway

Keep g01 active. It contains 40 numbered roadmaps and remains inside the normal
30-50 roadmap range.

## Goals

- [ ] Revalidate OpenCode, Gemini CLI ACP, Kimi Code ACP, Qwen Code, and Pi
      release and observation evidence.
- [ ] Select one second range whose transport and version seam add information
      beyond Codex.
- [ ] Reuse shared discovery and configuration records only where their
      authority matches the selected route.
- [ ] Freeze every baseline, latest boundary, behavior milestone, exclusion,
      and rejection neighbor before production changes.
- [ ] Prove the selected range without changing consumer operation shapes.

## Execution Plan

- [ ] Second installed-harness range selection: card 120.
- [ ] Selected-harness observation and compatibility corpus: card 121.
- [ ] Selected-harness private range dispatch: card 122.
- [ ] Cross-harness conformance and closeout: card 123.

## Cards

- `batch-cards/120-second-installed-harness-range-selection.md` — ready
- `batch-cards/121-selected-harness-observation-and-corpus.md` — planned
- `batch-cards/122-selected-harness-range-dispatch.md` — planned
- `batch-cards/123-cross-harness-range-conformance-and-closeout.md` — planned

## Current Evidence

OpenCode is the leading candidate. Swallowtail has an exact `1.14.48` fixture,
the local host exposes that release, upstream had reached `1.18.4` at the last
audit, and `/global/health` already exposes an exact server version. Its
attached HTTP/SSE lifecycle differs materially from Codex process stdio.

Gemini and Kimi expose executable and ACP handshake versions. Qwen exposes its
version in the first stream event. Pi has a one-point descriptor claim but no
production executable observation. Card 120 must refresh every point before
selection; this paragraph is a lead, not a range claim.

## Acceptance Criteria

- [ ] selection evidence is current, authoritative, and date-stamped
- [ ] the chosen exact-version seam is safe and host-authoritative
- [ ] the proposed range is closed, milestone-aware, and exclusion-aware
- [ ] public operation shapes and provider identities remain unchanged
- [ ] no live authentication enters default QA
- [ ] one coherent corpus and implementation tranche is ready

## Boundaries

- no range inferred from semantic ordering or a latest release
- no install, update, downgrade, route fallback, or ambient candidate search
- no provider implementation before card 120 settles the exact evidence
- no Nucleus or Soundcheck edit
- no new generation

## Planning Checkpoint

Stop after card 120 if choosing a support floor would establish consumer
product policy not already fixed by Swallowtail authority.
