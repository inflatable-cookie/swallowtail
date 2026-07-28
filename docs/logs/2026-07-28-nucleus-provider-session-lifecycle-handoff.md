# 2026-07-28 Nucleus Provider-Session Lifecycle Handoff

## Changed

- Completed card 060 without editing Nucleus.
- Published one bounded non-published lifecycle handoff.
- Separated universal Nucleus-local archive, restore, and delete from optional
  bound provider operations.
- Recorded exact Codex, Claude Agent, OpenCode, and Kimi local-server actions,
  deletion truth, inactive-handle ordering, partial results, uncertainty,
  unverified-newer posture, scenario cases, dependencies, and rollback.
- Extended Codex and Claude Agent public examples with compile-checked
  management preparation and execution shapes.

## Gap

`ProviderSessionManagementBinding` has no stable persistence codec. Initial
consumer adoption can use a binding retained in process. Missing bindings and
post-restart actions must remain local-only. Durable safe export/import is now
explicit backlog work; raw provider ids cannot reconstruct authority.

## Authority

Nucleus retains confirmation, persistence, local ordering, retry, warning, UI,
and adoption decisions. No consumer source, provider, credential, registry,
candidate, tag, or release state changed.

## Validation

- all workspace examples compile
- public API baseline passes for 23 crates
- provider route, lifecycle, and feature matrices pass
- Northstar and docs checks pass
- formatting and diff checks pass
- `effigy doctor` remains red only on the existing 66 file-size findings

## Next

Roadmap g02.025 returns to the provider solution matrix. Card 080 inventories
every remaining `No` with CSV-aware counts and current evidence before choosing
the next feature family.
