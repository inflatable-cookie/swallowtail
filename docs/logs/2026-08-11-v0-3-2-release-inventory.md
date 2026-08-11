# v0.3.2 Release Inventory

Date: 2026-08-11
Roadmap: `../roadmaps/g03/067-v0-3-2-source-patch-release.md`
Card: `../roadmaps/g03/batch-cards/210-v0-3-2-release-inventory-and-gates.md`

## Result

The next selected source release is compatible patch `v0.3.2`. `[Unreleased]`
contains nine post-`v0.3.1` changes: four added, three changed, and two fixed.
Command Code and idioms are both included.

Current source has 30 packages and 36 production routes. The release order now
contains every package, places idioms before runtime, and includes Command Code,
Antigravity, Cursor, Muse, and Oh My Pi. The semantic API gate preserves the 28
immutable release baselines, records separate first-release baselines for
Command Code and idioms, and verifies five compatible current-source API
overrides without allowing removal from an immutable baseline.

The provider-wide activity fixture also exposed stale aggregate assertions.
They now state the exact 23 harness routes, 30 prepared profiles, and the
response-only route's fail-closed unknown-frame posture.

## Validation

- `effigy package:metadata` — passed for 30 packages
- `effigy package:api` — passed for 28 immutable plus two unreleased packages
- `effigy validate:focused swallowtail-testkit` — 83 tests passed
- `effigy package:verify-affected swallowtail-testkit` — passed
- `effigy release prepare --plan --version 0.3.2` — ready, mutation-free
- `effigy release simulate --version 0.3.2` with Effigy
  `v0.11.0+local.53a4971` — all 11 configured gates passed, including 1,625
  workspace tests and the isolated source consumer
- shell syntax and `git diff --check` — passed

Effigy commit `53a4971da31344c0f1f3bb24308e78ee2e85ec3c` applies parent-index
exclusions to collected links. Both formerly failing roadmap checks and the
unchanged full simulation now pass. No Swallowtail policy workaround or gate
exception was added.

## Authority

No workspace version, lockfile, release marker, candidate, commit, push, tag,
consumer, registry, or provider state changed. Card 211 may prepare a local
candidate only after explicit operator authorization.
