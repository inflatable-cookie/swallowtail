# 121 Observable Activity Common Conformance

Status: planned
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

- [ ] all lifecycle fidelity levels have reusable assertions
- [ ] unknown semantic activity cannot become empty progress
- [ ] output and final assistant activity remain correlated but distinct
- [ ] callbacks and tool activity remain independently testable
- [ ] every workspace crate compiles
- [ ] docs and public API describe one existing-stream design
- [ ] no adapter gains an unproved positive activity profile

## Validation

- `cargo test -p swallowtail-testkit`
- `effigy format:check`
- `effigy check:rust`
- `effigy lint:rust`
- `effigy qa:docs`
- `effigy package:api`

## Stop Conditions

- Stop if one adapter must fabricate semantic activity to compile.
- Stop if common conformance requires provider-specific payload parsing.

## Auto-Continuation

Continue to card 122 only after g02.035 closes and the Codex evidence lane
remains current.

