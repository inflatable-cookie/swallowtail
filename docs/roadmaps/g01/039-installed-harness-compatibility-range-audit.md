# 039 Installed Harness Compatibility Range Audit

Status: completed
Owner: Tom
Updated: 2026-07-23

## Purpose

Turn installed harness pins into maintained compatibility windows only where
current release history and frozen behavior evidence support them.

## Generation Runway

Keep g01 active. It contains 39 numbered roadmaps and remains inside the normal
30-50 roadmap range.

## Goals

- [x] Revalidate installed harness versions, release dates, protocols, and
      discovery mechanisms from official or maintained sources.
- [x] Audit Codex exec and app-server first against a realistic six-month
      client-device release span.
- [x] Separate executable version, protocol revision, behavior milestones,
      deprecations, exclusions, and adapter release.
- [x] Recommend the smallest honest baseline-through-latest range or retain an
      exact-only claim where evidence is insufficient.
- [x] Compile fixture, descriptor, probe, and implementation work only after
      the range is exact.

## Execution Plan

- [x] Installed harness compatibility evidence: card 110.
- [x] Installed-executable observation records: card 111.
- [x] Codex multi-release compatibility corpora: card 112.
- [x] Codex version discovery and range drivers: card 113.
- [x] Codex range conformance and closeout: card 114.
- [x] Codex six-month legacy-span feasibility: card 115.
- [x] Harness-configuration posture records: card 116.
- [x] Codex six-month legacy corpora: card 117.
- [x] Codex legacy version dispatch: card 118.
- [x] Codex six-month range conformance and closeout: card 119.

## Cards

- `batch-cards/110-installed-harness-compatibility-range-audit.md` — completed
- `batch-cards/111-installed-executable-observation-records.md` — completed
- `batch-cards/112-codex-multi-release-compatibility-corpora.md` — completed
- `batch-cards/113-codex-version-discovery-and-range-drivers.md` — completed
- `batch-cards/114-codex-range-conformance-and-closeout.md` — completed
- `batch-cards/115-codex-six-month-legacy-span-feasibility.md` — completed
- `batch-cards/116-harness-configuration-posture-records.md` — completed
- `batch-cards/117-codex-six-month-legacy-corpora.md` — completed
- `batch-cards/118-codex-legacy-version-dispatch.md` — completed
- `batch-cards/119-codex-six-month-range-conformance-and-closeout.md` — completed

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

The older six-month span is compiled by card 115 into separate deprecated
segments. It is not part of either first claim until cards 116-119 prove it.

Contract 032 and card 111 now realize the additive target-aware discovery
boundary. Exact host-approved executable observations carry only authoritative
host, version, claim, and compatibility evidence. General discovery behavior
is unchanged.

Card 112 freezes exact exec and app-server evidence across every selected
checkpoint. Stable and experimental app-server schemas remain distinct.
Gate-enforcing fixtures expose the existing default-false fallback field as an
ungated experimental request.

Card 113 now publishes independent closed production claims and target-aware
discovery for both drivers. App-server dispatches bounded workspace support at
the exact `0.131.0` observation. Stable sessions omit experimental fields;
experimental tools, provider requests, and workspace roots negotiate the gate
explicitly.

Card 114 proves both windows across every frozen checkpoint, rejection
neighbor, behavior milestone, and authoritative host topology without changing
either common conformance profile. Full QA has a 549-test inventory: 545 pass
and four separately gated probes remain ignored. Doctor remains at the
inherited seven errors and twelve warnings. Card 115 now owns the older
six-month feasibility boundary; it cannot widen either published claim.

Research 026 and card 115 correct the legacy app-server assumption. The
selected v2 surface exists in exact `0.80.0` tagged source. Default stdio ends
at `0.99.0`; explicit listener invocation begins at `0.100.0`. No v1 driver is
needed.

The recommended exec extension keeps ambient configuration and pre-`0.99.0`
durable retention explicit, retains current suppressed behavior from `0.122.0`,
and excludes unpublished release gaps. Cards 116-119 sequence the missing
shared configuration posture, exact corpora, private driver dispatch, and full
six-month conformance. Consumer warning and route preference remain downstream.

Contract 033 and card 116 now bind harness configuration independently through
configured instances, operation requirements, immutable plans, and runtime
request policy. Exact ambient and provider-suppressed postures pass both common
harness shapes. Direct inference, mismatches, and host-scoped execution without
a separate host lease reject before effects.

Cards 117-118 now freeze and execute the legacy segments. Exec dispatch selects
retained boolean-search, retained mode-search, ephemeral ambient, or ephemeral
suppressed behavior only from the exact immutable version binding. App-server
selects default stdio, explicit stdio, current stable, or workspace-root
behavior the same way. Unsupported policy and capability combinations reject
before process work.

Card 119 closes the six-month proof across exact boundaries, milestones,
exclusions, topology, policy, lifecycle, redaction, and unchanged common
profiles. Full repository QA inventories 563 tests: 559 pass and four gated
live or installed probes remain ignored. Roadmap 040 now owns the second
installed-harness range selection.
