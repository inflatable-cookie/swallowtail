# 195 Nucleus Idioms Adoption Delta

Status: completed
Owner: Tom
Updated: 2026-08-09

## Goal

Prove the route-path opt-in on Nucleus's interactive session path without
importing product policy into Swallowtail.

## Scope

- bounded Nucleus adoption delta: one host registration and one
  session-option field on the interactive session path
- one deterministic Nucleus-side fixture for folded delivery and recorder
  wiring
- adoption notes in the Swallowtail log and Nucleus-side records

## Out Of Scope

- Nucleus product policy: what idioms mean, when signals are recorded
- the correction-loop proxy (later evidence-gated lane)
- non-interactive Nucleus surfaces

## Acceptance Criteria

- [x] the adoption delta compiles against released Swallowtail surface
      (pinned rev `1b19ccfe` carries the opt-in; tag pinning restores after
      the next source release)
- [x] folded delivery and recorder no-op prove on Nucleus's interactive path
- [x] no Swallowtail contract change driven by the adoption

## Validation

- [x] Nucleus focused `nucleus-agent-adapters` nextest: 28 passed, 2 skipped
      (includes the fold fixture)
- [x] `nucleus-agent-protocol` and `nucleus-server` compile on the pinned rev
- [x] Nucleus `effigy qa:docs` passes
- [x] production task path unchanged: opt-in stays unwired until the Nucleus
      product supplies a rules store
- [x] adoption recorded in the Nucleus log (`2026-08-09-swallowtail-idioms-route-opt-in-adoption.md`)
