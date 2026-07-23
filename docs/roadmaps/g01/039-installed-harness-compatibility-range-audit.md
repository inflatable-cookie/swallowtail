# 039 Installed Harness Compatibility Range Audit

Status: active
Owner: Tom
Updated: 2026-07-23

## Purpose

Turn installed harness pins into maintained compatibility windows only where
current release history and frozen behavior evidence support them.

## Generation Runway

Keep g01 active. It contains 39 numbered roadmaps and remains inside the normal
30-50 roadmap range.

## Goals

- [ ] Revalidate installed harness versions, release dates, protocols, and
      discovery mechanisms from official or maintained sources.
- [ ] Audit Codex exec and app-server first against a realistic six-month
      client-device release span.
- [ ] Separate executable version, protocol revision, behavior milestones,
      deprecations, exclusions, and adapter release.
- [ ] Recommend the smallest honest baseline-through-latest range or retain an
      exact-only claim where evidence is insufficient.
- [ ] Compile fixture, descriptor, probe, and implementation work only after
      the range is exact.

## Execution Plan

- [x] Installed harness compatibility evidence: card 110.
- [ ] Installed-executable observation records: card 111.
- [ ] Codex multi-release compatibility corpora: card 112.
- [ ] Codex version discovery and range drivers: card 113.
- [ ] Codex range conformance and closeout: card 114.
- [ ] Codex six-month legacy-span feasibility: card 115.

## Cards

- `batch-cards/110-installed-harness-compatibility-range-audit.md` — completed
- `batch-cards/111-installed-executable-observation-records.md` — ready
- `batch-cards/112-codex-multi-release-compatibility-corpora.md` — planned
- `batch-cards/113-codex-version-discovery-and-range-drivers.md` — planned
- `batch-cards/114-codex-range-conformance-and-closeout.md` — planned
- `batch-cards/115-codex-six-month-legacy-span-feasibility.md` — planned

## Boundaries

- no inferred range from semantic ordering or one current executable
- no compatibility shim without a proven behavior milestone
- no live authentication in default QA
- no consumer, Nucleus, or Soundcheck edit
- no provider, model, endpoint, credential, or harness fallback

## Completion Gate

The roadmap closes only after current evidence identifies exact baselines,
milestones, exclusions, and latest-qualified points for the selected installed
harness surfaces, or records why a maintained range cannot yet be published.

## Current Evidence

Research 025 selects Codex for the first retrofit and fixes separate candidate
windows:

- exec `0.122.0..=0.145.0`, pending frozen-corpus qualification
- app-server v2 `0.110.0..=0.145.0`, with a `0.131.0` behavior milestone,
  pending stable and experimental schema qualification

The older six-month span remains card 115 work. It is not part of either first
claim.
