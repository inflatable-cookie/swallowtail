# 161 Subagent Topology Guidance And Consumer Handoff

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../047-subagent-topology-acceptance-and-consumer-handoff.md`
Depends on: card 160

## Goal

Publish the minimal consumer projection contract and hand Nucleus an exact,
library-backed child-work adoption path.

## Scope

1. Document ordered graph projection:
   - operation root
   - child identity and parent updates
   - snapshot replacement
   - lifecycle independent from activity lifecycle
   - terminal operation handling
2. Document content, redaction, unknown parent, and unverified-newer posture.
3. Document visible provider collaboration actions as observation only.
4. Map the portable records to Nucleus child-work navigation requirements.
5. Keep storage, labels, grouping, selection, tabs, badges, and collapse state
   in Nucleus and Poodle.
6. Record exact unsupported control and inspection surfaces.
7. Close roadmap g02.047 and return to an operator checkpoint.

## Acceptance Criteria

- [x] guidance requires no raw provider payload parsing
- [x] repeated snapshots have explicit replacement semantics
- [x] unknown parent and status remain visible without invention
- [x] provider collaboration actions are not presented as consumer controls
- [x] the handoff separates Swallowtail records from Nucleus and Poodle UI state
- [x] no consumer repository is edited
- [x] docs and route truth pass
- [x] one clear next task remains

## Validation

- `effigy qa:routes`
- `effigy qa:docs`
- `git diff --check`

## Stop Conditions

- Ask the operator before editing Nucleus, Poodle, or Soundcheck.
- Do not design a product database or component hierarchy in Swallowtail.
- Do not imply retained provider history or direct child control.

## Auto-Continuation

No. Return to the g02 product checkpoint.

## Evidence

- the observable-activity guide now fixes per-operation directory lifetime,
  replacement, omission, unknown parent and status, terminal, capacity, and
  unverified-newer behavior
- the Nucleus handoff maps the public directory reducer to child navigation
  while keeping durable graph and Poodle presentation state downstream
- core and runtime pass 155 focused tests plus warnings-denied clippy against
  the final concurrent directory implementation
- the public Nucleus projection example compiles in 0.05 seconds
- route QA reports four topology-capable operations and zero direct-control
  operations
- Doctor remains warning-only at 144 findings; public-API, docs, and diff
  hygiene pass
- no Nucleus, Poodle, Soundcheck, provider, candidate, publication, tag, or
  release state changed
