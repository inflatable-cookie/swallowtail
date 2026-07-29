# 121 Observable Activity Common Conformance

Status: completed
Owner: Tom
Created: 2026-07-29
Milestone: `../035-observable-agent-activity-kernel.md`
Depends on: card 120

## Goal

Freeze reusable activity conformance and close the shared kernel before
provider mappings begin.

## Scope

1. Add deterministic testkit fixtures for:
   - complete lifecycle
   - update-and-completion
   - completion-only
   - unavailable activity
   - callback and direct-tool correlation
   - intermediate and final assistant messages
   - reasoning summaries
   - unknown semantic activity
   - unverified-newer profile preservation
2. Add ordering, bounds, redaction, and failure assertions.
3. Migrate existing common event fixtures without changing adapter claims.
4. Compile every adapter against the new public event model.
5. Update public activity guidance and close roadmap g02.035.

## Out Of Scope

- claiming provider activity before adapter evidence
- raw payload fixtures in public records
- package candidate replacement
- consumer repository changes

## Acceptance Criteria

- [x] all lifecycle fidelity levels have reusable assertions
- [x] unknown semantic activity cannot become empty progress
- [x] output and final assistant activity remain correlated but distinct
- [x] callbacks and tool activity remain independently testable
- [x] every workspace crate compiles
- [x] docs and public API describe one existing-stream design
- [x] no adapter gains an unproved positive activity profile

## Result

- Added reusable prepared-profile plus runtime-trace fixtures for every common
  lifecycle, availability, assistant, reasoning, unknown, callback, and
  direct-tool case.
- Added one adapter-facing trace assertion over the existing `RuntimeEvent`
  stream.
- Required activity observations to remain within exact lifecycle, content,
  disclosure, correlation, unknown-event, and exchange-identity claims.
- Proved final assistant activity and terminal output as correlated task data
  on distinct events.
- Migrated the common ordered-event fixture to carry semantic activity without
  changing any provider adapter profile.
- Added public one-stream integration guidance.
- Kept all production adapters on their existing unavailable or
  not-applicable activity profiles.

## Validation

- `cargo test -p swallowtail-core` — 54 passed
- `cargo test -p swallowtail-testkit` — 67 passed
- `effigy format:check`
- `effigy check:rust` — every workspace crate and target compiled
- `effigy lint:rust`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy package:api` — 23 public-API baselines passed after the intentional
  additive testkit refresh
- `effigy doctor` — unchanged 111 findings: 83 warnings and 28 errors

## Stop Conditions

- Stop if one adapter must fabricate semantic activity to compile.
- Stop if common conformance requires provider-specific payload parsing.

## Auto-Continuation

Continue to card 122. Roadmap g02.035 is closed and the Codex evidence lane
remains the selected next proof.
