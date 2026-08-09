# 195 Nucleus Idioms Adoption Delta

Status: ready
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

- the adoption delta compiles against released Swallowtail surface
- folded delivery and recorder no-op prove on Nucleus's interactive path
- no Swallowtail contract change driven by the adoption

## Validation

- Nucleus focused tests through its consumer path
- `effigy validate:focused swallowtail-adapter-codex swallowtail-runtime`
