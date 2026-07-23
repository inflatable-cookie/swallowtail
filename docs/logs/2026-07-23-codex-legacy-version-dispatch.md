# Codex Legacy Version Dispatch

Date: 2026-07-23
Card: `../roadmaps/g01/batch-cards/118-codex-legacy-version-dispatch.md`

## Outcome

Both production Codex drivers now execute the frozen January-to-July
compatibility segments from one immutable exact executable-version binding.
Driver identities and public operation shapes do not change.

Exec has four private behavior segments:

- `0.80.0..=0.81.0`: ambient config, durable retention, boolean search key,
  no ephemeral or suppression flags
- `0.84.0..=0.98.0`: ambient config, durable retention, mode search key, no
  ephemeral or suppression flags
- `0.99.0..=0.121.0`: ambient config, prohibited retention, ephemeral mode,
  no suppression flags
- `0.122.0..=0.145.0`: provider-suppressed config, prohibited retention, and
  the existing ephemeral suppression flags

App-server keeps one stable read-only operation subset across its deprecated
segments. `0.80.0..=0.99.0` uses default stdio where published release gaps
permit it. `0.100.0..=0.107.0` uses the explicit stdio listener. Current stable
and workspace-root behavior remains split at `0.131.0`.

Unpublished `0.82.0`, `0.83.0`, `0.108.0`, and `0.109.0` remain closed.
Prerelease, malformed, unknown, and out-of-window observations remain closed.

## Safety

Exact configured-instance posture, operation policy, retention, and selected
behavior must agree before process or materialization work. Legacy app-server
plans cannot claim dynamic tools, provider requests, or writable workspace
roots. Current exec never falls back from provider-suppressed to ambient
configuration.

No consumer, credential, endpoint, authentication, or live-provider surface
changed.

## Validation

- `effigy test --package swallowtail-adapter-codex`: 72 passed
- `effigy check:rust`: passed
- `effigy lint:rust`: passed
- `effigy doctor`: inherited 19 findings, seven errors and twelve warnings
- `git diff --check`: passed

The dispatch and range tests were split into focused modules after doctor
detected two new warning-level oversized files. The final doctor delta is zero.

## Continuation

Card 119 is ready. It owns the full closed six-month conformance proof, full
repository QA, roadmap 039 closeout, and selection of one later cross-harness
range task.
